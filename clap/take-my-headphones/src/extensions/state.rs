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

use crate::{clap::*, parameters::any::PARAMS_COUNT, plugin::Plugin, state::ParamChange};
use std::{os::raw::c_void, sync::Arc};

pub static STATE_EXT: clap_plugin_state_t = clap_plugin_state {
    save: Some(save),
    load: Some(load),
};

// [main-thread]
pub extern "C" fn save(plugin: *const clap_plugin_t, stream: *const clap_ostream_t) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *const Plugin).as_ref_unchecked() };

    let main = plugin_ref.main_thread.as_ref().unwrap();
    main.assert_main_thread();

    let stream_ref = unsafe { stream.as_ref_unchecked() };
    let Some(write) = stream_ref.write else {
        return false;
    };

    let snapshot = main.param_snapshot.load();

    let write_all = |buf: &[u8]| -> bool {
        let mut offset = 0;
        while offset < buf.len() {
            let n = unsafe { write(stream, buf.as_ptr().add(offset) as *const c_void, (buf.len() - offset) as u64) };
            if n <= 0 {
                return false;
            }
            offset += n as usize;
        }
        true
    };

    let values_bytes =
        unsafe { std::slice::from_raw_parts(snapshot.values.as_ptr() as *const u8, std::mem::size_of::<f64>() * PARAMS_COUNT) };

    if !write_all(values_bytes) {
        return false;
    }

    true
}

// [main-thread]
pub extern "C" fn load(plugin: *const clap_plugin_t, stream: *const clap_istream_t) -> bool {
    let plugin_ref = unsafe { ((*plugin).plugin_data as *mut Plugin).as_mut_unchecked() };

    let main_thread = plugin_ref.main_thread.as_mut().unwrap();
    main_thread.assert_main_thread();

    let stream_ref = unsafe { stream.as_ref_unchecked() };
    let Some(read) = stream_ref.read else {
        return false;
    };

    let read_all = |buf: &mut [u8]| -> bool {
        let mut offset = 0;
        while offset < buf.len() {
            let n = unsafe { read(stream, buf.as_mut_ptr().add(offset) as *mut c_void, (buf.len() - offset) as u64) };
            if n <= 0 {
                return false;
            }
            offset += n as usize;
        }
        true
    };

    let mut new_snapshot = *main_thread.param_snapshot.load_full();
    let values_bytes = unsafe {
        std::slice::from_raw_parts_mut(
            new_snapshot.values.as_mut_ptr() as *mut u8,
            std::mem::size_of::<f64>() * PARAMS_COUNT,
        )
    };
    if !read_all(values_bytes) {
        return false;
    }

    main_thread.param_snapshot.store(Arc::new(new_snapshot));
    for id in 0..PARAMS_COUNT {
        let _ = main_thread.param_changes.push(ParamChange::Value {
            id,
            value: new_snapshot.values[id],
        });
    }

    true
}
