use super::{Parameter, Select};

#[derive(Clone, Copy)]
pub struct Solo;

impl Solo {
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

impl Parameter<Solo, Select> {
    pub const ID: usize = 6;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "Solo",
            gestures: 0,
            behave: Select {
                options: &[Solo::L, Solo::OFF, Solo::R],
                def: Solo::OFF,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }
}
