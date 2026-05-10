// Copyright (C) 2026 Cristian A. Enguídanos Nebot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
