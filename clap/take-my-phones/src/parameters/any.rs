use super::{Parameter, Range, Select};
use crate::parameters::{calibration_mode::CalibrationMode, cutoff::Cutoff, mix::Mix, xfeed::XFeed};

pub const PARAMS_COUNT: usize = 4;

pub enum AnyParameter {
    Cutoff { inner: Parameter<Cutoff, Range> },
    XFeed { inner: Parameter<XFeed, Range> },
    Mix { inner: Parameter<Mix, Range> },
    CalibrationMode { inner: Parameter<CalibrationMode, Select> },
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
            Parameter::<Mix, Range>::ID => Ok(AnyParameter::Mix {
                inner: Parameter::<Mix, Range>::new(),
            }),
            Parameter::<CalibrationMode, Select>::ID => Ok(AnyParameter::CalibrationMode {
                inner: Parameter::<CalibrationMode, Select>::new(),
            }),
            _ => Err(()),
        }
    }
}
