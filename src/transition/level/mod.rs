pub mod actor;
pub mod solids;
pub mod tiles;

#[repr(C)]
pub struct LevelData {
    pub tiles: tiles::ffi::FnLevelTiles,
    pub solids: solids::ffi::FnLevelSolids,
    pub do_play: bool,
    pub level_passed: bool,
}

impl LevelData {
    pub fn new() -> Self {
        LevelData {
            tiles: tiles::LevelTiles::new(),
            solids: solids::LevelSolids::new(),
            do_play: true,
            level_passed: false,
        }
    }
}

pub mod ffi {
    pub type FnLevelData = super::LevelData;

    #[no_mangle]
    pub extern "C" fn fn_level_data_create() -> *mut FnLevelData {
        Box::into_raw(Box::new(FnLevelData::new()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_free(ptr: *mut FnLevelData) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}
