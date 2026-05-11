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
use std::{fmt::Display, str::FromStr};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Solo {
    L = 0,
    OFF = 1,
    R = 2,
}

impl Display for Solo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::L => write!(f, "L"),
            Self::OFF => write!(f, "Off"),
            Self::R => write!(f, "R"),
        }
    }
}

impl FromStr for Solo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::L),
            "Off" => Ok(Self::OFF),
            "R" => Ok(Self::R),
            _ => Err(()),
        }
    }
}

impl From<u8> for Solo {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::L,
            2 => Self::R,
            _ => Self::OFF,
        }
    }
}

/// Convert from a raw CLAP parameter value (f64) to [`Solo`].
///
/// CLAP stores all parameter values as `f64`, including stepped/enum parameters.
/// This impl rounds to the nearest integer before casting to `u8`, matching the
/// discrete steps defined in [`Parameter<Solo, Select>`].
///
/// The cast `f64 → u8` is safe here because valid parameter values are always
/// in `[0.0, 2.0]` (enforced by the host via `min_value`/`max_value`). Values
/// outside that range — which should never occur in practice — fall back to
/// [`Solo::OFF`] via the `From<u8>` impl.
impl From<f64> for Solo {
    fn from(v: f64) -> Self {
        Self::from(v.round() as u8)
    }
}

impl Parameter<Solo, Select> {
    pub const ID: usize = 6;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "Solo",
            gestures: 0,
            behave: Select {
                options: &[Solo::L as u8, Solo::OFF as u8, Solo::R as u8],
                def: Solo::OFF as u8,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }
}
