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

pub mod any;
pub mod blend;
pub mod input_gain;
pub mod output_gain;
pub mod tone;

use std::marker::PhantomData;

pub const PARAMETER_GESTURE_DRAG: u8 = 1 << 0;
pub const PARAMETER_GESTURE_DOUBLE_CLICK: u8 = 1 << 1;

pub struct ProposedParamChange {
    pub index: usize,
    pub value: f64,
}

#[derive(Clone, Copy)]
pub struct Range {
    pub min: f64,
    pub max: f64,
    pub def: f64,
}

#[derive(Clone, Copy)]
pub struct Parameter<T, R> {
    pub id: usize,
    pub name: &'static str,
    pub gestures: u8,
    pub behave: R,
    pub _marker_type: PhantomData<T>,
    pub _marker_behaviour: PhantomData<R>,
}

#[derive(Clone, Copy)]
pub struct ParameterDraggable<'a, T, R> {
    pub inner: &'a Parameter<T, R>,
    pub _marker_type: PhantomData<T>,
    pub _marker_behaviour: PhantomData<R>,
}

#[derive(Clone, Copy)]
pub struct ParameterClickable<'a, T, R> {
    pub inner: &'a Parameter<T, R>,
    pub _marker_type: PhantomData<T>,
    pub _marker_behaviour: PhantomData<R>,
}

impl<T> Parameter<T, Range> {
    /// Maps `value` from `[min, max]` to `[0.0, 1.0]`.
    ///
    /// Used to convert a raw parameter value (e.g. `-12.0 dB`) into the
    /// normalised form expected by renderers and gesture handlers, where
    /// `0.0` represents `min` and `1.0` represents `max`. Out-of-range
    /// inputs are clamped.
    pub fn normalize(&self, value: f64) -> f64 {
        ((value - self.behave.min) / (self.behave.max - self.behave.min)).clamp(0.0, 1.0)
    }
}
