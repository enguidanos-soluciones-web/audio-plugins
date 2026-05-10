use super::{Parameter, Range, Select};
use crate::parameters::{
    angle::Angle, calibration_mode::CalibrationMode, center::Center, cutoff::Cutoff, lrswap::LRSwap, phase::Phase, solo::Solo, xfeed::XFeed,
};

pub const PARAMS_COUNT: usize = 8;

pub enum AnyParameter {
    Cutoff { inner: Parameter<Cutoff, Range> },
    XFeed { inner: Parameter<XFeed, Range> },
    CalibrationMode { inner: Parameter<CalibrationMode, Select> },
    Center { inner: Parameter<Center, Range> },
    Angle { inner: Parameter<Angle, Range> },
    LRSwap { inner: Parameter<LRSwap, Select> },
    Solo { inner: Parameter<Solo, Select> },
    Phase { inner: Parameter<Phase, Select> },
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
            _ => Err(()),
        }
    }
}
