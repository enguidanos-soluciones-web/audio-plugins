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

/// Stereo ITD (Interaural Time Difference) delay.
///
/// Applies the same fractional delay to both L and R channels independently.
/// The caller uses the delayed output for the crossed path in bs2b, while
/// passing the un-delayed signal for the direct path.
///
/// Angle → delay mapping (SPL Phonitor 3 / JSFX):
///   0°  → 0 μs
///   75° → 635 μs  (linear interpolation)
pub struct ItdDelay {
    buf_l: Vec<f64>,
    buf_r: Vec<f64>,
    write_ptr: usize,
    capacity: usize,
}

impl ItdDelay {
    /// Max delay at 192 kHz: 635 μs × 192 000 ≈ 122 samples. 512 is safe headroom.
    const CAPACITY: usize = 512;

    pub fn new() -> Self {
        Self {
            buf_l: vec![0.0; Self::CAPACITY],
            buf_r: vec![0.0; Self::CAPACITY],
            write_ptr: 0,
            capacity: Self::CAPACITY,
        }
    }

    pub fn reset(&mut self) {
        self.buf_l.fill(0.0);
        self.buf_r.fill(0.0);
        self.write_ptr = 0;
    }

    /// Convert angle (degrees, 0–75) and sample rate to fractional delay in samples.
    pub fn angle_to_delay_samples(angle_deg: f64, sample_rate: f64) -> f64 {
        const MAX_DELAY_US: f64 = 635.0;
        let delay_us = (angle_deg / 75.0).clamp(0.0, 1.0) * MAX_DELAY_US;
        delay_us * 1e-6 * sample_rate
    }

    /// Write current sample pair to the buffer, then read back with fractional delay.
    /// Returns `(delayed_l, delayed_r)`.
    pub fn process(&mut self, sample: (f64, f64), delay_samples: f64) -> (f64, f64) {
        // Write current sample
        self.buf_l[self.write_ptr] = sample.0;
        self.buf_r[self.write_ptr] = sample.1;

        // Fractional read: linear interpolation between floor and floor+1 samples ago
        let delay_floor = delay_samples.floor() as usize;
        let frac = delay_samples - delay_samples.floor();

        let idx0 = (self.write_ptr + self.capacity - delay_floor) % self.capacity;
        let idx1 = (self.write_ptr + self.capacity - delay_floor - 1) % self.capacity;

        let out_l = self.buf_l[idx0] * (1.0 - frac) + self.buf_l[idx1] * frac;
        let out_r = self.buf_r[idx0] * (1.0 - frac) + self.buf_r[idx1] * frac;

        // Advance write pointer
        self.write_ptr = (self.write_ptr + 1) % self.capacity;

        (out_l, out_r)
    }
}
