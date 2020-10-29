use crate::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug)]
#[repr(C)]
pub struct LevelTiles {
    tiles: [[u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
}

impl LevelTiles {
    pub fn new() -> Self {
        LevelTiles {
            tiles: [[0u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u16) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.tiles[y][x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> u16 {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.tiles[y][x]
    }

    pub fn copy_from_to(
        &mut self,
        x_from: usize,
        y_from: usize,
        x_to: usize,
        y_to: usize,
    ) {
        self.set(x_to, y_to, self.get(x_from, y_from));
    }
}

pub mod ffi {
    pub type FnLevelTiles = super::LevelTiles;

    #[no_mangle]
    pub extern "C" fn fn_level_tiles_create() -> *mut FnLevelTiles {
        Box::into_raw(Box::new(FnLevelTiles::new()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_tiles_free(ptr: *mut FnLevelTiles) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_tiles_get(
        ptr: *const FnLevelTiles,
        x: usize,
        y: usize,
    ) -> u16 {
        assert!(!ptr.is_null());
        let tiles: &FnLevelTiles = unsafe { &(*ptr) };
        tiles.get(x, y)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_tiles_set(
        ptr: *mut FnLevelTiles,
        x: usize,
        y: usize,
        value: u16,
    ) {
        assert!(!ptr.is_null());
        let tiles = unsafe { &mut (*ptr) };
        tiles.set(x, y, value);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_tiles_copy_from_to(
        ptr: *mut FnLevelTiles,
        x_from: usize,
        y_from: usize,
        x_to: usize,
        y_to: usize,
    ) {
        assert!(!ptr.is_null());
        let tiles = unsafe { &mut (*ptr) };
        tiles.copy_from_to(x_from, y_from, x_to, y_to);
    }
}
