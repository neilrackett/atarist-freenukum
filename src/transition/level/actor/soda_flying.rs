use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{ANIMATION_SODAFLY, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    Specific {}
}

fn hero_touch_start(
    general: &mut ActorData,
    _specific: &mut Specific,
    actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    hero_data.score.add(1000);
    actor_queue.push_back(
        ActorType::Score1000,
        general.position.x as u16,
        general.position.y as u16,
    );
    general.is_alive = false;
}

fn act(
    general: &mut ActorData,
    _specific: &mut Specific,
    level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    general.position.y -= HALFTILE_HEIGHT as i16;
    if level_data.solids.get(
        general.position.x as usize / TILE_WIDTH,
        general.position.y as usize / TILE_HEIGHT,
    ) {
        actor_queue.push_back(
            ActorType::Explosion,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
    }
}

fn blit(
    general: &mut ActorData,
    _specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = tilecache
        .get_tile(
            ANIMATION_SODAFLY
                + ((general.position.y as usize / HALFTILE_HEIGHT) % 4),
        )
        .unwrap();
    let destrect = general.position;
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorHeroTouchStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
