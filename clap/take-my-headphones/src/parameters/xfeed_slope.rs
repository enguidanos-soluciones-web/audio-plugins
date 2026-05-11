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

use super::{
    PARAMETER_GESTURE_DOUBLE_CLICK, PARAMETER_GESTURE_DRAG, Parameter, ParameterClickable, ParameterDraggable, ProposedParamChange, Range,
};

/// Marker type for the XFeed lowpass Q (slope) parameter.
///
/// Controls the Q of the second-order biquad LP on the crossed path. Q shapes
/// the rolloff around `Cutoff`: low Q broadens the transition band (gentler
/// head shadow), high Q narrows it and introduces a resonance peak at `Cutoff`
/// (more pronounced head shadow effect).
///
/// The underlying biquad always rolls off at −12 dB/oct asymptotically. Q
/// affects only the behaviour in the transition region around `Cutoff`:
///
/// | Q     | Character                                        |
/// |-------|--------------------------------------------------|
/// | 0.5   | Critically damped — broadest rolloff, no peak    |
/// | 0.707 | Butterworth — maximally flat, no resonance       |
/// | 1.0   | Slight resonance peak at `Cutoff`                |
/// | >1.0  | Strong peak — use with care                      |
#[derive(Clone, Copy)]
pub struct XFeedSlope;

impl Parameter<XFeedSlope, Range> {
    pub const ID: usize = 9;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "Slope",
            gestures: PARAMETER_GESTURE_DRAG | PARAMETER_GESTURE_DOUBLE_CLICK,
            behave: Range {
                min: 0.1,
                max: 2.0,
                def: 0.707,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    /// Format Q for display — two decimal places (e.g. `"0.71"`).
    pub fn format_value(value: f64) -> String {
        format!("{:.2}", value)
    }

    pub fn as_draggable(&self) -> Option<ParameterDraggable<'_, XFeedSlope, Range>> {
        if self.gestures & PARAMETER_GESTURE_DRAG != 0 {
            Some(ParameterDraggable::<XFeedSlope, Range>::new(self))
        } else {
            None
        }
    }

    pub fn as_clickable(&self) -> Option<ParameterClickable<'_, XFeedSlope, Range>> {
        if self.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 {
            Some(ParameterClickable::<XFeedSlope, Range>::new(self))
        } else {
            None
        }
    }
}

impl<'a> ParameterDraggable<'a, XFeedSlope, Range> {
    pub fn new(inner: &'a Parameter<XFeedSlope, Range>) -> Self {
        Self {
            inner,
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn on_drag(&self, start_pos: (f64, f64), start_value: f64, current_pos: (f64, f64)) -> Option<ProposedParamChange> {
        const SENSITIVITY: f64 = 200.0;
        let delta = (start_pos.1 - current_pos.1) / SENSITIVITY;
        let normalized = (start_value + delta).clamp(0.0, 1.0);
        let value = self.inner.behave.min + normalized * (self.inner.behave.max - self.inner.behave.min);
        Some(ProposedParamChange {
            index: self.inner.id,
            value,
        })
    }
}

impl<'a> ParameterClickable<'a, XFeedSlope, Range> {
    pub fn new(inner: &'a Parameter<XFeedSlope, Range>) -> Self {
        Self {
            inner,
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn on_double_click(&self) -> Option<ProposedParamChange> {
        Some(ProposedParamChange {
            index: self.inner.id,
            value: self.inner.behave.def,
        })
    }
}
