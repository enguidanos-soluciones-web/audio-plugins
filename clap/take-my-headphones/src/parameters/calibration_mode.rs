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

use super::{PARAMETER_GESTURE_DOUBLE_CLICK, Parameter, ParameterClickable, ProposedParamChange, Select};

/// Calibration mode.
///
/// Off (0):          Normal bs2b processing of incoming audio.
/// Continuous (1):   Pink noise replaces input. Adjust Cutoff until stereo image
///                   sounds natural (not muddy, not artificial). Calibrates LP cutoff.
/// Intermittent (2): Alternating L/R mono pink noise (~500ms per side). Adjust XFeed
///                   until the bleed between sides feels correct for your head anatomy.
#[derive(Clone, Copy)]
pub struct CalibrationMode;

impl CalibrationMode {
    pub const OFF: u8 = 0;
    pub const CONTINUOUS: u8 = 1;
    pub const INTERMITTENT: u8 = 2;

    pub fn label(value: u8) -> &'static str {
        match value {
            Self::CONTINUOUS => "Continuous",
            Self::INTERMITTENT => "Intermittent",
            _ => "Off",
        }
    }
}

impl Parameter<CalibrationMode, Select> {
    pub const ID: usize = 8;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "CalibrationMode",
            gestures: PARAMETER_GESTURE_DOUBLE_CLICK,
            behave: Select {
                options: &[CalibrationMode::OFF, CalibrationMode::CONTINUOUS, CalibrationMode::INTERMITTENT],
                def: CalibrationMode::OFF,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn as_clickable(&self) -> Option<ParameterClickable<'_, CalibrationMode, Select>> {
        if self.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 {
            Some(ParameterClickable::<CalibrationMode, Select>::new(self))
        } else {
            None
        }
    }
}

impl<'a> ParameterClickable<'a, CalibrationMode, Select> {
    pub fn new(inner: &'a Parameter<CalibrationMode, Select>) -> Self {
        Self {
            inner,
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn on_double_click(&self) -> Option<ProposedParamChange> {
        Some(ProposedParamChange {
            index: self.inner.id,
            value: self.inner.behave.def as f64,
        })
    }
}
