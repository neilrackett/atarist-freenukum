use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    BACKGROUND_LIGHT_GREY, SOLID_SHOOTABLE_WALL_BRICKS, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = false;

    Specific {}
}

fn shot(
    general: &mut ActorData,
    _specific: &mut Specific,
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
    general: &mut ActorData,
    _specific: &mut Specific,
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

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_shootable_wall_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_shootable_wall_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_shootable_wall_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
