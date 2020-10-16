use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    OBJECT_SPIKE, OBJECT_SPIKES_DOWN, OBJECT_SPIKES_UP, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    touching_hero: bool,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = true;

    Specific {
        touching_hero: false,
    }
}

fn hero_touch_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    general.hurts_hero = true;
    specific.touching_hero = true;
}

fn hero_touch_end(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
) {
    general.hurts_hero = false;
    specific.touching_hero = false;
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = match general.actor_type {
        ActorType::SpikesUp => OBJECT_SPIKES_UP,
        ActorType::SpikesDown => OBJECT_SPIKES_DOWN,
        ActorType::Spike if specific.touching_hero => OBJECT_SPIKE + 1,
        ActorType::Spike => OBJECT_SPIKE,
        _ => unreachable!(),
    };

    let tile = tilecache.get_tile(tile).unwrap();
    let destrect = general.position;
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorFreeParams, FnLevelActorHeroTouchEndParams,
        FnLevelActorHeroTouchStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call(super::hero_touch_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
