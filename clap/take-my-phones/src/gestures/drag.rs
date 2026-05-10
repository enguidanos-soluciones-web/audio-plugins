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
            AnyParameter::Mix { inner } => inner.normalize(raw),
            AnyParameter::CalibrationMode { .. } => return None, // dropdown — no drag
        };

        let is_draggable = match &param {
            AnyParameter::Cutoff { inner } => inner.as_draggable().is_some(),
            AnyParameter::XFeed { inner } => inner.as_draggable().is_some(),
            AnyParameter::Mix { inner } => inner.as_draggable().is_some(),
            AnyParameter::CalibrationMode { .. } => false,
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
            AnyParameter::Mix { inner } => inner.id,
            AnyParameter::CalibrationMode { inner } => inner.id,
        }
    }

    pub fn on_drag(&self, x: f64, y: f64) -> Option<ProposedParamChange> {
        match &self.param {
            AnyParameter::Cutoff { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::XFeed { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::Mix { inner } => inner.as_draggable()?.on_drag(self.start_pos, self.start_value, (x, y)),
            AnyParameter::CalibrationMode { .. } => None,
        }
    }
}
