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
/// Signal chain per channel:
///   outL = highboost(inL) + lp(inR)
///   outR = highboost(inR) + lp(inL)
///
/// Reference: https://bs2b.sourceforge.net/
/// Canonical presets:
///   Default:   Fc=700 Hz, feed=4.5 dB  (Gd=-6.75, Ad_h=-2.25)
///   Chu Moy:   Fc=700 Hz, feed=6.0 dB  (Gd=-8.0,  Ad_h=-2.0 )
///   Jan Meier: Fc=650 Hz, feed=9.5 dB  (Gd=-10.917, Ad_h=-1.417)
#[derive(Clone, Copy, Default)]
pub struct Bs2bCoefficients {
    /// One-pole LP (crossed path): y[n] = a0*x[n] + b1*y[n-1]
    pub a0: f64,
    pub b1: f64,
    /// First-order IIR highboost (direct path): y[n] = a0_h*x[n] + a1_h*x[n-1] + b1_h*y[n-1]
    pub a0_h: f64,
    pub a1_h: f64,
    pub b1_h: f64,
}

impl Bs2bCoefficients {
    /// Compute from cutoff (Hz), crossfeed level (dB), sample rate.
    ///
    /// Gd/Ad_h split uses the Default preset ratio:
    ///   Gd = -(feed * 1.5)   →  -6.75 dB at feed=4.5
    ///   Ad_h = -(feed * 0.5) →  -2.25 dB at feed=4.5
    /// This matches the bs2b Default preset exactly.
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

/// Per-channel filter state.
/// Each channel holds state for both its LP filter (applied to this channel's signal
/// and mixed into the opposite output) and its highboost filter (applied to this
/// channel's signal for its own output).
pub struct Bs2bChannel {
    pub lowpass_y1: f64,
    pub highboost_x1: f64,
    pub highboost_y1: f64,
}

impl Bs2bChannel {
    pub fn new() -> Self {
        Self {
            lowpass_y1: 0.0,
            highboost_x1: 0.0,
            highboost_y1: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.lowpass_y1 = 0.0;
        self.highboost_x1 = 0.0;
        self.highboost_y1 = 0.0;
    }

    /// One-pole LP: y[n] = a0*x[n] + b1*y[n-1]
    pub fn lowpass(&mut self, x: f64, coeffs: &Bs2bCoefficients) -> f64 {
        let y = coeffs.a0 * x + coeffs.b1 * self.lowpass_y1;
        self.lowpass_y1 = y;
        y
    }

    /// Highboost: y[n] = a0_h*x[n] + a1_h*x[n-1] + b1_h*y[n-1]
    pub fn highboost(&mut self, x: f64, coeffs: &Bs2bCoefficients) -> f64 {
        let y = coeffs.a0_h * x + coeffs.a1_h * self.highboost_x1 + coeffs.b1_h * self.highboost_y1;
        self.highboost_x1 = x;
        self.highboost_y1 = y;
        y
    }
}

pub struct Bs2b {
    pub left: Bs2bChannel,
    pub right: Bs2bChannel,
    pub coeffs: Bs2bCoefficients,
}

impl Bs2b {
    pub fn new() -> Self {
        Self {
            left: Bs2bChannel::new(),
            right: Bs2bChannel::new(),
            coeffs: Bs2bCoefficients::default(),
        }
    }

    pub fn update_coeffs(&mut self, fc_hz: f64, feed_db: f64, sample_rate: f64) {
        self.coeffs = Bs2bCoefficients::compute(fc_hz, feed_db, sample_rate);
    }

    pub fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
    }

    /// Process with ITD: direct path uses `in_l`/`in_r`, crossed path uses pre-delayed signals.
    ///   outL = highboost(in_l) + lp(in_r_delayed)
    ///   outR = highboost(in_r) + lp(in_l_delayed)
    pub fn process_with_itd(&mut self, sample: (f64, f64), delayed: (f64, f64)) -> (f64, f64) {
        let highboost_l = self.left.highboost(sample.0, &self.coeffs);
        let highboost_r = self.right.highboost(sample.1, &self.coeffs);

        let lowpass_l = self.left.lowpass(delayed.0, &self.coeffs);
        let lowpass_r = self.right.lowpass(delayed.1, &self.coeffs);

        (highboost_l + lowpass_r, highboost_r + lowpass_l)
    }
}
