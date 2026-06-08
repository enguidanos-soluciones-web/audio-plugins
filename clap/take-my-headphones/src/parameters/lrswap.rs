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
pub enum LRSwap {
    Off = 0,
    On = 1,
}

impl Display for LRSwap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "On"),
            Self::Off => write!(f, "Off"),
        }
    }
}

impl FromStr for LRSwap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "On" => Ok(Self::On),
            "Off" => Ok(Self::Off),
            _ => Err(()),
        }
    }
}

impl From<u8> for LRSwap {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::On,
            _ => Self::Off,
        }
    }
}

/// Convert from a raw CLAP parameter value (f64) to [`LRSwap`].
///
/// CLAP stores all parameter values as `f64`, including stepped/enum parameters.
/// This impl rounds to the nearest integer before casting to `u8`, matching the
/// discrete steps defined in [`Parameter<LRSwap, Select>`].
///
/// The cast `f64 → u8` is safe here because valid parameter values are always
/// in `[0.0, 1.0]` (enforced by the host via `min_value`/`max_value`). Values
/// outside that range — which should never occur in practice — fall back to
/// [`LRSwap::OFF`] via the `From<u8>` impl.
impl From<f64> for LRSwap {
    fn from(v: f64) -> Self {
        Self::from(v.round() as u8)
    }
}

impl Parameter<LRSwap, Select> {
    pub const ID: usize = 5;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "L/R Swap",
            gestures: 0,
            behave: Select {
                options: &[LRSwap::Off as u8, LRSwap::On as u8],
                def: LRSwap::Off as u8,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }
}
