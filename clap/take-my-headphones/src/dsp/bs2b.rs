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
/// outL = low_shelf(inL) + lp(inR)
/// outR = low_shelf(inR) + lp(inL)
/// ```
///
/// - **LP (crossed path)**: attenuated second-order (biquad) lowpass applied to
///   the opposite channel, simulating the high-frequency shadowing of the head
///   (HRTF). The Q parameter controls the slope sharpness around `fc_hz`:
///   `Q = 0.707` (Butterworth) gives the maximally flat response; lower Q
///   broadens the rolloff, higher Q introduces a resonance peak.
/// - **Low shelf (direct path)**: first-order IIR filter that attenuates low
///   frequencies on the direct channel, leaving highs at unity gain, to
///   compensate for the overall level reduction introduced by the LP mix.
///
/// Reference: <https://bs2b.sourceforge.net/>
///
/// Canonical presets:
///
/// | Name      | Fc      | Feed    | Gd         | Ad_h      |
/// |-----------|---------|---------|------------|-----------|
/// | Default   | 700 Hz  | 4.5 dB  | −6.75 dB   | −2.25 dB  |
/// | Chu Moy   | 700 Hz  | 6.0 dB  | −8.0 dB    | −2.0 dB   |
/// | Jan Meier | 650 Hz  | 9.5 dB  | −10.917 dB | −1.417 dB |
///
/// # Size
///
/// 8 × f64 = 64 bytes of data, aligned to 64 bytes by `#[repr(align(64))]`.
/// Zero padding — the struct fills one cache line exactly.
#[derive(Clone, Copy, Default)]
#[repr(align(64))]
pub struct Bs2bCoefficients {
    /// Biquad LP feedforward coefficients (crossed path), Direct Form II Transposed.
    ///
    /// `H(z) = (b0_lp + b1_lp·z⁻¹ + b2_lp·z⁻²) / (1 + a1_lp·z⁻¹ + a2_lp·z⁻²)`
    ///
    /// All three feedforward coefficients are pre-scaled by the crossed path gain
    /// `g = 10^(Gd/20)`, so DC gain = `g` without a separate multiply per sample.
    pub b0_lp: f64,
    pub b1_lp: f64,
    pub b2_lp: f64,
    /// Biquad LP feedback coefficients (normalised, sign-as-stored for TDF-II).
    pub a1_lp: f64,
    pub a2_lp: f64,
    /// First-order IIR low shelf (direct path): `y[n] = a0_ls·x[n] + a1_ls·x[n-1] + b1_ls·y[n-1]`
    pub a0_ls: f64,
    pub a1_ls: f64,
    pub b1_ls: f64,
}

const _: () = assert!(std::mem::size_of::<Bs2bCoefficients>() == 64);

impl Bs2bCoefficients {
    /// Compute coefficients from cutoff frequency, crossfeed level, Q, and sample rate.
    ///
    /// ## Gd / Ad_h split
    ///
    /// `feed_db` controls the crossed LP path gain: `Gd = -(feed_db × 1.5)`.
    ///
    /// `ad_h_db` controls the direct low shelf independently. In the
    /// original bs2b design these were coupled via `Ad_h = -(feed_db × 0.5)`,
    /// but this function accepts them as separate parameters so each can be
    /// tuned without affecting the other.
    ///
    /// The perceived crossfeed = `|Gd| − |Ad_h|`. The 3:1 default ratio
    /// is empirical, derived from Sergey Umansky's original presets.
    ///
    /// ## Biquad LP design (RBJ Audio EQ Cookbook)
    ///
    /// The crossed path uses a second-order lowpass designed with the bilinear
    /// transform. Given `ω₀ = 2π·fc/fs` and `α = sin(ω₀) / (2·Q)`:
    ///
    /// ```text
    /// b0 =  (1 − cos ω₀) / 2        a0 = 1 + α
    /// b1 =   1 − cos ω₀       →  normalise all by a0
    /// b2 =  (1 − cos ω₀) / 2        a1 = −2·cos ω₀
    ///                                a2 =  1 − α
    /// ```
    ///
    /// All feedforward coefficients are additionally scaled by `g = 10^(Gd/20)`
    /// so the DC gain equals `g` directly, without a per-sample multiply.
    ///
    /// ## Q reference values
    ///
    /// | Q     | Character                          |
    /// |-------|------------------------------------|
    /// | 0.5   | Critically damped (no overshoot)   |
    /// | 0.707 | Butterworth (maximally flat)       |
    /// | 1.0   | Slight resonance at `fc`           |
    /// | >1.0  | Resonance peak — use with care     |
    pub fn compute(fc_hz: f64, feed_db: f64, q: f64, ad_h_db: f64, sample_rate: f64) -> Self {
        let gd = -(feed_db * 1.5);
        let ls_db = ad_h_db;

        let g = 10f64.powf(gd / 20.0);
        let a_ls = 10f64.powf(ls_db / 20.0);
        let g_ls = 1.0 - a_ls;

        // Low shelf cutoff frequency (derived from gain split)
        let gd_ls = 20.0 * g_ls.ln() / 10f64.ln();
        let fc_ls = fc_hz * 2f64.powf((gd - gd_ls) / 12.0);

        // Biquad LP — RBJ Audio EQ Cookbook, bilinear transform
        let w0 = 2.0 * PI * fc_hz / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q);
        let a0_norm = 1.0 + alpha;

        // Feedforward scaled by g → DC gain = g (crossed path attenuation)
        let b0_lp = g * (1.0 - cos_w0) / 2.0 / a0_norm;
        let b1_lp = g * (1.0 - cos_w0) / a0_norm;
        let b2_lp = b0_lp; // b2 == b0 for LP biquad
        let a1_lp = -2.0 * cos_w0 / a0_norm;
        let a2_lp = (1.0 - alpha) / a0_norm;

        // Low shelf — first-order IIR, transition at fc_ls
        let x_ls = (-2.0 * PI * fc_ls / sample_rate).exp();

        Self {
            b0_lp,
            b1_lp,
            b2_lp,
            a1_lp,
            a2_lp,
            a0_ls: 1.0 - g_ls * (1.0 - x_ls),
            a1_ls: -x_ls,
            b1_ls: x_ls,
        }
    }
}

/// Per-channel filter state for the bs2b crossfeed structure.
///
/// Each channel owns the delay memory for both its filters:
/// - The **lowpass** state (`lp_s1`, `lp_s2`) is fed the *opposite* channel's
///   signal and its output is mixed into this channel's output.
/// - The **low shelf** state (`lowshelf_x1`, `lowshelf_y1`) is fed this
///   channel's own signal for the direct path.
///
/// The biquad LP uses **Direct Form II Transposed (TDF-II)**. Compared to
/// Direct Form I, TDF-II requires only 2 state variables instead of 4,
/// and has better numerical precision for fixed-point and near-unity pole radii.
///
/// TDF-II recurrence:
///
/// ```text
/// y[n]     = b0·x[n] + s1
/// s1_new   = b1·x[n] − a1·y[n] + s2
/// s2_new   = b2·x[n] − a2·y[n]
/// ```
///
/// # Size
///
/// 4 × f64 = 32 bytes of data, padded to 64 bytes by `#[repr(align(64))]`.
/// Same rationale as [`Bs2bCoefficients`]: pins to a cache line boundary so
/// state reads and writes during the per-sample loop never straddle two lines.
#[repr(align(64))]
pub struct Bs2bChannel {
    /// TDF-II biquad LP delay line — first state register.
    pub lp_s1: f64,
    /// TDF-II biquad LP delay line — second state register.
    pub lp_s2: f64,
    /// Low shelf one-sample input delay (`x[n-1]`).
    pub lowshelf_x1: f64,
    /// Low shelf one-sample output delay (`y[n-1]`).
    pub lowshelf_y1: f64,
}

const _: () = assert!(std::mem::size_of::<Bs2bChannel>() == 64);

impl Bs2bChannel {
    /// Create a new channel with zeroed filter state.
    pub fn new() -> Self {
        Self {
            lp_s1: 0.0,
            lp_s2: 0.0,
            lowshelf_x1: 0.0,
            lowshelf_y1: 0.0,
        }
    }

    /// Clear all filter state (use on transport stop or reset).
    pub fn reset(&mut self) {
        self.lp_s1 = 0.0;
        self.lp_s2 = 0.0;
        self.lowshelf_x1 = 0.0;
        self.lowshelf_y1 = 0.0;
    }

    /// Second-order biquad lowpass via TDF-II.
    ///
    /// Applied to the opposite channel's signal; output is mixed into this
    /// channel's output to simulate head-shadowing crosstalk.
    ///
    /// ```text
    /// y[n]   = b0·x[n] + s1
    /// s1_new = b1·x[n] − a1·y[n] + s2
    /// s2_new = b2·x[n] − a2·y[n]
    /// ```
    pub fn lowpass(&mut self, x: f64, coeffs: &Bs2bCoefficients) -> f64 {
        let y = coeffs.b0_lp * x + self.lp_s1;
        self.lp_s1 = coeffs.b1_lp * x - coeffs.a1_lp * y + self.lp_s2;
        self.lp_s2 = coeffs.b2_lp * x - coeffs.a2_lp * y;
        y
    }

    /// First-order IIR low shelf: `y[n] = a0_ls·x[n] + a1_ls·x[n-1] + b1_ls·y[n-1]`
    ///
    /// Applied to this channel's own signal (direct path). Attenuates low
    /// frequencies by `ls_db` and passes high frequencies at unity gain,
    /// shaping the direct path to complement the LP crossed path.
    pub fn low_shelf(&mut self, x: f64, coeffs: &Bs2bCoefficients) -> f64 {
        let y = coeffs.a0_ls * x + coeffs.a1_ls * self.lowshelf_x1 + coeffs.b1_ls * self.lowshelf_y1;
        self.lowshelf_x1 = x;
        self.lowshelf_y1 = y;
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
    ///
    /// `q` controls the biquad LP slope sharpness — `0.707` (Butterworth) is the
    /// default. See [`Bs2bCoefficients::compute`] for the full Q reference table.
    pub fn update_coeffs(&mut self, fc_hz: f64, feed_db: f64, q: f64, ad_h_db: f64, sample_rate: f64) {
        self.coeffs = Bs2bCoefficients::compute(fc_hz, feed_db, q, ad_h_db, sample_rate);
    }

    /// Clear filter state on both channels (use on transport stop or reset).
    pub fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
    }

    /// Process one stereo sample with pre-computed ITD delay.
    ///
    /// `sample` is the undelayed signal used for the **direct** (low shelf) path.
    /// `delayed` is the ITD-delayed signal used for the **crossed** (LP) path.
    /// The two are kept separate so the ITD delay only affects the crossfeed,
    /// not the direct path — matching real speaker geometry.
    ///
    /// ```text
    /// outL = low_shelf(sample.0) + lp(delayed.1)
    /// outR = low_shelf(sample.1) + lp(delayed.0)
    /// ```
    pub fn process_with_itd(&mut self, sample: (f64, f64), delayed: (f64, f64)) -> (f64, f64) {
        let lowshelf_l = self.left.low_shelf(sample.0, &self.coeffs);
        let lowshelf_r = self.right.low_shelf(sample.1, &self.coeffs);

        let lowpass_l = self.left.lowpass(delayed.0, &self.coeffs);
        let lowpass_r = self.right.lowpass(delayed.1, &self.coeffs);

        (lowshelf_l + lowpass_r, lowshelf_r + lowpass_l)
    }
}
