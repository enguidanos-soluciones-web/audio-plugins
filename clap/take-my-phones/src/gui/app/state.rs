use crate::parameters::any::PARAMS_COUNT;

#[derive(Clone, Default)]
pub struct AppState {
    pub params: [f64; PARAMS_COUNT],
}
