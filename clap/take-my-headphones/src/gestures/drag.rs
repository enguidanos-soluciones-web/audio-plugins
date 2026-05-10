use crate::parameters::{ProposedParamChange, any::AnyParameter};

pub struct ActiveDrag {
    param: AnyParameter,
    start_pos: (f64, f64),
    start_value: f64,
}

impl ActiveDrag {
    pub fn from_index(index: usize, x: f64, y: f64, raw: f64) -> Option<Self> {
        let param = AnyParameter::try_from(index).ok()?;

        let start_value = match &param {
            AnyParameter::Cutoff { inner } => inner.normalize(raw),
            AnyParameter::XFeed { inner } => inner.normalize(raw),
            AnyParameter::Center { inner } => inner.normalize(raw),
            AnyParameter::Angle { inner } => inner.normalize(raw),
            AnyParameter::Gain { inner } => inner.normalize(raw),
            AnyParameter::CalibrationMode { .. } | AnyParameter::LRSwap { .. } | AnyParameter::Solo { .. } | AnyParameter::Phase { .. } => {
                return None;
            } // dropdown — no drag
        };

        let is_draggable = match &param {
            AnyParameter::Cutoff { inner } => inner.as_draggable().is_some(),
            AnyParameter::XFeed { inner } => inner.as_draggable().is_some(),
            AnyParameter::Center { inner } => inner.as_draggable().is_some(),
            AnyParameter::Angle { inner } => inner.as_draggable().is_some(),
            AnyParameter::Gain { inner } => inner.as_draggable().is_some(),
            AnyParameter::CalibrationMode { .. } | AnyParameter::LRSwap { .. } | AnyParameter::Solo { .. } | AnyParameter::Phase { .. } => {
                false
            }
        };

        if !is_draggable {
            return None;
        }

        Some(Self {
            param,
            start_pos: (x, y),
            start_value,
        })
    }

    pub fn param_id(&self) -> usize {
        match &self.param {
            AnyParameter::Cutoff { inner } => inner.id,
            AnyParameter::XFeed { inner } => inner.id,
            AnyParameter::Center { inner } => inner.id,
            AnyParameter::Angle { inner } => inner.id,
            AnyParameter::Gain { inner } => inner.id,
            AnyParameter::CalibrationMode { inner } => inner.id,
            AnyParameter::LRSwap { inner } => inner.id,
            AnyParameter::Solo { inner } => inner.id,
            AnyParameter::Phase { inner } => inner.id,
        }
    }

    pub fn on_drag(&self, x: f64, y: f64) -> Option<ProposedParamChange> {
        match &self.param {
            AnyParameter::Cutoff { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::XFeed { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Center { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Angle { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Gain { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::CalibrationMode { .. } | AnyParameter::LRSwap { .. } | AnyParameter::Solo { .. } | AnyParameter::Phase { .. } => {
                None
            }
        }
    }
}
