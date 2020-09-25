use super::geometry::Geometry;
use super::texture::Texture;
use super::tilecache::TileCache;
use crate::{
    FONT_ASCII_LOWERCASE, FONT_ASCII_UPPERCASE, FONT_HEIGHT,
    FONT_QUESTIONMARK, FONT_WIDTH,
};

fn print_letter(
    target: &mut Texture,
    geometry: Geometry,
    tilecache: &TileCache,
    letter: char,
) {
    let tilenr = match letter {
        c if c >= ' ' && c <= 'Z' => {
            c as usize - ' ' as usize + FONT_ASCII_UPPERCASE
        }
        c if c >= 'a' && c <= 'z' => {
            c as usize - 'a' as usize + FONT_ASCII_LOWERCASE
        }
        _ => FONT_QUESTIONMARK,
    } as usize;
    tilecache.get_tile(tilenr).unwrap().clone_to_texture(
        None,
        target,
        Some(geometry),
    );
}

pub fn print(
    target: &mut Texture,
    geometry: Geometry,
    tilecache: &TileCache,
    text: &str,
) {
    let mut dstrect = geometry.clone();
    dstrect.w = FONT_WIDTH as u16;
    dstrect.h = FONT_HEIGHT as u16;

    for c in text.chars() {
        if c == '\n' {
            dstrect.x = geometry.x;
            dstrect.y += FONT_HEIGHT as i16;
        } else {
            print_letter(target, dstrect.clone(), tilecache, c);
            dstrect.x += FONT_WIDTH as i16;
        }
    }
}

pub mod ffi {
    use super::super::geometry::ffi::FnGeometry;
    use super::super::texture::ffi::FnTexture;
    use super::super::tilecache::ffi::FnTileCache;
    use libc::c_char;
    use std::ffi::CStr;

    #[no_mangle]
    pub extern "C" fn fn_text_print(
        target: *mut FnTexture,
        geometry: FnGeometry,
        tilecache: *const FnTileCache,
        text: *const c_char,
    ) {
        assert!(!target.is_null());
        let target = unsafe { &mut (*target) };

        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        assert!(!text.is_null());

        match unsafe { CStr::from_ptr(text) }.to_str() {
            Ok(text) => {
                super::print(target, geometry, tilecache, text);
            }
            Err(e) => {
                eprintln!("Couldn't read text: {:?}.", e);
            }
        }
    }
}
