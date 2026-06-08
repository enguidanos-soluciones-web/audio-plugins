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

pub const TARGET_LOUDNESS_DBFS: f64 = -18.0;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("nam.h");

        type NamDsp;

        fn load(json: &str) -> UniquePtr<NamDsp>;
        fn process(dsp: Pin<&mut NamDsp>, input: &[f64], output: &mut [f64]);
        fn reset(dsp: Pin<&mut NamDsp>, sample_rate: f64, max_block_size: i32);
        fn reset_and_prewarm(dsp: Pin<&mut NamDsp>, sample_rate: f64, max_block_size: i32);
        fn get_sample_rate_from_nam_file(json: &str) -> f64;
        fn has_loudness(dsp: &NamDsp) -> bool;
        fn get_loudness(dsp: &NamDsp) -> f64;
    }
}
