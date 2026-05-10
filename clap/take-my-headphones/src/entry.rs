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
