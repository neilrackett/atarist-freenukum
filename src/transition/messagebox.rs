use super::geometry::Geometry;
use super::text;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::{
    BORDER_BLUE_BOTTOM, BORDER_BLUE_BOTTOMLEFT, BORDER_BLUE_BOTTOMRIGHT,
    BORDER_BLUE_LEFT, BORDER_BLUE_MIDDLE, BORDER_BLUE_RIGHT,
    BORDER_BLUE_TOP, BORDER_BLUE_TOPLEFT, BORDER_BLUE_TOPRIGHT,
    FONT_HEIGHT, FONT_WIDTH,
};

fn get_information(text: &str) -> (usize, usize) {
    let mut columns = 0;
    let mut rows = 0;

    for line in text.lines() {
        columns = std::cmp::max(columns, line.len());
        rows += 1;
    }
    (columns, rows)
}

pub fn messagebox(
    text: &str,
    tilecache: &TileCache,
    params: TextureCreationParams,
) -> Texture {
    let (columns, rows) = get_information(text);

    let mut messagebox = Texture::create_with_params(
        (FONT_WIDTH * (columns + 2)) as u16,
        (FONT_HEIGHT * (rows + 2)) as u16,
        params,
    );

    for row in 0..=rows {
        for col in 0..=columns {
            let tilenr = match (row, col) {
                (0, 0) => BORDER_BLUE_TOPLEFT,
                (0, c) if c == columns => BORDER_BLUE_TOPRIGHT,
                (r, 0) if r == rows => BORDER_BLUE_BOTTOMLEFT,
                (r, c) if r == rows && c == columns => {
                    BORDER_BLUE_BOTTOMRIGHT
                }
                (0, _) => BORDER_BLUE_TOP,
                (_, 0) => BORDER_BLUE_LEFT,
                (r, _) if r == rows => BORDER_BLUE_BOTTOM,
                (_, c) if c == columns => BORDER_BLUE_RIGHT,
                _ => BORDER_BLUE_MIDDLE,
            };

            let r = Geometry::new(
                col as i16 * FONT_WIDTH as i16,
                row as i16 * FONT_HEIGHT as i16,
                FONT_WIDTH as u16,
                FONT_HEIGHT as u16,
            );

            tilecache.get_tile(tilenr).unwrap().clone_to_texture(
                None,
                &mut messagebox,
                Some(r),
            );
        }
    }

    let r = Geometry::new(
        FONT_WIDTH as i16,
        FONT_HEIGHT as i16,
        FONT_WIDTH as u16,
        FONT_HEIGHT as u16,
    );
    text::print(&mut messagebox, r, tilecache, text);

    messagebox
}

pub mod ffi {
    use super::super::texture::ffi::{FnTexture, FnTextureCreationParams};
    use super::super::tilecache::ffi::FnTileCache;
    use libc::c_char;
    use std::ffi::CStr;

    #[no_mangle]
    pub extern "C" fn fn_messagebox(
        text: *const c_char,
        tilecache: *const FnTileCache,
        params: FnTextureCreationParams,
    ) -> *mut FnTexture {
        assert!(!text.is_null());

        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        match unsafe { CStr::from_ptr(text) }.to_str() {
            Ok(text) => Box::into_raw(Box::new(super::messagebox(
                text, tilecache, params,
            ))),
            Err(e) => {
                eprintln!("Couldn't read text: {:?}.", e);
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_messagebox_get_text_information(
        text: *const c_char,
        cols: *mut u16,
        rows: *mut u16,
    ) {
        assert!(!text.is_null());
        assert!(!cols.is_null());
        assert!(!rows.is_null());

        let cols: &mut u16 = unsafe { &mut (*cols) };
        let rows: &mut u16 = unsafe { &mut (*rows) };

        match unsafe { CStr::from_ptr(text) }.to_str() {
            Ok(text) => {
                let (cols_usize, rows_usize) =
                    super::get_information(text);
                *cols = cols_usize as u16;
                *rows = rows_usize as u16;
            }
            Err(e) => {
                eprintln!("Couldn't read text: {:?}.", e);
            }
        }
    }
}
