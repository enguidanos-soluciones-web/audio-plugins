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

use crate::parameters::{ProposedParamChange, any::AnyParameter};

pub struct ActiveDrag {
    param: AnyParameter,
    start_pos: (f64, f64),
    start_value: f64,
}

impl ActiveDrag {
    pub fn from_index(index: usize, x: f64, y: f64, raw: f64) -> Option<Self> {
        let param = AnyParameter::try_from(index).ok()?;

        let start_value = match &param {
            AnyParameter::Cutoff { inner } => inner.normalize(raw),
            AnyParameter::XFeed { inner } => inner.normalize(raw),
            AnyParameter::XFeedSlope { inner } => inner.normalize(raw),
            AnyParameter::Bs2bLowShelf { inner } => inner.normalize(raw),
            AnyParameter::Center { inner } => inner.normalize(raw),
            AnyParameter::Angle { inner } => inner.normalize(raw),
            AnyParameter::Gain { inner } => inner.normalize(raw),
            AnyParameter::CalibrationMode { .. } | AnyParameter::LRSwap { .. } | AnyParameter::Solo { .. } | AnyParameter::Phase { .. } => {
                return None;
            } // dropdown — no drag
        };

        let is_draggable = match &param {
            AnyParameter::Cutoff { inner } => inner.as_draggable().is_some(),
            AnyParameter::XFeed { inner } => inner.as_draggable().is_some(),
            AnyParameter::XFeedSlope { inner } => inner.as_draggable().is_some(),
            AnyParameter::Bs2bLowShelf { inner } => inner.as_draggable().is_some(),
            AnyParameter::Center { inner } => inner.as_draggable().is_some(),
            AnyParameter::Angle { inner } => inner.as_draggable().is_some(),
            AnyParameter::Gain { inner } => inner.as_draggable().is_some(),
            AnyParameter::CalibrationMode { .. } | AnyParameter::LRSwap { .. } | AnyParameter::Solo { .. } | AnyParameter::Phase { .. } => {
                false
            }
        };

        if !is_draggable {
            return None;
        }

        Some(Self {
            param,
            start_pos: (x, y),
            start_value,
        })
    }

    pub fn param_id(&self) -> usize {
        match &self.param {
            AnyParameter::Cutoff { inner } => inner.id,
            AnyParameter::XFeed { inner } => inner.id,
            AnyParameter::XFeedSlope { inner } => inner.id,
            AnyParameter::Bs2bLowShelf { inner } => inner.id,
            AnyParameter::Center { inner } => inner.id,
            AnyParameter::Angle { inner } => inner.id,
            AnyParameter::Gain { inner } => inner.id,
            AnyParameter::CalibrationMode { inner } => inner.id,
            AnyParameter::LRSwap { inner } => inner.id,
            AnyParameter::Solo { inner } => inner.id,
            AnyParameter::Phase { inner } => inner.id,
        }
    }

    pub fn on_drag(&self, x: f64, y: f64) -> Option<ProposedParamChange> {
        match &self.param {
            AnyParameter::Cutoff { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::XFeed { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::XFeedSlope { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Bs2bLowShelf { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Center { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Angle { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Gain { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::CalibrationMode { .. } | AnyParameter::LRSwap { .. } | AnyParameter::Solo { .. } | AnyParameter::Phase { .. } => {
                None
            }
        }
    }
}
