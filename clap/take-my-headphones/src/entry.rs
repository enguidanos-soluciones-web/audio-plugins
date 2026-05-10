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

use crate::{clap::*, factory::PLUGIN_FACTORY, preset_factory::PRESET_DISCOVERY_FACTORY, version::CLAP_VERSION_INIT};
use std::ffi::{CStr, c_char, c_void};

#[unsafe(no_mangle)]
pub static clap_entry: clap_plugin_entry_t = clap_plugin_entry {
    clap_version: CLAP_VERSION_INIT,
    init: Some(entry_init),
    deinit: Some(entry_deinit),
    get_factory: Some(entry_get_factory),
};

unsafe extern "C" fn entry_init(_plugin_path: *const c_char) -> bool {
    true
}

unsafe extern "C" fn entry_deinit() {}

unsafe extern "C" fn entry_get_factory(factory_id: *const c_char) -> *const c_void {
    let id = unsafe { CStr::from_ptr(factory_id) };

    if id == unsafe { CStr::from_ptr(CLAP_PLUGIN_FACTORY_ID.as_ptr() as *const c_char) } {
        return &PLUGIN_FACTORY as *const _ as *const c_void;
    }
    if id == unsafe { CStr::from_ptr(CLAP_PRESET_DISCOVERY_FACTORY_ID.as_ptr() as *const c_char) }
        || id == unsafe { CStr::from_ptr(CLAP_PRESET_DISCOVERY_FACTORY_ID_COMPAT.as_ptr() as *const c_char) }
    {
        return &PRESET_DISCOVERY_FACTORY as *const _ as *const c_void;
    }

    std::ptr::null()
}
