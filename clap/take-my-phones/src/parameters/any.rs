use super::{Parameter, Range};
use crate::parameters::{cutoff::Cutoff, xfeed::XFeed, mix::Mix};

pub const PARAMS_COUNT: usize = 3;

pub enum AnyParameter {
    Cutoff { inner: Parameter<Cutoff, Range> },
    XFeed  { inner: Parameter<XFeed,  Range> },
    Mix    { inner: Parameter<Mix,    Range> },
}

impl TryFrom<usize> for AnyParameter {
    type Error = ();

    fn try_from(id: usize) -> Result<Self, Self::Error> {
        match id {
            Parameter::<Cutoff, Range>::ID => Ok(AnyParameter::Cutoff { inner: Parameter::<Cutoff, Range>::new() }),
            Parameter::<XFeed,  Range>::ID => Ok(AnyParameter::XFeed  { inner: Parameter::<XFeed,  Range>::new() }),
            Parameter::<Mix,    Range>::ID => Ok(AnyParameter::Mix    { inner: Parameter::<Mix,    Range>::new() }),
            _ => Err(()),
        }
    }
}
