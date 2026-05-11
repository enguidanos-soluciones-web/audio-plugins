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
/// ITD is the microsecond-scale difference in arrival time between the two ears
/// when a sound source is off-centre. It is the dominant localisation cue below
/// ~1.5 kHz — the same frequency range where bs2b crossfeed is most active.
///
/// This struct applies the same fractional delay to both L and R channels
/// independently via a circular buffer with linear interpolation. The delayed
/// output is used for the **crossed path** in bs2b (`process_with_itd`), while
/// the undelayed signal is used for the **direct path**. The asymmetry between
/// the two paths is what externalises the stereo image.
///
/// Angle → delay mapping (SPL Phonitor 3 / JSFX reference):
///
/// | Angle | Delay  |
/// |-------|--------|
/// | 0°    | 0 μs   |
/// | 75°   | 635 μs |
///
/// Intermediate values are linearly interpolated by [`ItdDelay::angle_to_delay_samples`].
///
/// # Size
///
/// The circular buffers are `Box<[f64; CAPACITY]>` — fixed-size arrays allocated
/// on the heap. This has three advantages over `Vec<f64>`:
/// - No `len`/`capacity` overhead in the `Vec` header.
/// - The `capacity` field is eliminated from the struct; `CAPACITY` is a
///   compile-time constant, so `% CAPACITY` can be reduced to `& (CAPACITY - 1)`
///   by the compiler (512 = 2⁹).
/// - Guaranteed heap allocation regardless of where `ItdDelay` is instantiated,
///   avoiding accidental 8 KB stack frames.
///
/// The struct itself is 2 × pointer + `write_ptr` = 24 bytes, padded to 64 bytes
/// by `#[repr(align(64))]`. All three fields are hot on every sample.
#[repr(align(64))]
pub struct ItdDelay {
    buf_l: Box<[f64; Self::CAPACITY]>,
    buf_r: Box<[f64; Self::CAPACITY]>,
    write_ptr: usize,
}

const _: () = assert!(std::mem::size_of::<ItdDelay>() == 64);

impl ItdDelay {
    /// Circular buffer capacity in samples.
    ///
    /// Maximum delay at 192 kHz: 635 μs × 192 000 ≈ 122 samples. 512 gives
    /// safe headroom for any realistic sample rate and angle combination.
    /// 512 = 2⁹, so `% CAPACITY` compiles to a bitwise `& 511`.
    const CAPACITY: usize = 512;

    /// Create a new delay line with zeroed buffers allocated on the heap.
    pub fn new() -> Self {
        Self {
            buf_l: Box::new([0.0; Self::CAPACITY]),
            buf_r: Box::new([0.0; Self::CAPACITY]),
            write_ptr: 0,
        }
    }

    /// Clear the delay buffers and reset the write pointer.
    pub fn reset(&mut self) {
        self.buf_l.fill(0.0);
        self.buf_r.fill(0.0);
        self.write_ptr = 0;
    }

    /// Convert angle (degrees, 0–75) and sample rate to fractional delay in samples.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(ItdDelay::angle_to_delay_samples(0.0, 44100.0), 0.0);
    /// assert!((ItdDelay::angle_to_delay_samples(75.0, 44100.0) - 28.0).abs() < 1.0); // 635 μs × 44100 ≈ 28 samples
    /// ```
    pub fn angle_to_delay_samples(angle_deg: f64, sample_rate: f64) -> f64 {
        const MAX_DELAY_US: f64 = 635.0;
        let delay_us = (angle_deg / 75.0).clamp(0.0, 1.0) * MAX_DELAY_US;
        delay_us * 1e-6 * sample_rate
    }

    /// Write the current sample pair to the circular buffer, then read back with
    /// fractional delay using linear interpolation between adjacent samples.
    ///
    /// Returns `(delayed_l, delayed_r)`. At `delay_samples = 0.0` the output
    /// equals the input (zero-delay passthrough).
    pub fn process(&mut self, sample: (f64, f64), delay_samples: f64) -> (f64, f64) {
        // Write current sample
        self.buf_l[self.write_ptr] = sample.0;
        self.buf_r[self.write_ptr] = sample.1;

        // Fractional read: linear interpolation between floor and floor+1 samples ago
        let delay_floor = delay_samples.floor() as usize;
        let frac = delay_samples - delay_samples.floor();

        let idx0 = (self.write_ptr + Self::CAPACITY - delay_floor) % Self::CAPACITY;
        let idx1 = (self.write_ptr + Self::CAPACITY - delay_floor - 1) % Self::CAPACITY;

        let out_l = self.buf_l[idx0] * (1.0 - frac) + self.buf_l[idx1] * frac;
        let out_r = self.buf_r[idx0] * (1.0 - frac) + self.buf_r[idx1] * frac;

        // Advance write pointer
        self.write_ptr = (self.write_ptr + 1) % Self::CAPACITY;

        (out_l, out_r)
    }
}
