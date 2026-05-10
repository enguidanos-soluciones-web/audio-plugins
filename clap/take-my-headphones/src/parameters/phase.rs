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

use super::{Parameter, Select};

#[derive(Clone, Copy)]
pub struct Phase;

impl Phase {
    pub const L: u8 = 0;
    pub const OFF: u8 = 1;
    pub const R: u8 = 2;

    pub fn label(v: u8) -> &'static str {
        match v {
            Self::L => "L",
            Self::R => "R",
            _ => "Off",
        }
    }
}

impl Parameter<Phase, Select> {
    pub const ID: usize = 7;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "Phase",
            gestures: 0,
            behave: Select {
                options: &[Phase::L, Phase::OFF, Phase::R],
                def: Phase::OFF,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }
}
