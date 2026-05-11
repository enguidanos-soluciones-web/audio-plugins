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

/// State for calibration mode: pink noise generator and intermittent phase counter.
///
/// Two calibration modes are supported:
///
/// - **Continuous**: mono pink noise on L only. Lets the user tune `Cutoff` and
///   `XFeed` by listening to how much of the signal bleeds into the right ear via
///   the crossfeed LP path.
///
/// - **Intermittent**: alternates 500 ms of noise on L, then 500 ms on R, in a
///   loop. Lets the user tune `Angle` by listening to how the ITD delay
///   externalises the image to the side.
///
/// Both modes output pink noise at -12 dBFS via [`PinkNoise`].
pub struct Calibration {
    pub pink_noise: PinkNoise,
    /// Sample counter within the current full period (`2 * half_period`).
    /// `phase < half_period` → noise on L; `phase >= half_period` → noise on R.
    pub phase: u64,
    /// Number of samples per side (500 ms). Computed once from `sample_rate` in
    /// [`Calibration::new`] and never changes during a session.
    pub half_period: u64,
}

impl Calibration {
    /// Create calibration state for the given sample rate.
    ///
    /// `half_period` is set to 500 ms (`sample_rate * 0.5`).
    pub fn new(sample_rate: f64) -> Self {
        Self {
            pink_noise: PinkNoise::new(),
            phase: 0,
            half_period: (sample_rate * 0.5) as u64,
        }
    }

    /// Reset the phase counter and pink noise filter state.
    ///
    /// Does not reinitialise the PRNG — see [`PinkNoise::reset`].
    pub fn reset(&mut self) {
        self.pink_noise.reset();
        self.phase = 0;
    }
}
