use std::ffi::c_char;

pub fn copy_cstr(dst: &mut [c_char], src: &[u8]) {
    let len = src.len().min(dst.len() - 1);
    for (d, s) in dst[..len].iter_mut().zip(src[..len].iter()) {
        *d = *s as c_char;
    }
    dst[len] = 0; // null terminator
}
