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

/// Marker type for the bs2b direct-path low shelf filter gain parameter.
///
/// Controls `ls_db` — the low-frequency gain of the first-order IIR low shelf
/// on the **direct path** of the bs2b crossfeed structure. The filter attenuates
/// low frequencies and passes high frequencies at unity, which perceptually
/// appears as a high-frequency lift on the direct path.
///
/// At DC (ω = 0): gain = `ls_db`.
/// At Nyquist: gain → 0 dB (unity).
/// Transition frequency: derived from Cutoff and XFeed internals.
///
/// | ls_db  | Character                                                |
/// |--------|----------------------------------------------------------|
/// | 0 dB   | No attenuation — direct path flat across all frequencies |
/// | −3 dB  | Moderate shelf — typical for medium crossfeed levels     |
/// | −6 dB  | Deep shelf — strong low-frequency attenuation            |
///
/// In the original bs2b design this value was derived from XFeed via the fixed
/// 3:1 ratio `ls_db = −XFeed / 2`. This parameter breaks that coupling so the
/// shelf can be tuned independently.
///
/// # Relationship to XFeed
///
/// Perceived crossfeed = `|Gd| − |ls_db|`, where `Gd = −XFeed × 1.5`.
/// Raising `ls_db` toward 0 dB reduces perceived crossfeed without changing
/// the LP gain; lowering it deepens the shelf and increases perceived crossfeed.
#[derive(Clone, Copy)]
pub struct Bs2bLowShelf;

impl Parameter<Bs2bLowShelf, Range> {
    pub const ID: usize = 10;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "Low Shelf",
            gestures: PARAMETER_GESTURE_DRAG | PARAMETER_GESTURE_DOUBLE_CLICK,
            behave: Range {
                min: -12.0,
                max: 0.0,
                def: -3.1,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }

    /// Format low shelf gain for display — two decimal places with unit (e.g. `"-3.10 dB"`).
    pub fn format_value(value: f64) -> String {
        format!("{:.2} dB", value)
    }

    pub fn as_draggable(&self) -> Option<ParameterDraggable<'_, Bs2bLowShelf, Range>> {
        if self.gestures & PARAMETER_GESTURE_DRAG != 0 {
            Some(ParameterDraggable::<Bs2bLowShelf, Range>::new(self))
        } else {
            None
        }
    }

    pub fn as_clickable(&self) -> Option<ParameterClickable<'_, Bs2bLowShelf, Range>> {
        if self.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 {
            Some(ParameterClickable::<Bs2bLowShelf, Range>::new(self))
        } else {
            None
        }
    }
}

impl<'a> ParameterDraggable<'a, Bs2bLowShelf, Range> {
    pub fn new(inner: &'a Parameter<Bs2bLowShelf, Range>) -> Self {
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

impl<'a> ParameterClickable<'a, Bs2bLowShelf, Range> {
    pub fn new(inner: &'a Parameter<Bs2bLowShelf, Range>) -> Self {
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
