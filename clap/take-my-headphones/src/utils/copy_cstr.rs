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

/// Copy a UTF-8 byte slice into a null-terminated C string buffer.
///
/// Writes at most `dst.len() - 1` bytes from `src`, then appends a null terminator.
/// Always null-terminates `dst`, even if `src` is longer than the buffer.
///
/// # Examples
///
/// ```
/// let mut buf = [0i8; 8];
/// copy_cstr(&mut buf, b"hello");
/// assert_eq!(buf[5], 0); // null terminator
/// ```
pub fn copy_cstr(dst: &mut [c_char], src: &[u8]) {
    let len = src.len().min(dst.len() - 1);
    for (d, s) in dst[..len].iter_mut().zip(src[..len].iter()) {
        *d = *s as c_char;
    }
    dst[len] = 0; // null terminator
}
