use crate::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug)]
pub struct LevelRaw {
    raw: [[u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
}

impl LevelRaw {
    pub fn new() -> Self {
        LevelRaw {
            raw: [[0u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: u16) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.raw[y][x] = tile;
    }

    pub fn get(&self, x: usize, y: usize) -> u16 {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.raw[y][x]
    }
}

pub mod ffi {
    pub type FnLevelRaw = super::LevelRaw;

    #[no_mangle]
    pub extern "C" fn fn_level_raw_create() -> *mut FnLevelRaw {
        Box::into_raw(Box::new(FnLevelRaw::new()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_raw_free(ptr: *mut FnLevelRaw) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_raw_get(
        ptr: *const FnLevelRaw,
        x: usize,
        y: usize,
    ) -> u16 {
        assert!(!ptr.is_null());
        let raw: &FnLevelRaw = unsafe { &(*ptr) };
        raw.get(x, y)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_raw_set(
        ptr: *mut FnLevelRaw,
        x: usize,
        y: usize,
        tile: u16,
    ) {
        assert!(!ptr.is_null());
        let raw: &mut FnLevelRaw = unsafe { &mut (*ptr) };
        raw.set(x, y, tile);
    }
}
