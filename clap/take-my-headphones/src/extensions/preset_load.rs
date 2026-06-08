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

use crate::{clap::*, parameters::any::PARAMS_COUNT, plugin::Plugin, preset_factory::preset_values, state::ParamChange};
use std::{ffi::CStr, sync::Arc};

pub static PRESET_LOAD_EXT: clap_plugin_preset_load_t = clap_plugin_preset_load {
    from_location: Some(from_location),
};

// [main-thread]
unsafe extern "C" fn from_location(
    plugin: *const clap_plugin_t,
    location_kind: u32,
    _location: *const std::ffi::c_char,
    load_key: *const std::ffi::c_char,
) -> bool {
    if location_kind != clap_preset_discovery_location_kind_CLAP_PRESET_DISCOVERY_LOCATION_PLUGIN as u32 {
        return false;
    }

    let key = unsafe { CStr::from_ptr(load_key) }.to_str().unwrap_or("");
    let Some(values) = preset_values(key) else {
        return false;
    };

    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };
    let main = plugin_ref.main_thread.as_mut().unwrap();
    main.assert_main_thread();

    let mut new_snapshot = *main.param_snapshot.load_full();
    new_snapshot.values = values;
    main.param_snapshot.store(Arc::new(new_snapshot));

    for (id, _) in values.iter().enumerate().take(PARAMS_COUNT) {
        let _ = main.param_changes.push(ParamChange::Value { id, value: values[id] });
    }

    // Notify the host that all parameter values changed so it can refresh its UI.
    let host = plugin_ref.host;
    let host_ref = unsafe { host.as_ref_unchecked() };
    if let Some(get_extension) = host_ref.get_extension {
        let ext = unsafe { get_extension(host, CLAP_EXT_PARAMS.as_ptr() as *const std::ffi::c_char) };
        if !ext.is_null() {
            let host_params = unsafe { (ext as *const clap_host_params_t).as_ref_unchecked() };
            if let Some(rescan) = host_params.rescan {
                unsafe { rescan(host, CLAP_PARAM_RESCAN_VALUES as u32) };
            }
        }
    }

    true
}
