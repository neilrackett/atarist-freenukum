pub mod actor;
pub mod raw;
pub mod solids;
pub mod tiles;

pub struct LevelData {
    pub tiles: tiles::LevelTiles,
    pub solids: solids::LevelSolids,
    pub do_play: bool,
    pub level_passed: bool,
    pub actors: actor::ActorsList,
}

impl LevelData {
    pub fn new() -> Self {
        LevelData {
            tiles: tiles::LevelTiles::new(),
            solids: solids::LevelSolids::new(),
            do_play: true,
            level_passed: false,
            actors: Vec::new(),
        }
    }
}

pub mod ffi {
    pub type FnLevelData = super::LevelData;

    use super::actor::ffi::FnLevelActorsList;
    use super::solids::ffi::FnLevelSolids;
    use super::tiles::ffi::FnLevelTiles;

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

    #[no_mangle]
    pub extern "C" fn fn_level_data_get_tiles(
        ptr: *mut FnLevelData,
    ) -> *mut FnLevelTiles {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };
        &mut d.tiles
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_get_solids(
        ptr: *mut FnLevelData,
    ) -> *mut FnLevelSolids {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };
        &mut d.solids
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_get_actors_list(
        ptr: *mut FnLevelData,
    ) -> *mut FnLevelActorsList {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };
        &mut d.actors
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_get_do_play(
        ptr: *const FnLevelData,
    ) -> bool {
        assert!(!ptr.is_null());
        let d: &FnLevelData = unsafe { &(*ptr) };
        d.do_play
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_set_do_play(
        ptr: *mut FnLevelData,
        do_play: bool,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };
        d.do_play = do_play
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_get_level_passed(
        ptr: *const FnLevelData,
    ) -> bool {
        assert!(!ptr.is_null());
        let d: &FnLevelData = unsafe { &(*ptr) };
        d.level_passed
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_set_level_passed(
        ptr: *mut FnLevelData,
        level_passed: bool,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };
        d.level_passed = level_passed
    }
}
