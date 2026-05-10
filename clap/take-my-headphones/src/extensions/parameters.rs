// Copyright (C) 2026 Cristian A. Enguídanos Nebot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    clap::*,
    helper::copy_cstr,
    parameters::{
        Parameter, Range, Select,
        angle::Angle,
        any::{AnyParameter, PARAMS_COUNT},
        calibration_mode::CalibrationMode,
        center::Center,
        cutoff::Cutoff,
        gain::Gain,
        lrswap::LRSwap,
        phase::Phase,
        solo::Solo,
        xfeed::XFeed,
    },
    plugin::Plugin,
    processors::{handle_clap_event::handle_clap_event, sync_main_to_audio::sync_main_to_audio},
};
use std::io::Write;
use std::{ffi::c_char, sync::Arc};

pub static PARAMETERS_EXT: clap_plugin_params_t = clap_plugin_params {
    count: Some(count),
    get_info: Some(get_info),
    get_value: Some(get_value),
    value_to_text: Some(value_to_text),
    text_to_value: Some(text_to_value),
    flush: Some(flush),
};

// [main-thread]
pub extern "C" fn count(plugin: *const clap_plugin_t) -> u32 {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *const Plugin).as_ref_unchecked() };

    let main_thread = plugin_ref.main_thread.as_ref().expect("main thread not initialized");
    main_thread.assert_main_thread();

    PARAMS_COUNT as u32
}

// [main-thread]
pub extern "C" fn get_info(plugin: *const clap_plugin_t, index: u32, information: *mut clap_param_info_t) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *const Plugin).as_ref_unchecked() };

    let main_thread = plugin_ref.main_thread.as_ref().expect("main thread not initialized");
    main_thread.assert_main_thread();

    // Parameter IDs equal their display index — no indirection needed.
    let id = index as usize;
    let Ok(param) = AnyParameter::try_from(id) else {
        return false;
    };

    let mut new_information = unsafe { std::mem::zeroed::<clap_param_info_t>() };
    new_information.id = id as u32;

    match &param {
        AnyParameter::Cutoff { inner } => {
            new_information.flags = CLAP_PARAM_IS_AUTOMATABLE as u32;
            new_information.min_value = inner.behave.min;
            new_information.max_value = inner.behave.max;
            new_information.default_value = inner.behave.def;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::XFeed { inner } => {
            new_information.flags = CLAP_PARAM_IS_AUTOMATABLE as u32;
            new_information.min_value = inner.behave.min;
            new_information.max_value = inner.behave.max;
            new_information.default_value = inner.behave.def;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::CalibrationMode { inner } => {
            new_information.flags = (CLAP_PARAM_IS_STEPPED | CLAP_PARAM_IS_ENUM) as u32;
            new_information.min_value = 0.0;
            new_information.max_value = (inner.behave.options.len() - 1) as f64;
            new_information.default_value = inner.behave.def as f64;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::Center { inner } => {
            new_information.flags = CLAP_PARAM_IS_AUTOMATABLE as u32;
            new_information.min_value = inner.behave.min;
            new_information.max_value = inner.behave.max;
            new_information.default_value = inner.behave.def;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::Angle { inner } => {
            new_information.flags = CLAP_PARAM_IS_AUTOMATABLE as u32;
            new_information.min_value = inner.behave.min;
            new_information.max_value = inner.behave.max;
            new_information.default_value = inner.behave.def;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::LRSwap { inner } => {
            new_information.flags = (CLAP_PARAM_IS_STEPPED | CLAP_PARAM_IS_ENUM) as u32;
            new_information.min_value = 0.0;
            new_information.max_value = (inner.behave.options.len() - 1) as f64;
            new_information.default_value = inner.behave.def as f64;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::Solo { inner } => {
            new_information.flags = (CLAP_PARAM_IS_STEPPED | CLAP_PARAM_IS_ENUM) as u32;
            new_information.min_value = 0.0;
            new_information.max_value = (inner.behave.options.len() - 1) as f64;
            new_information.default_value = inner.behave.def as f64;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::Phase { inner } => {
            new_information.flags = (CLAP_PARAM_IS_STEPPED | CLAP_PARAM_IS_ENUM) as u32;
            new_information.min_value = 0.0;
            new_information.max_value = (inner.behave.options.len() - 1) as f64;
            new_information.default_value = inner.behave.def as f64;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
        AnyParameter::Gain { inner } => {
            new_information.flags = CLAP_PARAM_IS_AUTOMATABLE as u32;
            new_information.min_value = inner.behave.min;
            new_information.max_value = inner.behave.max;
            new_information.default_value = inner.behave.def;
            copy_cstr(&mut new_information.name, inner.name.as_bytes());
        }
    }

    unsafe { std::ptr::write(information, new_information) };

    true
}

// [main-thread]
pub extern "C" fn get_value(plugin: *const clap_plugin_t, id: clap_id, value: *mut f64) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *const Plugin).as_ref_unchecked() };

    let main_thread = plugin_ref.main_thread.as_ref().expect("main thread not initialized");
    main_thread.assert_main_thread();

    if id as usize >= PARAMS_COUNT {
        return false;
    }

    let value_ref = unsafe { value.as_mut_unchecked() };
    let snapshot = main_thread.param_snapshot.load();
    *value_ref = snapshot.values[id as usize] as f64;

    true
}

// [main-thread]
pub extern "C" fn value_to_text(plugin: *const clap_plugin_t, id: clap_id, value: f64, display: *mut c_char, size: u32) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *const Plugin).as_ref_unchecked() };

    let main_thread = plugin_ref.main_thread.as_ref().expect("main thread not initialized");
    main_thread.assert_main_thread();

    if id as usize >= PARAMS_COUNT {
        return false;
    }

    let buffer = unsafe { std::slice::from_raw_parts_mut(display as *mut u8, size as usize) };
    let mut cursor = std::io::Cursor::new(buffer);

    match id as usize {
        Parameter::<Cutoff, Range>::ID => write!(cursor, "{:.0} Hz\0", value).is_ok(),
        Parameter::<XFeed, Range>::ID => write!(cursor, "{:.1} dB\0", value).is_ok(),
        Parameter::<Center, Range>::ID => write!(cursor, "{:.2} dB\0", value).is_ok(),
        Parameter::<Angle, Range>::ID => write!(cursor, "{:.0} deg\0", value).is_ok(),
        Parameter::<Gain, Range>::ID => write!(cursor, "{:.1} dB\0", value).is_ok(),
        Parameter::<LRSwap, Select>::ID => write!(cursor, "{}\0", LRSwap::label(value.round() as u8)).is_ok(),
        Parameter::<Solo, Select>::ID => write!(cursor, "{}\0", Solo::label(value.round() as u8)).is_ok(),
        Parameter::<Phase, Select>::ID => write!(cursor, "{}\0", Phase::label(value.round() as u8)).is_ok(),
        _ => {
            let label = CalibrationMode::label(value.round() as u8);
            write!(cursor, "{}\0", label).is_ok()
        }
    }
}

// [main-thread]
pub extern "C" fn text_to_value(plugin: *const clap_plugin_t, _param_id: clap_id, display: *const c_char, value: *mut f64) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *const Plugin).as_ref_unchecked() };

    let main_thread = plugin_ref.main_thread.as_ref().expect("main thread not initialized");
    main_thread.assert_main_thread();

    let Ok(s) = (unsafe { std::ffi::CStr::from_ptr(display) }).to_str() else {
        return false;
    };
    let s = s.trim();

    if _param_id as usize == Parameter::<CalibrationMode, Select>::ID {
        let v = match s {
            "Off" => CalibrationMode::OFF as f64,
            "Continuous" => CalibrationMode::CONTINUOUS as f64,
            "Intermittent" => CalibrationMode::INTERMITTENT as f64,
            _ => return false,
        };
        unsafe { *value = v };
        return true;
    }
    if _param_id as usize == Parameter::<LRSwap, Select>::ID {
        let v = match s {
            "On" => LRSwap::ON as f64,
            _ => LRSwap::OFF as f64,
        };
        unsafe { *value = v };
        return true;
    }
    if _param_id as usize == Parameter::<Solo, Select>::ID {
        let v = match s {
            "L" => Solo::L as f64,
            "R" => Solo::R as f64,
            _ => Solo::OFF as f64,
        };
        unsafe { *value = v };
        return true;
    }
    if _param_id as usize == Parameter::<Phase, Select>::ID {
        let v = match s {
            "L" => Phase::L as f64,
            "R" => Phase::R as f64,
            _ => Phase::OFF as f64,
        };
        unsafe { *value = v };
        return true;
    }

    // Strip unit suffix — take only the numeric part before the first space.
    let numeric = s.split_whitespace().next().unwrap_or(s);
    let Ok(parsed) = numeric.parse::<f64>() else {
        return false;
    };

    let result = parsed;

    unsafe { *value = result };

    true
}

// [active ? audio-thread : main-thread]
pub extern "C" fn flush(plugin: *const clap_plugin_t, inn: *const clap_input_events_t, out: *const clap_output_events_t) {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    let in_ref = unsafe { inn.as_ref_unchecked() };
    let event_count = in_ref.size.map(|f| unsafe { f(inn) }).unwrap_or(0);

    if let Some(audio_thread) = plugin_ref.audio_thread.as_mut() {
        // active → audio-thread
        audio_thread.assert_audio_thread();

        for n in 0..event_count {
            if let Some(get) = in_ref.get {
                let event = unsafe { get(inn, n) };
                handle_clap_event(audio_thread, event);
            }
        }
        sync_main_to_audio(audio_thread, out);
        return;
    }

    // !active → main-thread
    let main_thread = plugin_ref.main_thread.as_mut().expect("main thread not initialized");
    main_thread.assert_main_thread();

    // When !active not audio-thread is running.
    // The params events goes into the snapshot via main thread.
    for n in 0..event_count {
        if let Some(get) = in_ref.get {
            let event = unsafe { get(inn, n) };
            let event_ref = unsafe { event.as_ref_unchecked() };

            if event_ref.space_id == CLAP_CORE_EVENT_SPACE_ID && event_ref.type_ as u32 == CLAP_EVENT_PARAM_VALUE as u32 {
                let value_event = unsafe { (event as *const clap_event_param_value_t).as_ref_unchecked() };

                let id = value_event.param_id as usize;
                let value = value_event.value;

                // We can write into the snapshot from the main thread.
                let mut new_snapshot = *main_thread.param_snapshot.load_full();
                new_snapshot.values[id] = value;

                main_thread.param_snapshot.store(Arc::new(new_snapshot));
            }
        }

        // When !active there is nothing to sync into audio-thread.
        // out events are not needed.
    }
}
