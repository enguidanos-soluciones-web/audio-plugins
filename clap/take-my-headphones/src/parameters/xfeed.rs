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

#[derive(Clone, Copy)]
pub struct XFeed;

impl Parameter<XFeed, Range> {
    pub const ID: usize = 1;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "XFeed",
            gestures: PARAMETER_GESTURE_DRAG | PARAMETER_GESTURE_DOUBLE_CLICK,
            behave: Range {
                min: 1.0,
                max: 15.0,
                def: 6.2,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    pub fn format_value(value: f64) -> String {
        format!("{:.1}", value)
    }

    pub fn as_draggable(&self) -> Option<ParameterDraggable<'_, XFeed, Range>> {
        if self.gestures & PARAMETER_GESTURE_DRAG != 0 {
            Some(ParameterDraggable::<XFeed, Range>::new(self))
        } else {
            None
        }
    }

    pub fn as_clickable(&self) -> Option<ParameterClickable<'_, XFeed, Range>> {
        if self.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 {
            Some(ParameterClickable::<XFeed, Range>::new(self))
        } else {
            None
        }
    }
}

impl<'a> ParameterDraggable<'a, XFeed, Range> {
    pub fn new(inner: &'a Parameter<XFeed, Range>) -> Self {
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

impl<'a> ParameterClickable<'a, XFeed, Range> {
    pub fn new(inner: &'a Parameter<XFeed, Range>) -> Self {
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

