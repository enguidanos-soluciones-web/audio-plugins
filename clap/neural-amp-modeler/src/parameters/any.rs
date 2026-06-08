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

use super::{Parameter, Range};
use crate::parameters::{blend::Blend, input_gain::InputGain, output_gain::OutputGain, tone::Tone};

pub const PARAMS_COUNT: usize = 4;

pub enum AnyParameter {
    InputGain { inner: Parameter<InputGain, Range> },
    OutputGain { inner: Parameter<OutputGain, Range> },
    Tone { inner: Parameter<Tone, Range> },
    Blend { inner: Parameter<Blend, Range> },
}

impl TryFrom<usize> for AnyParameter {
    type Error = ();

    fn try_from(id: usize) -> Result<Self, Self::Error> {
        match id {
            Parameter::<InputGain, Range>::ID => Ok(AnyParameter::InputGain {
                inner: Parameter::<InputGain, Range>::new(),
            }),
            Parameter::<OutputGain, Range>::ID => Ok(AnyParameter::OutputGain {
                inner: Parameter::<OutputGain, Range>::new(),
            }),
            Parameter::<Tone, Range>::ID => Ok(AnyParameter::Tone {
                inner: Parameter::<Tone, Range>::new(),
            }),
            Parameter::<Blend, Range>::ID => Ok(AnyParameter::Blend {
                inner: Parameter::<Blend, Range>::new(),
            }),
            _ => Err(()),
        }
    }
}
