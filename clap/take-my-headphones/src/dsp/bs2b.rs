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

/// Coefficients for the bs2b two-filter crossfeed structure.
///
/// bs2b simulates the natural crosstalk that occurs when listening to stereo
/// speakers: the right ear hears a delayed, attenuated, low-pass-filtered copy
/// of the left channel and vice versa. Without this, headphone listening is
/// unnaturally wide because each ear receives only one channel.
///
/// Each output channel is the sum of two filtered paths:
///
/// ```text
/// outL = highboost(inL) + lp(inR)
/// outR = highboost(inR) + lp(inL)
/// ```
///
/// - **LP (crossed path)**: attenuated one-pole lowpass applied to the opposite
///   channel, simulating the high-frequency shadowing of the head (HRTF).
/// - **Highboost (direct path)**: first-order IIR shelf that boosts highs
///   slightly to compensate for the overall level reduction introduced by the
///   LP mix.
///
/// Reference: <https://bs2b.sourceforge.net/>
///
/// Canonical presets:
///
/// | Name      | Fc      | Feed    | Gd       | Ad_h     |
/// |-----------|---------|---------|----------|----------|
/// | Default   | 700 Hz  | 4.5 dB  | −6.75 dB | −2.25 dB |
/// | Chu Moy   | 700 Hz  | 6.0 dB  | −8.0 dB  | −2.0 dB  |
/// | Jan Meier | 650 Hz  | 9.5 dB  | −10.917 dB | −1.417 dB |
/// 5 × f64 = 40 bytes of data, padded to 64 bytes by `#[repr(align(64))]`.
/// The alignment pins the struct to a cache line boundary so it is never split
/// across two lines — avoiding an extra cache miss on every sample. The 24 bytes
/// of trailing padding are the cost of that guarantee.
///
/// # Size assertion
///
/// ```
/// assert_eq!(std::mem::size_of::<Bs2bCoefficients>(), 64);
/// ```
#[derive(Clone, Copy, Default)]
#[repr(align(64))]
pub struct Bs2bCoefficients {
    /// One-pole LP (crossed path): `y[n] = a0·x[n] + b1·y[n-1]`
    pub a0: f64,
    pub b1: f64,
    /// First-order IIR highboost (direct path): `y[n] = a0_h·x[n] + a1_h·x[n-1] + b1_h·y[n-1]`
    pub a0_h: f64,
    pub a1_h: f64,
    pub b1_h: f64,
}

const _: () = assert!(std::mem::size_of::<Bs2bCoefficients>() == 64);

impl Bs2bCoefficients {
    /// Compute coefficients from cutoff frequency, crossfeed level, and sample rate.
    ///
    /// `feed_db` is the total perceived crossfeed. It is split into two parts
    /// using the 3:1 ratio from the bs2b Default preset:
    ///
    /// - `Gd = -(feed * 1.5)` — attenuation of the crossed LP path in dB.
    /// - `Ad_h = -(feed * 0.5)` — low-frequency attenuation of the direct highboost in dB.
    ///
    /// The perceived crossfeed equals `|Gd| - |Ad_h| = feed_db` (the two paths
    /// partially cancel, leaving the intended feed level at the listener's ear).
    /// The 3:1 ratio is empirical, derived from Sergey Umansky's original presets.
    pub fn compute(fc_hz: f64, feed_db: f64, sample_rate: f64) -> Self {
        let gd = -(feed_db * 1.5);
        let ad_h = -(feed_db * 0.5);

        let g = 10f64.powf(gd / 20.0);
        let a_h = 10f64.powf(ad_h / 20.0);
        let g_h = 1.0 - a_h;

        let gd_h = 20.0 * g_h.ln() / 10f64.ln();
        let fc_h = fc_hz * 2f64.powf((gd - gd_h) / 12.0);

        let x = (-2.0 * PI * fc_hz / sample_rate).exp();
        let x_h = (-2.0 * PI * fc_h / sample_rate).exp();

        Self {
            a0: g * (1.0 - x),
            b1: x,
            a0_h: 1.0 - g_h * (1.0 - x_h),
            a1_h: -x_h,
            b1_h: x_h,
        }
    }
}

/// Per-channel filter state for the bs2b crossfeed structure.
///
/// Each channel owns the delay memory for both its filters:
/// - The **lowpass** state (`lowpass_y1`) is fed the *opposite* channel's signal
///   and its output is mixed into this channel's output.
/// - The **highboost** state (`highboost_x1`, `highboost_y1`) is fed this
///   channel's own signal for the direct path.
pub struct Bs2bChannel {
    pub lowpass_y1: f64,
    pub highboost_x1: f64,
    pub highboost_y1: f64,
}

impl Bs2bChannel {
    /// Create a new channel with zeroed filter state.
    pub fn new() -> Self {
        Self {
            lowpass_y1: 0.0,
            highboost_x1: 0.0,
            highboost_y1: 0.0,
        }
    }

    /// Clear all filter state (use on transport stop or reset).
    pub fn reset(&mut self) {
        self.lowpass_y1 = 0.0;
        self.highboost_x1 = 0.0;
        self.highboost_y1 = 0.0;
    }

    /// One-pole lowpass: `y[n] = a0·x[n] + b1·y[n-1]`
    ///
    /// Applied to the opposite channel's signal; output is mixed into this
    /// channel's output to simulate head-shadowing crosstalk.
    pub fn lowpass(&mut self, x: f64, coeffs: &Bs2bCoefficients) -> f64 {
        let y = coeffs.a0 * x + coeffs.b1 * self.lowpass_y1;
        self.lowpass_y1 = y;
        y
    }

    /// First-order IIR highboost: `y[n] = a0_h·x[n] + a1_h·x[n-1] + b1_h·y[n-1]`
    ///
    /// Applied to this channel's own signal. The negative `a1_h` creates a
    /// high-frequency shelf that compensates for the low-frequency energy added
    /// by the crossed LP path, preserving the overall tonal balance.
    pub fn highboost(&mut self, x: f64, coeffs: &Bs2bCoefficients) -> f64 {
        let y = coeffs.a0_h * x + coeffs.a1_h * self.highboost_x1 + coeffs.b1_h * self.highboost_y1;
        self.highboost_x1 = x;
        self.highboost_y1 = y;
        y
    }
}

/// Top-level bs2b processor: two [`Bs2bChannel`]s and shared [`Bs2bCoefficients`].
pub struct Bs2b {
    pub left: Bs2bChannel,
    pub right: Bs2bChannel,
    pub coeffs: Bs2bCoefficients,
}

impl Bs2b {
    /// Create a new processor with zeroed state and default (unity) coefficients.
    pub fn new() -> Self {
        Self {
            left: Bs2bChannel::new(),
            right: Bs2bChannel::new(),
            coeffs: Bs2bCoefficients::default(),
        }
    }

    /// Recompute coefficients. Call once per audio block, before [`Bs2b::process_with_itd`].
    pub fn update_coeffs(&mut self, fc_hz: f64, feed_db: f64, sample_rate: f64) {
        self.coeffs = Bs2bCoefficients::compute(fc_hz, feed_db, sample_rate);
    }

    /// Clear filter state on both channels (use on transport stop or reset).
    pub fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
    }

    /// Process one stereo sample with pre-computed ITD delay.
    ///
    /// `sample` is the undelayed signal used for the **direct** (highboost) path.
    /// `delayed` is the ITD-delayed signal used for the **crossed** (LP) path.
    /// The two are kept separate so the ITD delay only affects the crossfeed,
    /// not the direct path — matching real speaker geometry.
    ///
    /// ```text
    /// outL = highboost(sample.0) + lp(delayed.1)
    /// outR = highboost(sample.1) + lp(delayed.0)
    /// ```
    pub fn process_with_itd(&mut self, sample: (f64, f64), delayed: (f64, f64)) -> (f64, f64) {
        let highboost_l = self.left.highboost(sample.0, &self.coeffs);
        let highboost_r = self.right.highboost(sample.1, &self.coeffs);

        let lowpass_l = self.left.lowpass(delayed.0, &self.coeffs);
        let lowpass_r = self.right.lowpass(delayed.1, &self.coeffs);

        (highboost_l + lowpass_r, highboost_r + lowpass_l)
    }
}
