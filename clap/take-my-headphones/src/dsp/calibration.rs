use super::pink_noise::PinkNoise;

/// All state needed for calibration mode (pink noise + phase tracking).
pub struct Calibration {
    pub pink_noise: PinkNoise,
    /// Counts samples elapsed within the current intermittent half-period.
    /// Resets every `half_period` samples. Even half = noise on L, odd half = noise on R.
    pub phase: u64,
    pub half_period: u64,
}

impl Calibration {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            pink_noise: PinkNoise::new(),
            phase: 0,
            half_period: (sample_rate * 0.5) as u64, // 500ms per side
        }
    }

    pub fn reset(&mut self) {
        self.pink_noise.reset();
        self.phase = 0;
    }
}
