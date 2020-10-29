pub mod actor;
pub mod raw;
pub mod solids;
pub mod tiles;

use super::hero::HeroData;
use super::infobox::InfoMessageQueue;
use actor::ActorMessageQueue;

#[derive(Debug)]
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
            actors: actor::ActorsList::new(),
        }
    }

    pub fn hero_interact_start(
        &mut self,
        hero: &mut HeroData,
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        self.actors.start_interaction(
            &mut self.level_passed,
            hero,
            info_message_queue,
            actor_message_queue,
        );
    }

    pub fn hero_interact_end(&mut self, hero: &mut HeroData) {
        self.actors.end_interaction(&mut self.level_passed, hero);
    }
}

impl actor::ActorAdder for LevelData {
    fn add_actor(&mut self, actor_type: actor::ActorType, x: u16, y: u16) {
        let mut general = actor::ActorData::new(actor_type);
        general.position.x = x as i16;
        general.position.y = y as i16;
        let specific =
            actor_type.create_actor_interface(&mut general, self);
        self.actors.add(actor::Actor { general, specific });
    }
}

pub mod ffi {
    pub type FnLevelData = super::LevelData;

    use super::super::hero::ffi::FnHeroData;
    use super::super::infobox::ffi::FnInfoMessageQueue;
    use super::actor::ffi::{FnLevelActorMessageQueue, FnLevelActorsList};
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
    pub extern "C" fn fn_level_data_hero_interact_start(
        ptr: *mut FnLevelData,
        hero: *mut FnHeroData,
        info_message_queue: *mut FnInfoMessageQueue,
        actor_message_queue: *mut FnLevelActorMessageQueue,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };

        assert!(!hero.is_null());
        let hero: &mut FnHeroData = unsafe { &mut (*hero) };

        assert!(!info_message_queue.is_null());
        let info_message_queue: &mut FnInfoMessageQueue =
            unsafe { &mut (*info_message_queue) };

        assert!(!actor_message_queue.is_null());
        let actor_message_queue: &mut FnLevelActorMessageQueue =
            unsafe { &mut (*actor_message_queue) };

        d.hero_interact_start(
            hero,
            info_message_queue,
            actor_message_queue,
        );
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_hero_interact_end(
        ptr: *mut FnLevelData,
        hero: *mut FnHeroData,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };

        assert!(!hero.is_null());
        let hero: &mut FnHeroData = unsafe { &mut (*hero) };

        d.hero_interact_end(hero);
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
