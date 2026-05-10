use crate::{clap::*, parameters::any::PARAMS_COUNT, version::CLAP_VERSION_INIT};
use std::ffi::{CStr, CString, c_char, c_void};

// Param indices: 0=Cutoff, 1=XFeed, 2=Angle, 3=Center, 4=Gain, 5=LRSwap, 6=Solo, 7=Phase, 8=CalibrationMode
// bs2b canonical presets from https://bs2b.sourceforge.net/
// Angle=30°, Center=0.0 dB (neutral), Gain=0.0 dB, LRSwap=Off, Solo=Off, Phase=Off, CalMode=Off
pub struct PresetDef {
    pub name: &'static str,
    pub load_key: &'static str,
    pub values: [f64; PARAMS_COUNT],
}

pub const PRESETS: &[PresetDef] = &[
    PresetDef {
        name: "Default",
        load_key: "default",
        //          Cutoff  XFeed  Angle  Center   Gain  LRSwap  Solo  Phase  CalMode
        values: [822.0, 6.2, 30.0, -0.27, 0.0, 0.0, 1.0, 1.0, 0.0],
    },
    PresetDef {
        name: "Chu Moy",
        load_key: "chu-moy",
        //          Cutoff  XFeed  Angle  Center   Gain  LRSwap  Solo  Phase  CalMode
        values: [700.0, 6.0, 30.0, -0.27, 0.0, 0.0, 1.0, 1.0, 0.0],
    },
    PresetDef {
        name: "Jan Meier",
        load_key: "jan-meier",
        //          Cutoff  XFeed  Angle  Center   Gain  LRSwap  Solo  Phase  CalMode
        values: [650.0, 9.5, 30.0, -0.27, 0.0, 0.0, 1.0, 1.0, 0.0],
    },
];

/// Returns the parameter values for a preset identified by load_key.
pub fn preset_values(load_key: &str) -> Option<[f64; PARAMS_COUNT]> {
    PRESETS.iter().find(|p| p.load_key == load_key).map(|p| p.values)
}

static PROVIDER_DESCRIPTOR: clap_preset_discovery_provider_descriptor_t = clap_preset_discovery_provider_descriptor {
    clap_version: CLAP_VERSION_INIT,
    id: c"com.enguidanosweb.TakeMyHeadphones.presets".as_ptr(),
    name: c"TakeMyPhones Presets".as_ptr(),
    vendor: c"enguidanosweb".as_ptr(),
};

unsafe impl Sync for clap_preset_discovery_provider_descriptor_t {}

pub static PRESET_DISCOVERY_FACTORY: clap_preset_discovery_factory_t = clap_preset_discovery_factory {
    count: Some(factory_count),
    get_descriptor: Some(factory_get_descriptor),
    create: Some(factory_create),
};

unsafe impl Sync for clap_preset_discovery_factory_t {}

unsafe extern "C" fn factory_count(_factory: *const clap_preset_discovery_factory_t) -> u32 {
    1
}

unsafe extern "C" fn factory_get_descriptor(
    _factory: *const clap_preset_discovery_factory_t,
    index: u32,
) -> *const clap_preset_discovery_provider_descriptor_t {
    if index == 0 { &PROVIDER_DESCRIPTOR } else { std::ptr::null() }
}

unsafe extern "C" fn factory_create(
    _factory: *const clap_preset_discovery_factory_t,
    indexer: *const clap_preset_discovery_indexer_t,
    provider_id: *const c_char,
) -> *const clap_preset_discovery_provider_t {
    let id = unsafe { CStr::from_ptr(provider_id) };
    if id != unsafe { CStr::from_ptr(PROVIDER_DESCRIPTOR.id) } {
        return std::ptr::null();
    }

    let provider = Box::new(clap_preset_discovery_provider {
        desc: &PROVIDER_DESCRIPTOR,
        provider_data: indexer as *mut c_void,
        init: Some(provider_init),
        destroy: Some(provider_destroy),
        get_metadata: Some(provider_get_metadata),
        get_extension: Some(provider_get_extension),
    });

    Box::into_raw(provider)
}

unsafe extern "C" fn provider_init(provider: *const clap_preset_discovery_provider_t) -> bool {
    let provider_ref = unsafe { provider.as_ref_unchecked() };
    let indexer = provider_ref.provider_data as *const clap_preset_discovery_indexer_t;
    let indexer_ref = unsafe { indexer.as_ref_unchecked() };

    let location = clap_preset_discovery_location {
        flags: clap_preset_discovery_flags_CLAP_PRESET_DISCOVERY_IS_FACTORY_CONTENT as u32,
        name: c"Built-in".as_ptr(),
        kind: clap_preset_discovery_location_kind_CLAP_PRESET_DISCOVERY_LOCATION_PLUGIN as u32,
        location: std::ptr::null(),
    };

    if let Some(declare_location) = indexer_ref.declare_location {
        unsafe { declare_location(indexer, &location) };
    }

    true
}

unsafe extern "C" fn provider_destroy(provider: *const clap_preset_discovery_provider_t) {
    drop(unsafe { Box::from_raw(provider as *mut clap_preset_discovery_provider_t) });
}

unsafe extern "C" fn provider_get_metadata(
    _provider: *const clap_preset_discovery_provider_t,
    location_kind: u32,
    _location: *const c_char,
    metadata_receiver: *const clap_preset_discovery_metadata_receiver_t,
) -> bool {
    if location_kind != clap_preset_discovery_location_kind_CLAP_PRESET_DISCOVERY_LOCATION_PLUGIN as u32 {
        return false;
    }

    let receiver_ref = unsafe { metadata_receiver.as_ref_unchecked() };

    let plugin_id = clap_universal_plugin_id {
        abi: c"clap".as_ptr(),
        id: c"com.enguidanosweb.TakeMyHeadphones".as_ptr(),
    };

    for preset in PRESETS {
        let name = CString::new(preset.name).unwrap();
        let key = CString::new(preset.load_key).unwrap();

        if let Some(begin) = receiver_ref.begin_preset {
            if !unsafe { begin(metadata_receiver, name.as_ptr(), key.as_ptr()) } {
                return false;
            }
        }

        if let Some(add_plugin_id) = receiver_ref.add_plugin_id {
            unsafe { add_plugin_id(metadata_receiver, &plugin_id) };
        }

        if let Some(set_flags) = receiver_ref.set_flags {
            unsafe {
                set_flags(
                    metadata_receiver,
                    clap_preset_discovery_flags_CLAP_PRESET_DISCOVERY_IS_FACTORY_CONTENT as u32,
                )
            };
        }
    }

    true
}

unsafe extern "C" fn provider_get_extension(
    _provider: *const clap_preset_discovery_provider_t,
    _extension_id: *const c_char,
) -> *const c_void {
    std::ptr::null()
}
