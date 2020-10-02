use crate::{LEVEL_HEIGHT,LEVEL_WIDTH};

pub struct LevelSolids {
    solids: [[bool;LEVEL_WIDTH];LEVEL_HEIGHT]
}

impl LevelSolids {
    pub fn new() -> Self {
        LevelSolids {
            solids: [[false;LEVEL_WIDTH];LEVEL_HEIGHT]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.solids[y][x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.solids[y][x]
    }
}

pub mod ffi {
    pub type FnLevelSolids = super::LevelSolids;

    #[no_mangle]
    pub extern "C" fn fn_level_solids_create() -> *mut FnLevelSolids {
        Box::into_raw(Box::new(FnLevelSolids::new()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_solids_free(ptr: *mut FnLevelSolids) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_solids_get(
        ptr: *const FnLevelSolids,
        x: usize,
        y: usize,
    ) -> bool {
        assert!(!ptr.is_null());
        let solids: &FnLevelSolids = unsafe { &(*ptr) };
        solids.get(x,y)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_solids_set(ptr: *mut FnLevelSolids,
        x: usize,
        y: usize,
        value: bool
    ) {
        assert!(!ptr.is_null());
        let solids = unsafe { &mut (*ptr) };
        solids.set(x,y, value);
    }
}
