// Thread and channel architecture
//
//  ┌─────────────────────────────────────────────────────────────────────────────────┐
//  │ GUI THREAD  (baseview)                                                          │
//  │  WindowHandler / Dioxus components                                              │
//  │  · on_frame  — reads  param_snapshot (ArcSwap, lock-free)                       │
//  │  · on_event  — writes gui_requests Sender<GuiRequest>                           │
//  │              — calls  host.request_callback() [thread-safe]                     │
//  └──────────────────────────────┬──────────────────────────────────────────────────┘
//                                 │ gui_requests
//                                 │ Sender<GuiRequest>  ────────────────────────────-┐
//                                 ▼                                                  │
//  ┌─────────────────────────────────────────────────────────────────────────────────┤
//  │ MAIN THREAD  (host-managed)                                                     │
//  │  on_main_thread()                                                               │
//  │  · drains gui_requests Receiver<GuiRequest>   → updates param_snapshot          │
//  │                                               → pushes  param_changes           │
//  │  · drains daw_events   Receiver<ParamEvent>                                     │
//  │      Automation  → updates param_snapshot                                       │
//  │      Ack         → (no-op)                                                      │
//  │      Nack        → requeues into param_changes                                  │
//  └──────────────────────────────┬──────────────────────────────────────────────────┘
//                                 │ param_changes
//                                 │ Sender<ParamChange>  ─────────────────────────── ┐
//                                 ▼                                                  │
//  ┌─────────────────────────────────────────────────────────────────────────────────┤
//  │ AUDIO THREAD  (host-managed, real-time)                                         │
//  │  process()                                                                      │
//  │  · sync_main_to_audio — drains param_changes Receiver<ParamChange>              │
//  │                       — emits  CLAP_EVENT_PARAM_VALUE → host out_events         │
//  │                       — pushes daw_events Sender<ParamEvent> (Ack / Nack)       │
//  │  · handle_clap_event  — reads  CLAP_EVENT_PARAM_VALUE from host in_events       │
//  │                       — pushes daw_events Sender<ParamEvent> (Automation)       │
//  │                       — calls  host.request_callback() [thread-safe]            │
//  │  · render_audio       — reads  param_snapshot (ArcSwap, lock-free)              │
//  └─────────────────────────────────────────────────────────────────────────────────┘
//
//  Shared state (Arc<ArcSwap<ParamSnapshot>>)
//  · written by main thread  in on_main_thread()
//  · read    by audio thread in render_audio()        (lock-free load)
//  · read    by GUI thread   in on_frame()            (lock-free load)

use crate::{
    channel::channel,
    clap::*,
    descriptor::PLUGIN_DESCRIPTOR,
    dsp::bs2b::Bs2bState,
    extensions::{audio_ports::AUDIO_PORTS_EXT, gui::GUI_EXT, parameters::PARAMETERS_EXT, state::STATE_EXT},
    gestures::click::ActiveClick,
    parameters::any::PARAMS_COUNT,
    plugin,
    processors::{
        handle_clap_event::handle_clap_event,
        render_audio::{render_audio_f32, render_audio_f64},
        sync_main_to_audio::sync_main_to_audio,
    },
    state::{AudioThreadState, GuiRequest, MainThreadState, ParamChange, ParamEvent, ParamSnapshot},
};
use arc_swap::ArcSwap;
use std::{
    ffi::{CStr, c_char, c_void},
    sync::Arc,
};

pub struct Plugin {
    pub inner: clap_plugin_t,
    pub host: *const clap_host_t,
    pub main_thread: Option<MainThreadState>,   // None until init()
    pub audio_thread: Option<AudioThreadState>, // None until activate()
}

pub const PLUGIN_CLASS: clap_plugin_t = clap_plugin_t {
    desc: &PLUGIN_DESCRIPTOR,
    plugin_data: std::ptr::null_mut(),
    init: Some(plugin::init),
    destroy: Some(plugin::destroy),
    activate: Some(plugin::activate),
    deactivate: Some(plugin::deactivate),
    start_processing: Some(plugin::start_processing),
    stop_processing: Some(plugin::stop_processing),
    reset: Some(plugin::reset),
    process: Some(plugin::process),
    get_extension: Some(plugin::get_extension),
    on_main_thread: Some(plugin::on_main_thread),
};

// [main-thread]
pub unsafe extern "C" fn init(plugin: *const clap_plugin_t) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    let (param_changes_tx, _) = channel::<ParamChange>(64);
    let (_, daw_events_rx) = channel::<ParamEvent>(64);

    let param_snapshot = Arc::new(ArcSwap::new(Arc::new(ParamSnapshot {
        values: [0.0; PARAMS_COUNT],
    })));

    let (_, gui_requests_rx) = channel::<GuiRequest>(128);

    plugin_ref.main_thread = Some(MainThreadState {
        param_snapshot,
        daw_events: daw_events_rx,
        param_changes: param_changes_tx,
        gui_shared: Default::default(),
        gui_window: None,
        gui_width: 800,
        gui_height: 400,
        gui_requests: gui_requests_rx,
        thread_id: Some(std::thread::current().id()),
    });

    let main_thread = plugin_ref.main_thread.as_mut().unwrap();
    let mut default_values = [0.0; PARAMS_COUNT];
    for n in 0..PARAMS_COUNT {
        let mut information = unsafe { std::mem::zeroed::<clap_param_info_t>() };
        if let Some(get_info) = PARAMETERS_EXT.get_info {
            // SAFETY: MAIN-THREAD must have FIRST THREAD_ID as SOME. OTHERWISE get_info WILL PANIC.
            if unsafe { get_info(plugin, n as u32, &mut information) } {
                default_values[n] = information.default_value;
            }
        }
    }

    main_thread.param_snapshot.store(Arc::new(ParamSnapshot { values: default_values }));

    true
}

pub unsafe extern "C" fn destroy(plugin: *const clap_plugin_t) {
    let plugin = unsafe { (*plugin).plugin_data as *mut Plugin };
    drop(unsafe { Box::from_raw(plugin) });
}

// [main-thread & !active]
pub unsafe extern "C" fn activate(plugin: *const clap_plugin, sample_rate: f64, _min_frames_count: u32, max_frames_count: u32) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    if plugin_ref.audio_thread.is_some() {
        return true;
    }

    let main_thread = plugin_ref.main_thread.as_mut().expect("main thread not initialized");
    main_thread.assert_main_thread();

    plugin_ref.audio_thread = Some(AudioThreadState {
        host: plugin_ref.host,
        sample_rate,
        bs2b: Bs2bState::new(700.0, 4.5, sample_rate),
        input_buf: vec![0.0; max_frames_count as usize],
        output_buf: vec![0.0; max_frames_count as usize],
        param_snapshot: Arc::clone(&main_thread.param_snapshot),
        param_changes: main_thread.param_changes.new_receiver(),
        daw_events: main_thread.daw_events.new_sender(),
        thread_id: None,
    });

    true
}

// [main-thread & active]
pub unsafe extern "C" fn deactivate(plugin: *const clap_plugin) {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    if plugin_ref.audio_thread.is_none() {
        return;
    }

    let main_thread = plugin_ref.main_thread.as_mut().expect("main thread not initialized");
    main_thread.assert_main_thread();

    // Drain pending events from audio before drop because we don't want to
    // loose the automatization that arrived just before 'deactivate'.
    while let Some(event) = main_thread.daw_events.pop() {
        match event {
            ParamEvent::Automation { id, value } => {
                let mut new_snapshot = *main_thread.param_snapshot.load_full();
                new_snapshot.values[id] = value;
                main_thread.param_snapshot.store(Arc::new(new_snapshot));
            }
            ParamEvent::Ack => {}
            ParamEvent::Nack { id } => {
                let value = main_thread.param_snapshot.load().values[id];
                let _ = main_thread.param_changes.push(ParamChange { id, value });
            }
        }
    }

    plugin_ref.audio_thread.take();
}

// [audio-thread & active & !processing]
pub unsafe extern "C" fn start_processing(plugin: *const clap_plugin) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    if let Some(audio_thread) = plugin_ref.audio_thread.as_mut() {
        if audio_thread.thread_id.is_none() {
            audio_thread.thread_id = Some(std::thread::current().id());
        }
    }

    true
}

// [audio-thread & active & processing]
pub unsafe extern "C" fn stop_processing(plugin: *const clap_plugin) {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    let audio_thread = plugin_ref.audio_thread.as_mut().expect("Audio Thread not initialized");
    audio_thread.assert_audio_thread();

    audio_thread.thread_id = None;
}

// [audio-thread & active]
pub unsafe extern "C" fn reset(plugin: *const clap_plugin) {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    let audio_thread = plugin_ref.audio_thread.as_mut().expect("Audio Thread not initialized");
    audio_thread.assert_audio_thread();
    audio_thread.reset();
}

// [thread-safe]
pub unsafe extern "C" fn get_extension(_plugin: *const clap_plugin, id: *const c_char) -> *const c_void {
    if unsafe { CStr::from_ptr(id) } == CLAP_EXT_AUDIO_PORTS {
        return &AUDIO_PORTS_EXT as *const _ as *const c_void;
    }
    if unsafe { CStr::from_ptr(id) } == CLAP_EXT_PARAMS {
        return &PARAMETERS_EXT as *const _ as *const c_void;
    }
    if unsafe { CStr::from_ptr(id) } == CLAP_EXT_STATE {
        return &STATE_EXT as *const _ as *const c_void;
    }
    if unsafe { CStr::from_ptr(id) } == CLAP_EXT_GUI {
        return &GUI_EXT as *const _ as *const c_void;
    }

    std::ptr::null()
}

// [main-thread]
pub unsafe extern "C" fn on_main_thread(plugin: *const clap_plugin) {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    let main = plugin_ref.main_thread.as_mut().expect("main thread not initialized");
    main.assert_main_thread();

    let mut snapshot_dirty = false;
    let mut new_snapshot = *main.param_snapshot.load_full();

    // 1. Events from audio-thread (automation + acks + nacks)
    while let Some(event) = main.daw_events.pop() {
        match event {
            ParamEvent::Automation { id, value } => {
                new_snapshot.values[id] = value;
                snapshot_dirty = true;
            }
            ParamEvent::Ack => {}
            ParamEvent::Nack { id } => {
                let value = new_snapshot.values[id];
                let _ = main.param_changes.push(ParamChange { id, value });
            }
        }
    }

    // 3. GUI requests
    while let Some(request) = main.gui_requests.pop() {
        match request {
            GuiRequest::ResetParam(id) => {
                let Some(change) = ActiveClick::from_index(id).and_then(|c| c.on_double_click()) else {
                    continue;
                };
                new_snapshot.values[change.index] = change.value;
                snapshot_dirty = true;
                let _ = main.param_changes.push(ParamChange {
                    id: change.index,
                    value: change.value,
                });
            }
            GuiRequest::SetParam(id, value) => {
                if id < new_snapshot.values.len() {
                    new_snapshot.values[id] = value;
                    snapshot_dirty = true;
                    let _ = main.param_changes.push(ParamChange { id, value });
                }
            }
        }
    }

    if snapshot_dirty {
        main.param_snapshot.store(Arc::new(new_snapshot));
    }
}

// [audio-thread & active & processing]
pub unsafe extern "C" fn process(plugin: *const clap_plugin, process: *const clap_process_t) -> clap_process_status {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };
    let process_ref = unsafe { process.as_ref_unchecked() };

    let Some(audio_thread) = plugin_ref.audio_thread.as_mut() else {
        return CLAP_PROCESS_ERROR as clap_process_status;
    };
    if audio_thread.thread_id.is_none() {
        return CLAP_PROCESS_ERROR as clap_process_status;
    }

    audio_thread.assert_audio_thread();

    sync_main_to_audio(audio_thread, process_ref.out_events.cast_mut());

    let in_events = unsafe { process_ref.in_events.as_ref_unchecked() };
    let event_count = in_events.size.map(|f| unsafe { f(process_ref.in_events) }).unwrap_or_default();

    for i in 0..event_count {
        if let Some(get) = in_events.get {
            let event = unsafe { get(process_ref.in_events, i) };
            handle_clap_event(audio_thread, event);
        }
    }

    // Some hosts (e.g. REAPER on macOS) may call process() before audio buffers
    // are fully set up, which is technically outside the CLAP spec but happens
    // in practice. Guard against null pointers to avoid a crash.
    if process_ref.audio_inputs_count == 0 || process_ref.audio_outputs_count == 0 {
        return CLAP_PROCESS_CONTINUE as clap_process_status;
    }

    let audio_inputs = unsafe { process_ref.audio_inputs.as_ref() };
    let audio_outputs = unsafe { process_ref.audio_outputs.as_mut() };

    let (Some(audio_inputs), Some(audio_outputs)) = (audio_inputs, audio_outputs) else {
        return CLAP_PROCESS_CONTINUE as clap_process_status;
    };

    let nframes = process_ref.frames_count as usize;

    if !audio_inputs.data64.is_null() && !audio_outputs.data64.is_null() {
        let in_l  = unsafe { *audio_inputs.data64.offset(0) };
        let in_r  = unsafe { *audio_inputs.data64.offset(1) };
        let out_l = unsafe { *audio_outputs.data64.offset(0) };
        let out_r = unsafe { *audio_outputs.data64.offset(1) };
        render_audio_f64(audio_thread, in_l, in_r, out_l, out_r, nframes);
        return CLAP_PROCESS_CONTINUE as clap_process_status;
    }
    if !audio_inputs.data32.is_null() && !audio_outputs.data32.is_null() {
        let in_l  = unsafe { *audio_inputs.data32.offset(0) };
        let in_r  = unsafe { *audio_inputs.data32.offset(1) };
        let out_l = unsafe { *audio_outputs.data32.offset(0) };
        let out_r = unsafe { *audio_outputs.data32.offset(1) };
        render_audio_f32(audio_thread, in_l, in_r, out_l, out_r, nframes);
        return CLAP_PROCESS_CONTINUE as clap_process_status;
    }

    CLAP_PROCESS_CONTINUE as clap_process_status
}
