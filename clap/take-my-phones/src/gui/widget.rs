use vello::Scene;

pub trait Widget: Sync {
    fn dom_id(&self) -> &'static str;
    fn param_id(&self) -> usize;

    fn draw(&self, scene: &mut Scene, coordinates: (f64, f64), dimensions: (f64, f64), cursor: (f64, f64), value: f64);
}
