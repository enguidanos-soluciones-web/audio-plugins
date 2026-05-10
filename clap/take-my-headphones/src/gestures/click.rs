use crate::parameters::{PARAMETER_GESTURE_DOUBLE_CLICK, ProposedParamChange, any::AnyParameter};

pub struct ActiveClick(AnyParameter);

impl ActiveClick {
    pub fn from_index(index: usize) -> Option<Self> {
        let param = AnyParameter::try_from(index).ok()?;

        let gestures = match &param {
            AnyParameter::Cutoff { inner } => inner.gestures,
            AnyParameter::XFeed { inner } => inner.gestures,
            AnyParameter::Center { inner } => inner.gestures,
            AnyParameter::Angle { inner } => inner.gestures,
            AnyParameter::Gain { inner } => inner.gestures,
            AnyParameter::CalibrationMode { inner } => inner.gestures,
            AnyParameter::LRSwap { inner } => inner.gestures,
            AnyParameter::Solo { inner } => inner.gestures,
            AnyParameter::Phase { inner } => inner.gestures,
        };

        if gestures & PARAMETER_GESTURE_DOUBLE_CLICK == 0 {
            return None;
        }

        Some(ActiveClick(param))
    }

    pub fn on_double_click(&self) -> Option<ProposedParamChange> {
        match &self.0 {
            AnyParameter::Cutoff { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::XFeed { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Center { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Angle { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            AnyParameter::Gain { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => inner.as_clickable()?.on_double_click(),
            AnyParameter::CalibrationMode { inner } if inner.gestures & PARAMETER_GESTURE_DOUBLE_CLICK != 0 => {
                inner.as_clickable()?.on_double_click()
            }
            _ => None,
        }
    }
}
