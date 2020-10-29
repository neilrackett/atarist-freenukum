pub mod actor;
pub mod raw;
pub mod solids;
pub mod tiles;

use super::hero::HeroData;
use super::infobox::InfoMessageQueue;
use super::level::solids::LevelSolids;
use super::level::tiles::LevelTiles;
use super::shot::{Shot, ShotList};
use super::tilecache::TileCache;
use super::HorizontalDirection;
use crate::HALFTILE_WIDTH;
use actor::{ActorAdder, ActorMessageQueue, ActorsList};
use transdl::video::Surface;

#[derive(Debug)]
pub struct LevelData {
    pub tiles: LevelTiles,
    pub solids: LevelSolids,
    pub do_play: bool,
    pub level_passed: bool,
    pub actors: ActorsList,
    pub animated_frames_since_last_act: usize,
    pub shots: ShotList,
}

impl LevelData {
    pub fn new() -> Self {
        LevelData {
            tiles: LevelTiles::new(),
            solids: LevelSolids::new(),
            do_play: true,
            level_passed: false,
            actors: ActorsList::new(),
            animated_frames_since_last_act: 0,
            shots: Vec::new(),
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

    pub fn animated_frames_since_last_act_increase(&mut self) -> usize {
        self.animated_frames_since_last_act += 1;
        self.animated_frames_since_last_act %= 1;
        self.animated_frames_since_last_act
    }

    pub fn blit(
        &self,
        target: &mut Surface,
        tilecache: &TileCache,
        draw_collision_bounds: bool,
    ) {
        for shot in self.shots.iter() {
            shot.blit(target, tilecache, draw_collision_bounds);
        }
    }

    pub fn act(
        &mut self,
        hero_data: &mut HeroData,
        actor_adder: &mut dyn ActorAdder,
    ) {
        for shot in self.shots.iter_mut() {
            shot.act(
                hero_data,
                &mut self.actors,
                &mut self.solids,
                &mut self.tiles,
                actor_adder,
            );
        }
        self.shots.retain(|s| s.is_alive);
    }

    pub fn fire_shot(
        &mut self,
        hero: &mut HeroData,
        actor_adder: &mut dyn ActorAdder,
    ) {
        if self.shots.len() < hero.firepower.num_shots() as usize {
            let heropos = hero.position.geometry;
            let mut shot = Shot::new(heropos.x, heropos.y, hero.direction);

            let distance = match shot.direction {
                HorizontalDirection::Left => -(HALFTILE_WIDTH as i16),
                HorizontalDirection::Right => HALFTILE_WIDTH as i16,
                _ => unreachable!(),
            };

            // we only push half of the distance, but do it twice, so that
            // also the intermediate position gets covered, not just the
            // end position.
            shot.push(
                hero,
                &mut self.actors,
                &mut self.solids,
                &mut self.tiles,
                distance,
                actor_adder,
            );

            self.shots.push(shot);
        }
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
    use super::super::tilecache::ffi::FnTileCache;
    use super::actor::ffi::{
        FnLevelActorMessageQueue, FnLevelActorQueue, FnLevelActorsList,
    };
    use super::solids::ffi::FnLevelSolids;
    use super::tiles::ffi::FnLevelTiles;
    use transdl::ll::SDL_Surface;

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
    pub extern "C" fn fn_level_data_act(
        ptr: *mut FnLevelData,
        hero: *mut FnHeroData,
        actor_queue: *mut FnLevelActorQueue,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };

        assert!(!hero.is_null());
        let hero: &mut FnHeroData = unsafe { &mut (*hero) };

        assert!(!actor_queue.is_null());
        let actor_queue: &mut FnLevelActorQueue =
            unsafe { &mut (*actor_queue) };

        d.act(hero, actor_queue);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_fire_shot(
        ptr: *mut FnLevelData,
        hero: *mut FnHeroData,
        actor_queue: *mut FnLevelActorQueue,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };

        assert!(!hero.is_null());
        let hero: &mut FnHeroData = unsafe { &mut (*hero) };

        assert!(!actor_queue.is_null());
        let actor_queue: &mut FnLevelActorQueue =
            unsafe { &mut (*actor_queue) };

        d.fire_shot(hero, actor_queue);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_data_blit(
        ptr: *const FnLevelData,
        target: *mut SDL_Surface,
        tilecache: *const FnTileCache,
        draw_collision_bounds: bool,
    ) {
        assert!(!ptr.is_null());
        let d: &FnLevelData = unsafe { &(*ptr) };

        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        assert!(!target.is_null());
        let mut target = transdl::video::Surface { raw: target };

        d.blit(&mut target, tilecache, draw_collision_bounds);
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

    #[no_mangle]
    pub extern "C" fn fn_level_data_animated_frames_since_last_act_increase(
        ptr: *mut FnLevelData,
    ) -> usize {
        assert!(!ptr.is_null());
        let d: &mut FnLevelData = unsafe { &mut (*ptr) };
        d.animated_frames_since_last_act_increase()
    }
}
