use super::{Parameter, Select};

#[derive(Clone, Copy)]
pub struct LRSwap;

impl LRSwap {
    pub const OFF: u8 = 0;
    pub const ON: u8 = 1;

    pub fn label(v: u8) -> &'static str {
        match v {
            Self::ON => "On",
            _ => "Off",
        }
    }
}

impl Parameter<LRSwap, Select> {
    pub const ID: usize = 5;

    pub const fn new() -> Self {
        Self {
            id: Self::ID,
            name: "LR Swap",
            gestures: 0,
            behave: Select {
                options: &[LRSwap::OFF, LRSwap::ON],
                def: LRSwap::OFF,
            },
            _marker_type: std::marker::PhantomData,
            _marker_behaviour: std::marker::PhantomData,
        }
    }
}
