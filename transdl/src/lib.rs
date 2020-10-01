#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod ll;

pub mod event;
pub mod timer;
pub mod ttf;
pub mod video;

fn get_error() -> Option<String> {
    use std::ffi::CStr;

    let c = unsafe { ll::SDL_GetError() };
    if c.is_null() {
        None
    } else {
        let cstr = unsafe { CStr::from_ptr(c) };
        Some(cstr.to_string_lossy().to_string())
    }
}
