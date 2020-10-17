use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{
    BACKGROUND_LIGHT_GREY, SOLID_SHOOTABLE_WALL_BRICKS, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        Specific {}
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        hero_data.score.add(10);
        actor_queue.push_back(
            ActorType::Explosion,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
        level_data.solids.set(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT,
            false,
        );
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        tilecache
            .get_tile(BACKGROUND_LIGHT_GREY)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
        tilecache
            .get_tile(SOLID_SHOOTABLE_WALL_BRICKS)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_shootable_wall_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_shootable_wall_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_shootable_wall_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
