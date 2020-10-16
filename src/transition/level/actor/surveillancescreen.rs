use super::super::super::hero::HeroData;
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorMessageQueue};
use crate::{ANIMATION_BADGUYSCREEN, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16 * 2;
    general.position.h = TILE_HEIGHT as u16;

    Specific {}
}

fn hero_interact_start(
    _general: &mut ActorData,
    _specific: &mut Specific,
    _level_data: &mut LevelData,
    _hero_data: &mut HeroData,
    info_message_queue: &mut InfoMessageQueue,
    _actor_message_queue: &mut ActorMessageQueue,
) {
    // TODO: implement functionality.
    info_message_queue.push_back("Not implemented yet.".to_string());
}

fn blit(
    general: &mut ActorData,
    _specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let mut destrect = general.position;
    tilecache
        .get_tile(ANIMATION_BADGUYSCREEN)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.x += TILE_WIDTH as i16;
    tilecache
        .get_tile(ANIMATION_BADGUYSCREEN + 1)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorHeroInteractStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_surveillancescreen_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_surveillancescreen_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call(super::hero_interact_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_surveillancescreen_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
