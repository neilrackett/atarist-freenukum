use super::super::super::hero::HeroData;
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorMessageQueue, ActorQueue};
use crate::{ANIMATION_EXITDOOR, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Closed,
    Opening,
    Closing,
}

#[derive(Debug)]
struct Specific {
    tile: usize,
    counter: usize,
    state: State,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16 * 2;
    general.position.h = TILE_HEIGHT as u16 * 2;
    general.is_in_foreground = false;

    Specific {
        tile: ANIMATION_EXITDOOR,
        counter: 0,
        state: State::Closed,
    }
}

fn hero_interact_start(
    _general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _hero_data: &mut HeroData,
    _info_message_queue: &mut InfoMessageQueue,
    _actor_message_queue: &mut ActorMessageQueue,
) {
    if specific.state == State::Closed {
        specific.state = State::Opening;
    }
    level_data.level_passed = true;
}

fn act(
    _general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    match specific.state {
        State::Closed => {}
        State::Opening => {
            specific.counter += 1;
            if specific.counter >= 4 {
                hero_data.hidden = true;
                specific.state = State::Closing;
                specific.counter -= 1;
            }
        }
        State::Closing => {
            if specific.counter == 0 {
                level_data.do_play = false;
                hero_data.hidden = false;
            } else {
                specific.counter -= 1;
            }
        }
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let mut destrect = general.position;
    tilecache
        .get_tile(specific.tile + specific.counter * 4)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.x += TILE_WIDTH as i16;
    tilecache
        .get_tile(specific.tile + specific.counter * 4 + 1)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.x -= TILE_WIDTH as i16;
    destrect.y += TILE_HEIGHT as i16;
    tilecache
        .get_tile(specific.tile + specific.counter * 4 + 2)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.x += TILE_WIDTH as i16;
    tilecache
        .get_tile(specific.tile + specific.counter * 4 + 3)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call(super::hero_interact_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
