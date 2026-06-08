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

use crate::channel::{Receiver, Sender};
use crate::dsp::dc_filter::DcFilter;
use crate::dsp::lowpass_filter::LowPassFilter;
use crate::{clap::*, dsp};
use crate::{dsp::nam, parameters::any::PARAMS_COUNT};
use arc_swap::ArcSwap;
use std::fmt::Debug;
use std::sync::Arc;

/// Requests sent from the GUI thread to the main thread.
#[derive(Debug)]
pub enum GuiRequest {
    /// User clicked "Load Model" — main thread should open a file browser and load the selected NAM file.
    OpenFileBrowser,
    /// User double-clicked a knob — main thread should reset the parameter to its default value.
    ResetParam(usize),
    /// User dragged a knob — main thread should apply the new parameter value.
    SetParam(usize, f64),
    /// User started dragging a knob — notify DAW to begin automation recording.
    BeginGesture(usize),
    /// User stopped dragging a knob — notify DAW to end automation recording.
    EndGesture(usize),
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

/// Carries a fully-loaded NAM model from the main thread to the audio thread.
pub struct ModelUpdate {
    pub model: cxx::UniquePtr<nam::ffi::NamDsp>,
    pub loudness_correction: f64,
}

// SAFETY: NamDsp is used across main/audio thread boundaries throughout this codebase
// (created on main thread in activate(), used on audio thread in process()).
// The same aliasing guarantees that make the existing code safe apply here.
unsafe impl Send for ModelUpdate {}

pub struct AudioThreadState {
    pub host: *const clap_host_t,
    pub sample_rate: f64,

    pub nam_model: Option<cxx::UniquePtr<nam::ffi::NamDsp>>,
    pub nam_loudness_correction: f64,

    pub input_buf: Vec<f64>,
    pub output_buf: Vec<f64>,

    pub dc_filter: DcFilter,
    pub lowpass_filter: LowPassFilter,

    pub model_updates: Receiver<ModelUpdate>,

    pub daw_events: Sender<ParamEvent>,
    pub param_changes: Receiver<ParamChange>,
    pub param_snapshot: Arc<ArcSwap<ParamSnapshot>>,

    pub thread_id: Option<std::thread::ThreadId>,
}

impl AudioThreadState {
    pub fn reset(&mut self) {
        if let Some(nam_model) = self.nam_model.as_mut() {
            dsp::nam::ffi::reset(nam_model.pin_mut(), self.sample_rate, self.input_buf.len() as i32);
        }

        self.input_buf.fill(0.0);
        self.output_buf.fill(0.0);
        self.dc_filter.reset();
        self.lowpass_filter.reset();
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

    pub model_updates: Sender<ModelUpdate>,
    pub gui_requests: Receiver<GuiRequest>,
    pub selected_model_path: Option<String>,

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
pub struct GUIShared {
    pub nam_model_rate: Option<f64>,
    pub model_name: Option<String>,
}
