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

use crate::{clap::*, version::CLAP_VERSION_INIT};

unsafe impl Sync for clap_plugin_descriptor_t {}

struct FeatureList([*const c_char; 4]);

unsafe impl Sync for FeatureList {}

static PLUGIN_FEATURES: FeatureList = FeatureList([
    CLAP_PLUGIN_FEATURE_STEREO.as_ptr() as *const c_char,
    CLAP_PLUGIN_FEATURE_AUDIO_EFFECT.as_ptr() as *const c_char,
    CLAP_PLUGIN_FEATURE_UTILITY.as_ptr() as *const c_char,
    std::ptr::null(),
]);

pub static PLUGIN_DESCRIPTOR: clap_plugin_descriptor_t = clap_plugin_descriptor {
    clap_version: CLAP_VERSION_INIT,
    id: c"com.enguidanosweb.TakeMyHeadphones".as_ptr(),
    name: c"Take My Headphones".as_ptr(),
    vendor: c"enguidanosweb".as_ptr(),
    url: c"https://enguidanosweb.com".as_ptr(),
    manual_url: c"https://enguidanosweb.com".as_ptr(),
    support_url: c"https://enguidanosweb.com".as_ptr(),
    version: c"0.0.1".as_ptr(),
    description: c"Functional emulation of the Matrix section of the SPL Phonitor 3".as_ptr(),
    features: PLUGIN_FEATURES.0.as_ptr(),
};
