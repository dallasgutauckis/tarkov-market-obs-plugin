use std::ffi::CString;
use std::os::raw::{c_char, c_void};

// OBS constants
const OBS_SOURCE_VIDEO: u32 = 1;
const OBS_SOURCE_TYPE_FILTER: u32 = 2;
const OBS_SOURCE_ASYNC: u32 = 4;

// Basic OBS structs for FFI
#[repr(C)]
struct obs_source_info {
    id: *const c_char,
    type_: u32,
    output_flags: u32,
    get_name: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    create: Option<extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void>,
    destroy: Option<extern "C" fn(*mut c_void)>,
    get_properties: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    update: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    video_render: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    _padding: [u64; 20], // Add padding to match the full struct size
}

// For testing only - OBS would provide this function
#[no_mangle]
pub extern "C" fn obs_register_source(_info: *mut obs_source_info) {
    // This is just a stub for testing
    println!("Mock: obs_register_source called");
}

// Plugin data structure - empty for our simple mock
struct TarkovPriceOverlayData {}

// Plugin API functions
extern "C" fn tarkov_price_overlay_get_name(_data: *mut c_void) -> *const c_char {
    CString::new("Tarkov Item Price Overlay").unwrap().into_raw()
}

extern "C" fn tarkov_price_overlay_create(_settings: *mut c_void, _source: *mut c_void) -> *mut c_void {
    let data = Box::new(TarkovPriceOverlayData {});
    Box::into_raw(data) as *mut c_void
}

extern "C" fn tarkov_price_overlay_destroy(data: *mut c_void) {
    if !data.is_null() {
        unsafe {
            // We need to drop the box to avoid memory leaks
            drop(Box::from_raw(data as *mut TarkovPriceOverlayData));
        }
    }
}

extern "C" fn tarkov_price_overlay_get_properties(_data: *mut c_void) -> *mut c_void {
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn obs_module_load() -> bool {
    let mut info = obs_source_info {
        id: CString::new("tarkov-price-overlay").unwrap().into_raw(),
        type_: OBS_SOURCE_TYPE_FILTER,
        output_flags: OBS_SOURCE_VIDEO | OBS_SOURCE_ASYNC,
        get_name: Some(tarkov_price_overlay_get_name),
        create: Some(tarkov_price_overlay_create),
        destroy: Some(tarkov_price_overlay_destroy),
        get_properties: Some(tarkov_price_overlay_get_properties),
        update: None,
        video_render: None,
        _padding: [0; 20],
    };
    
    unsafe {
        obs_register_source(&mut info);
    }
    
    // Print a message to show the plugin is loading
    eprintln!("Tarkov Price Overlay plugin loaded successfully!");
    true
}

#[no_mangle]
pub extern "C" fn obs_module_ver() -> u32 {
    // Version in the format MAJOR * 10000 + MINOR * 100 + PATCH
    10000 + 1 * 100 + 0 // 1.01.0
}

// Required by OBS
#[no_mangle]
pub extern "C" fn obs_module_set_pointer(_ptr: *mut c_void) {}

#[no_mangle]
pub extern "C" fn obs_module_name() -> *const c_char {
    CString::new("Tarkov Item Price Overlay").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn obs_module_description() -> *const c_char {
    CString::new("Shows item prices from Tarkov Market API").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn obs_module_author() -> *const c_char {
    CString::new("Tarkov Price Overlay Developers").unwrap().into_raw()
}

// Support functions for OBS
#[no_mangle]
pub extern "C" fn obs_module_free_locale() {}

#[no_mangle]
pub extern "C" fn obs_module_set_locale(_locale: *const c_char) {}

#[no_mangle]
pub extern "C" fn obs_module_unload() {
    eprintln!("Tarkov Price Overlay plugin unloaded!");
}
