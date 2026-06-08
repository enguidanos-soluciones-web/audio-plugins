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

use crate::parameters::{PARAMETER_GESTURE_DOUBLE_CLICK, ProposedParamChange, any::AnyParameter};

pub struct ActiveClick(AnyParameter);

impl ActiveClick {
    pub fn from_index(index: usize) -> Option<Self> {
        let param = AnyParameter::try_from(index).ok()?;

        let gestures = match &param {
            AnyParameter::InputGain { inner } => inner.gestures,
            AnyParameter::OutputGain { inner } => inner.gestures,
            AnyParameter::Tone { inner } => inner.gestures,
            AnyParameter::Blend { inner } => inner.gestures,
        };

        if gestures & PARAMETER_GESTURE_DOUBLE_CLICK == 0 {
            return None;
        }

        Some(ActiveClick(param))
    }

    pub fn on_double_click(&self) -> Option<ProposedParamChange> {
        match &self.0 {
            AnyParameter::InputGain { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::OutputGain { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Tone { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => inner.as_clickable()?.on_double_click(),
            AnyParameter::Blend { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            _ => None,
        }
    }
}
