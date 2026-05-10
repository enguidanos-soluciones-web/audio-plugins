use crate::channel::{Receiver, Sender};
use crate::clap::*;
use crate::dsp::bs2b::Bs2b;
use crate::dsp::calibration::Calibration;
use crate::dsp::itd::ItdDelay;
use crate::parameters::any::PARAMS_COUNT;
use arc_swap::ArcSwap;
use std::fmt::Debug;
use std::sync::Arc;

/// Requests sent from the GUI thread to the main thread.
#[derive(Debug)]
pub enum GuiRequest {
    /// User double-clicked a knob — main thread should reset the parameter to its default value.
    ResetParam(usize),
    /// User dragged a knob — main thread should apply the new parameter value.
    SetParam(usize, f64),
    /// User started a drag or click gesture on a parameter.
    BeginGesture(usize),
    /// User ended a drag or click gesture on a parameter.
    EndGesture(usize),
    /// User selected a built-in preset from the GUI dropdown.
    LoadPreset(&'static str),
}

#[derive(Debug)]
pub enum ParamEvent {
    Ack,
    Nack { id: usize },
    Automation { id: usize, value: f64 },
}

#[derive(Debug)]
pub enum ParamChange {
    Value { id: usize, value: f64 },
    GestureBegin { id: usize },
    GestureEnd { id: usize },
}

#[derive(Debug, Clone, Copy)]
pub struct ParamSnapshot {
    pub values: [f64; PARAMS_COUNT],
}

pub struct AudioThreadState {
    pub host: *const clap_host_t,
    pub sample_rate: f64,

    pub input_buf: Vec<f64>,
    pub output_buf: Vec<f64>,

    pub bs2b: Bs2b,
    pub itd: ItdDelay,
    pub cal: Calibration,

    pub daw_events: Sender<ParamEvent>,
    pub param_changes: Receiver<ParamChange>,
    pub param_snapshot: Arc<ArcSwap<ParamSnapshot>>,

    pub thread_id: Option<std::thread::ThreadId>,
}

impl AudioThreadState {
    pub fn reset(&mut self) {
        self.input_buf.fill(0.0);
        self.output_buf.fill(0.0);
        self.bs2b.reset();
        self.itd.reset();
        self.cal.reset();
    }

    pub fn assert_audio_thread(&self) {
        debug_assert_eq!(
            std::thread::current().id(),
            self.thread_id.expect("premature access to audio thread id"),
            "AudioThreadState accessed from wrong thread!"
        );
    }
}

pub struct MainThreadState {
    pub param_snapshot: Arc<ArcSwap<ParamSnapshot>>,

    pub daw_events: Receiver<ParamEvent>,
    pub param_changes: Sender<ParamChange>,

    pub gui_shared: Arc<ArcSwap<GUIShared>>,
    pub gui_window: Option<baseview::WindowHandle>,
    pub gui_width: u32,
    pub gui_height: u32,
    pub gui_requests: Receiver<GuiRequest>,

    pub thread_id: Option<std::thread::ThreadId>,
}

impl MainThreadState {
    pub fn assert_main_thread(&self) {
        debug_assert_eq!(
            std::thread::current().id(),
            self.thread_id.expect("premature access to main thread"),
            "MainThreadState accessed from wrong thread!"
        );
    }
}

#[derive(Debug, Default, Clone)]
pub struct GUIShared {}
