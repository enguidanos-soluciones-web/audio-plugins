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

pub struct LowPassFilter {
    alpha: f64,
    prev: f64,
}

impl LowPassFilter {
    pub fn new(cutoff_hz: f64, sample_rate: f64) -> Self {
        let alpha = 1.0 - (-2.0 * std::f64::consts::PI * cutoff_hz / sample_rate).exp();
        Self { alpha, prev: 0.0 }
    }

    pub fn process_sample(&mut self, input: f64) -> f64 {
        self.prev = self.alpha * input + (1.0 - self.alpha) * self.prev;
        self.prev
    }

    pub fn reset(&mut self) {
        self.prev = 0.0;
    }

    pub fn set_cutoff(&mut self, cutoff_hz: f64, sample_rate: f64) {
        self.alpha = 1.0 - (-2.0 * std::f64::consts::PI * cutoff_hz / sample_rate).exp();
    }
}
