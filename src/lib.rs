use obs_sys::*;
use obs_text::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;

// OBS constants
const OBS_SOURCE_VIDEO: u32 = 1;
const OBS_SOURCE_TYPE_FILTER: u32 = 2;
const OBS_SOURCE_ASYNC: u32 = 4;

// OBS text type constants
const OBS_TEXT_DEFAULT: u32 = 0;
const OBS_TEXT_PASSWORD: u32 = 1;
const OBS_TEXT_MULTILINE: u32 = 2;

// Basic OBS structs for FFI
#[repr(C)]
struct obs_source_info {
    id: *const c_char,
    type_: u32,
    output_flags: u32,
    get_name: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    create: Option<extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void>,
    destroy: Option<extern "C" fn(*mut c_void)>,
    get_properties: Option<extern "C" fn(*mut c_void) -> *mut obs_properties_t>,
    update: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    video_render: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    _padding: [u64; 20],
}

// For testing only - OBS would provide this function
#[no_mangle]
pub extern "C" fn obs_register_source(_info: *mut obs_source_info) {
    // This is just a stub for testing
    println!("Mock: obs_register_source called");
}

// Plugin data structure
struct TarkovPriceOverlayData {
    settings: Mutex<Settings>,
}

#[derive(Default)]
struct Settings {
    api_key: String,
    refresh_interval: u32,
    font_size: u32,
    position_x: u32,
    position_y: u32,
}

// Plugin API functions
extern "C" fn tarkov_price_overlay_get_name(_data: *mut c_void) -> *const c_char {
    CString::new("Tarkov Item Price Overlay")
        .unwrap()
        .into_raw()
}

extern "C" fn tarkov_price_overlay_create(
    settings: *mut c_void,
    _source: *mut c_void,
) -> *mut c_void {
    let data = Box::new(TarkovPriceOverlayData {
        settings: Mutex::new(Settings::default()),
    });
    Box::into_raw(data) as *mut c_void
}

extern "C" fn tarkov_price_overlay_destroy(data: *mut c_void) {
    if !data.is_null() {
        unsafe {
            drop(Box::from_raw(data as *mut TarkovPriceOverlayData));
        }
    }
}

#[no_mangle]
pub extern "C" fn get_properties(_data: *mut c_void) -> *mut obs_properties_t {
    unsafe {
        let props = obs_properties_create();

        obs_properties_add_text(
            props,
            CString::new("api_key").unwrap().as_ptr(),
            CString::new("Tarkov Market API Key").unwrap().as_ptr(),
            OBS_TEXT_DEFAULT,
        );

        obs_properties_add_int(
            props,
            CString::new("update_interval").unwrap().as_ptr(),
            CString::new("Update Interval (seconds)").unwrap().as_ptr(),
            1,
            3600,
            1,
        );

        obs_properties_add_int(
            props,
            CString::new("font_size").unwrap().as_ptr(),
            CString::new("Font Size").unwrap().as_ptr(),
            8,
            72,
            1,
        );

        props
    }
}

#[no_mangle]
pub extern "C" fn obs_module_load() -> bool {
    // Create and register our source
    let mut info = obs_source_info {
        id: CString::new("tarkov_price_overlay").unwrap().into_raw(),
        type_: OBS_SOURCE_VIDEO,
        output_flags: OBS_SOURCE_VIDEO | OBS_SOURCE_ASYNC,
        get_name: Some(tarkov_price_overlay_get_name),
        create: Some(tarkov_price_overlay_create),
        destroy: Some(tarkov_price_overlay_destroy),
        get_properties: Some(get_properties),
        update: None,
        video_render: None,
        _padding: [0; 20],
    };

    unsafe {
        obs_register_source(&mut info);
    }

    eprintln!("Tarkov Price Overlay plugin loaded successfully!");
    true
}

#[no_mangle]
pub extern "C" fn obs_module_ver() -> u32 {
    0x010000
}

#[no_mangle]
pub extern "C" fn obs_module_set_pointer(_ptr: *mut c_void) {}

#[no_mangle]
pub extern "C" fn obs_module_name() -> *const c_char {
    CString::new("Tarkov Item Price Overlay")
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn obs_module_description() -> *const c_char {
    CString::new("Shows item prices from Tarkov Market API")
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn obs_module_author() -> *const c_char {
    CString::new("Tarkov Price Overlay Developers")
        .unwrap()
        .into_raw()
}

// Support functions for OBS
#[no_mangle]
pub extern "C" fn obs_module_free_locale() {}

#[no_mangle]
pub extern "C" fn obs_module_set_locale(_locale: *const c_char) {}

#[no_mangle]
pub extern "C" fn obs_module_unload() {
    // Cleanup if needed
}
