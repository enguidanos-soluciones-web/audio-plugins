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
            AnyParameter::InputGain { inner } => inner.normalize(raw),
            AnyParameter::OutputGain { inner } => inner.normalize(raw),
            AnyParameter::Tone { inner } => inner.normalize(raw),
            AnyParameter::Blend { inner } => inner.normalize(raw),
        };

        let is_draggable = match &param {
            AnyParameter::InputGain { inner } => inner.as_draggable().is_some(),
            AnyParameter::OutputGain { inner } => inner.as_draggable().is_some(),
            AnyParameter::Tone { inner } => inner.as_draggable().is_some(),
            AnyParameter::Blend { inner } => inner.as_draggable().is_some(),
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
            AnyParameter::InputGain { inner } => inner.id,
            AnyParameter::OutputGain { inner } => inner.id,
            AnyParameter::Tone { inner } => inner.id,
            AnyParameter::Blend { inner } => inner.id,
        }
    }

    pub fn on_drag(&self, x: f64, y: f64) -> Option<ProposedParamChange> {
        match &self.param {
            AnyParameter::InputGain { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::OutputGain { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Tone { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Blend { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
        }
    }
}
