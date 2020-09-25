use super::geometry::Geometry;
use super::text;
use super::texture::Texture;
use super::tilecache::TileCache;
use crate::{FONT_HEIGHT, FONT_WIDTH};

pub struct InputField {
    text: String,
    max_length: usize,
    cursor_position: usize,
}

impl InputField {
    pub fn new(max_length: usize) -> Self {
        InputField {
            text: String::new(),
            max_length,
            cursor_position: 0,
        }
    }

    pub fn backspace_pressed(&mut self) {
        if self.cursor_position > 0
            && self.cursor_position <= self.text.len()
        {
            self.cursor_position -= 1;
            self.text.remove(self.cursor_position);
        }
    }

    pub fn delete_pressed(&mut self) {
        if self.text.len() > 0
            && self.cursor_position < self.text.len() - 1
        {
            self.text.remove(self.cursor_position);
        }
    }

    pub fn left_pressed(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn right_pressed(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }

    pub fn symbol_pressed(&mut self, symbol: char) {
        if self.text.len() < self.max_length {
            if self.cursor_position == self.text.len() {
                self.text.push(symbol);
            } else {
                self.text.insert(self.cursor_position, symbol);
            }
            self.cursor_position += 1;
        }
    }

    pub fn blit(&self, target: &mut Texture, tilecache: &TileCache) {
        target.fill_area(None, 0, 0, 0);
        let sourcerect = Geometry {
            x: 0,
            y: 0,
            w: (FONT_WIDTH * self.cursor_position) as u16,
            h: FONT_HEIGHT as u16,
        };
        text::print(target, sourcerect, tilecache, &self.text);
        let cursorrect = Geometry {
            x: (self.cursor_position * FONT_WIDTH) as i16,
            y: 1,
            w: 1,
            h: FONT_HEIGHT as u16 - 2,
        };
        target.fill_area(Some(cursorrect), 0x88, 0x88, 0x88);
    }

    pub fn get_text(&self) -> &str {
        self.text.as_ref()
    }
}

pub mod ffi {
    pub type FnInputField = super::InputField;

    use super::super::geometry::ffi::FnGeometry;
    use super::super::texture::ffi::FnTexture;
    use super::super::tilecache::ffi::FnTileCache;
    use libc::c_char;
    use std::ffi::CStr;

    #[no_mangle]
    pub extern "C" fn fn_inputfield_create(
        max_length: u8,
    ) -> *mut FnInputField {
        Box::into_raw(Box::new(FnInputField::new(max_length as usize)))
    }

    #[no_mangle]
    pub extern "C" fn fn_inputfield_free(ptr: *mut FnInputField) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_inputfield_backspace_pressed(
        ptr: *mut FnInputField,
    ) {
        assert!(!ptr.is_null());
        let field = unsafe { &mut (*ptr) };
        field.backspace_pressed();
    }

    #[no_mangle]
    pub extern "C" fn fn_inputfield_delete_pressed(
        ptr: *mut FnInputField,
    ) {
        assert!(!ptr.is_null());
        let field = unsafe { &mut (*ptr) };
        field.delete_pressed();
    }

    #[no_mangle]
    pub extern "C" fn fn_inputfield_left_pressed(ptr: *mut FnInputField) {
        assert!(!ptr.is_null());
        let field = unsafe { &mut (*ptr) };
        field.left_pressed();
    }

    #[no_mangle]
    pub extern "C" fn fn_inputfield_right_pressed(ptr: *mut FnInputField) {
        assert!(!ptr.is_null());
        let field = unsafe { &mut (*ptr) };
        field.right_pressed();
    }

    #[no_mangle]
    pub extern "C" fn fn_inputfield_symbol_pressed(
        ptr: *mut FnInputField,
        symbol: c_char,
    ) {
        assert!(!ptr.is_null());
        let field = unsafe { &mut (*ptr) };
        field.symbol_pressed(symbol as u8 as char);
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_inputfield_blit(
        ptr: *const FnInputField,
        target: *mut FnTexture,
        tilecache: *const FnTileCache,
    ) {
        assert!(!ptr.is_null());
        assert!(!target.is_null());
        assert!(!tilecache.is_null());

        let inputfield = &(*ptr);
        let texture: &mut FnTexture = &mut (*target);
        let tilecache = &(*tilecache);

        inputfield.blit(texture, tilecache);
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_inputfield_text_length(
        ptr: *const FnInputField,
    ) -> usize {
        assert!(!ptr.is_null());
        let inputfield = &(*ptr);
        inputfield.get_text().as_bytes().len()
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_inputfield_copy_text_to(
        ptr: *const FnInputField,
        buffer: *mut c_char,
        max_length: usize,
    ) -> usize {
        assert!(!ptr.is_null());
        let inputfield = &(*ptr);

        assert!(!buffer.is_null());
        let buffer: &mut [u8] =
            std::slice::from_raw_parts_mut(buffer as *mut u8, max_length);
        let bytes = inputfield.get_text().as_bytes();
        let copy_count = std::cmp::min(bytes.len(), max_length);
        buffer[..copy_count].copy_from_slice(&bytes[..copy_count]);
        return copy_count;
    }
}
