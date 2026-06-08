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

use std::f64::consts::PI;

/// First-order IIR DC blocker matching the AudioDSPTools HighPass implementation.
/// y[n] = α · (x[n] - x[n-1]) + α · y[n-1]
/// Where α = 1 / (2π·fc/fs + 1)
pub struct DcFilter {
    x_prev: f64,
    y_prev: f64,
    alpha: f64,
}

impl DcFilter {
    pub fn new(cut_freq: f64, sample_rate: f64) -> Self {
        let c = 2.0 * PI * cut_freq / sample_rate;
        Self {
            x_prev: 0.0,
            y_prev: 0.0,
            alpha: 1.0 / (c + 1.0),
        }
    }

    pub fn reset(&mut self) {
        self.x_prev = 0.0;
        self.y_prev = 0.0;
    }

    pub fn process_sample(&mut self, input: f64) -> f64 {
        let output = self.alpha * (input - self.x_prev + self.y_prev);
        self.x_prev = input;
        self.y_prev = output;
        output
    }
}
