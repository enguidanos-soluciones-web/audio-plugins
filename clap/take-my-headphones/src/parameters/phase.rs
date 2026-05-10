use super::{Parameter, Select};

#[derive(Clone, Copy)]
pub struct Phase;

impl Phase {
    pub const L: u8 = 0;
    pub const OFF: u8 = 1;
    pub const R: u8 = 2;

    pub fn label(v: u8) -> &'static str {
        match v {
            Self::L => "L",
            Self::R => "R",
            _ => "Off",
        }
    }
}

impl Parameter<Phase, Select> {
    pub const ID: usize = 7;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "Phase",
            gestures: 0,
            behave: Select {
                options: &[Phase::L, Phase::OFF, Phase::R],
                def: Phase::OFF,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }
}
