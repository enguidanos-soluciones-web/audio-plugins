use crate::parameters::{PARAMETER_GESTURE_DOUBLE_CLICK, ProposedParamChange, any::AnyParameter};

pub struct ActiveClick(AnyParameter);

impl ActiveClick {
    pub fn from_index(index: usize) -> Option<Self> {
        let param = AnyParameter::try_from(index).ok()?;

        let gestures = match &param {
            AnyParameter::Cutoff { inner } => inner.gestures,
            AnyParameter::XFeed   { inner } => inner.gestures,
            AnyParameter::Mix    { inner } => inner.gestures,
        };

        if gestures & PARAMETER_GESTURE_DOUBLE_CLICK == 0 {
            return None;
        }

        Some(ActiveClick(param))
    }

    pub fn on_double_click(&self) -> Option<ProposedParamChange> {
        match &self.0 {
            AnyParameter::Cutoff { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => inner.as_clickable()?.on_double_click(),
            AnyParameter::XFeed   { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => inner.as_clickable()?.on_double_click(),
            AnyParameter::Mix    { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => inner.as_clickable()?.on_double_click(),
            _ => None,
        }
    }
}
