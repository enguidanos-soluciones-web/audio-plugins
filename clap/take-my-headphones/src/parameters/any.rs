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

use super::{Parameter, Range, Select};
use crate::parameters::{
    angle::Angle, bs2b_low_shelf::Bs2bLowShelf, calibration_mode::CalibrationMode, center::Center, cutoff::Cutoff, gain::Gain,
    lrswap::LRSwap, phase::Phase, solo::Solo, xfeed::XFeed, xfeed_slope::XFeedSlope,
};

pub const PARAMS_COUNT: usize = 11;

pub enum AnyParameter {
    Cutoff { inner: Parameter<Cutoff, Range> },
    XFeed { inner: Parameter<XFeed, Range> },
    XFeedSlope { inner: Parameter<XFeedSlope, Range> },
    Bs2bLowShelf { inner: Parameter<Bs2bLowShelf, Range> },
    CalibrationMode { inner: Parameter<CalibrationMode, Select> },
    Center { inner: Parameter<Center, Range> },
    Angle { inner: Parameter<Angle, Range> },
    LRSwap { inner: Parameter<LRSwap, Select> },
    Solo { inner: Parameter<Solo, Select> },
    Phase { inner: Parameter<Phase, Select> },
    Gain { inner: Parameter<Gain, Range> },
}

impl TryFrom<usize> for AnyParameter {
    type Error = ();

    fn try_from(id: usize) -> Result<Self, Self::Error> {
        match id {
            Parameter::<Cutoff, Range>::ID => Ok(AnyParameter::Cutoff {
                inner: Parameter::<Cutoff, Range>::new(),
            }),
            Parameter::<XFeed, Range>::ID => Ok(AnyParameter::XFeed {
                inner: Parameter::<XFeed, Range>::new(),
            }),
            Parameter::<XFeedSlope, Range>::ID => Ok(AnyParameter::XFeedSlope {
                inner: Parameter::<XFeedSlope, Range>::new(),
            }),
            Parameter::<Bs2bLowShelf, Range>::ID => Ok(AnyParameter::Bs2bLowShelf {
                inner: Parameter::<Bs2bLowShelf, Range>::new(),
            }),
            Parameter::<CalibrationMode, Select>::ID => Ok(AnyParameter::CalibrationMode {
                inner: Parameter::<CalibrationMode, Select>::new(),
            }),
            Parameter::<Center, Range>::ID => Ok(AnyParameter::Center {
                inner: Parameter::<Center, Range>::new(),
            }),
            Parameter::<Angle, Range>::ID => Ok(AnyParameter::Angle {
                inner: Parameter::<Angle, Range>::new(),
            }),
            Parameter::<LRSwap, Select>::ID => Ok(AnyParameter::LRSwap {
                inner: Parameter::<LRSwap, Select>::new(),
            }),
            Parameter::<Solo, Select>::ID => Ok(AnyParameter::Solo {
                inner: Parameter::<Solo, Select>::new(),
            }),
            Parameter::<Phase, Select>::ID => Ok(AnyParameter::Phase {
                inner: Parameter::<Phase, Select>::new(),
            }),
            Parameter::<Gain, Range>::ID => Ok(AnyParameter::Gain {
                inner: Parameter::<Gain, Range>::new(),
            }),
            _ => Err(()),
        }
    }
}
