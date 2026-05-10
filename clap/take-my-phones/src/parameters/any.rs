use super::{Parameter, Range};
use crate::parameters::{cutoff::Cutoff, feed::Feed, mix::Mix};

pub const PARAMS_COUNT: usize = 3;

pub enum AnyParameter {
    Cutoff  { inner: Parameter<Cutoff, Range> },
    Feed    { inner: Parameter<Feed,   Range> },
    Mix     { inner: Parameter<Mix,    Range> },
}

impl TryFrom<usize> for AnyParameter {
    type Error = ();

    fn try_from(id: usize) -> Result<Self, Self::Error> {
        match id {
            Parameter::<Cutoff, Range>::ID => Ok(AnyParameter::Cutoff { inner: Parameter::<Cutoff, Range>::new() }),
            Parameter::<Feed,   Range>::ID => Ok(AnyParameter::Feed   { inner: Parameter::<Feed,   Range>::new() }),
            Parameter::<Mix,    Range>::ID => Ok(AnyParameter::Mix    { inner: Parameter::<Mix,    Range>::new() }),
            _ => Err(()),
        }
    }
}
