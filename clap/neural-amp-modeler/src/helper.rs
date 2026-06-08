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

use std::ffi::c_char;

/// Selects the dB scale and converts dB values to linear multipliers.
///
/// dB always means `factor * log10(x / x_ref)`. The factor differs by domain:
/// - `Amplitude` (voltage, pressure, PCM samples): factor = **20**
/// - `Power` (watts, intensity): factor = **10**
///
/// The two scales are consistent: a 6 dB amplitude gain equals a 6 dB power gain
/// because power is proportional to amplitude squared (`P ∝ A²`), and
/// `20·log10(A) = 10·log10(A²)`.
pub enum DecibelConversion {
    /// For amplitude quantities: `dB = 20·log10(A)`, inverse `A = 10^(dB/20)`.
    Amplitude,
    /// For power quantities: `dB = 10·log10(P)`, inverse `P = 10^(dB/10)`.
    #[allow(dead_code)]
    Power,
}

impl DecibelConversion {
    /// Convert a dB value to a linear multiplier.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(DecibelConversion::Amplitude.to_linear(0.0), 1.0);
    /// assert!((DecibelConversion::Amplitude.to_linear(6.0) - 2.0).abs() < 0.01);
    /// assert!((DecibelConversion::Amplitude.to_linear(-6.0) - 0.5).abs() < 0.01);
    /// ```
    pub fn to_linear(&self, db: f64) -> f64 {
        let factor = match self {
            Self::Amplitude => 20.0,
            Self::Power => 10.0,
        };
        f64::powf(10.0, db / factor)
    }
}

pub fn copy_cstr(dst: &mut [c_char], src: &[u8]) {
    let len = src.len().min(dst.len() - 1);
    for (d, s) in dst[..len].iter_mut().zip(src[..len].iter()) {
        *d = *s as c_char;
    }
    dst[len] = 0; // null terminator
}
