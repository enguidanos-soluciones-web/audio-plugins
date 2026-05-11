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
            AnyParameter::Cutoff { inner } => inner.gestures,
            AnyParameter::XFeed { inner } => inner.gestures,
            AnyParameter::Center { inner } => inner.gestures,
            AnyParameter::Angle { inner } => inner.gestures,
            AnyParameter::Gain { inner } => inner.gestures,
            AnyParameter::XFeedSlope { inner } => inner.gestures,
            AnyParameter::Bs2bLowShelf { inner } => inner.gestures,
            AnyParameter::CalibrationMode { inner } => inner.gestures,
            AnyParameter::LRSwap { inner } => inner.gestures,
            AnyParameter::Solo { inner } => inner.gestures,
            AnyParameter::Phase { inner } => inner.gestures,
        };

        if gestures & PARAMETER_GESTURE_DOUBLE_CLICK == 0 {
            return None;
        }

        Some(ActiveClick(param))
    }

    pub fn on_double_click(&self) -> Option<ProposedParamChange> {
        match &self.0 {
            AnyParameter::Cutoff { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::XFeed { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Center { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Angle { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Gain { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => inner.as_clickable()?.on_double_click(),
            AnyParameter::XFeedSlope { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Bs2bLowShelf { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::CalibrationMode { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            _ => None,
        }
    }
}
