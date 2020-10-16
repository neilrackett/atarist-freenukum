use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorMessageType, ActorQueue};
use crate::{OBJECT_DOOR, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug, PartialEq, Eq)]
enum State {
    Closed,
    Opening,
    Open,
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
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    Specific {
        tile: OBJECT_DOOR,
        counter: 0,
        state: State::Closed,
    }
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    match specific.state {
        State::Closed => {}
        State::Opening => {
            if specific.counter == 0 {
                level_data.solids.set(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT,
                    false,
                );
            }
            specific.counter += 1;
            if specific.counter == 8 {
                specific.state = State::Open;
                general.is_alive = false;
            }
        }
        State::Open => {}
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    tilecache
        .get_tile(specific.tile + specific.counter)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(general.position));
}

fn receive_message(
    _general: &mut ActorData,
    specific: &mut Specific,
    message: ActorMessageType,
    _hero_data: &mut HeroData,
    _level_data: &mut LevelData,
) {
    if message != ActorMessageType::OpenDoor {
        return;
    }
    specific.state = State::Opening;
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorReceiveMessageParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        p.call(super::receive_message);
    }
}
