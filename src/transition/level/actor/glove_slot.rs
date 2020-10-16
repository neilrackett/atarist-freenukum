use super::super::super::hero::{HeroData, InventoryItem};
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorData, ActorMessageQueue, ActorMessageType, ActorQueue, ActorType,
};
use crate::{OBJECT_GLOVE_SLOT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
enum State {
    Idle,
    Shooting,
    Expanded,
}

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    state: State,
    countdown: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = false;

    Specific {
        tile: OBJECT_GLOVE_SLOT,
        current_frame: 0,
        num_frames: 4,
        state: State::Idle,
        countdown: 0,
    }
}

fn hero_interact_start(
    _general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    hero_data: &mut HeroData,
    _info_message_queue: &mut InfoMessageQueue,
    actor_message_queue: &mut ActorMessageQueue,
) {
    match specific.state {
        State::Idle => {
            if hero_data.inventory.is_set(InventoryItem::Glove) {
                actor_message_queue.push_back(
                    ActorType::ExpandingFloor,
                    ActorMessageType::Expand,
                );
                specific.state = State::Expanded;
            } else {
                specific.state = State::Shooting;
                specific.countdown = 20;
            }
        }
        State::Shooting => {}
        State::Expanded => {}
    }
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    match specific.state {
        State::Idle => {
            specific.current_frame += 1;
            specific.current_frame %= specific.num_frames;
        }
        State::Shooting => {
            specific.current_frame += 1;
            specific.current_frame %= specific.num_frames;
            specific.countdown -= 1;
            if specific.countdown % 4 == 0 {
                actor_queue.push_back(
                    ActorType::HostileShotRight,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            } else if specific.countdown % 4 == 2 {
                actor_queue.push_back(
                    ActorType::HostileShotLeft,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            if specific.countdown == 0 {
                specific.state = State::Idle;
            }
        }
        State::Expanded => {}
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let adder = if specific.current_frame == 0 { 0 } else { 1 };
    let mut destrect = general.position;
    tilecache
        .get_tile(specific.tile + adder)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));

    destrect.x -= TILE_WIDTH as i16;
    tilecache
        .get_tile(specific.tile + 2)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));

    destrect.x += 2 * TILE_WIDTH as i16;
    tilecache
        .get_tile(specific.tile + 3)
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
    pub extern "C" fn fn_level_actor_function_glove_slot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call(super::hero_interact_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
