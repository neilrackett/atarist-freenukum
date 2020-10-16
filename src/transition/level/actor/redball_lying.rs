use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{ANIMATION_MINE, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    touching_hero: u8,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    Specific {
        tile: ANIMATION_MINE,
        touching_hero: 0,
    }
}

fn hero_touch_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    general.hurts_hero = true;
    specific.touching_hero = 1;
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    if !level_data.solids.get(
        general.position.x as usize / TILE_WIDTH,
        general.position.y as usize / TILE_HEIGHT + 1,
    ) {
        general.position.y += HALFTILE_HEIGHT as i16;
    }

    match specific.touching_hero {
        1 => specific.touching_hero += 1,
        2 => {
            general.hurts_hero = false;
            general.is_alive = false;
            actor_queue.push_back(
                ActorType::BombFire,
                general.position.x as u16,
                general.position.y as u16,
            );
        }
        _ => {}
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = tilecache.get_tile(specific.tile as usize).unwrap();
    let destrect = general.position;
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
