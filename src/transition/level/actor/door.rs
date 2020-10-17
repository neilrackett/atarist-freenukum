use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorInterface, ActorMessageType, ActorQueue};
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

impl ActorInterface for Specific {
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
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        match self.state {
            State::Closed => {}
            State::Opening => {
                if self.counter == 0 {
                    level_data.solids.set(
                        general.position.x as usize / TILE_WIDTH,
                        general.position.y as usize / TILE_HEIGHT,
                        false,
                    );
                }
                self.counter += 1;
                if self.counter == 8 {
                    self.state = State::Open;
                    general.is_alive = false;
                }
            }
            State::Open => {}
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        tilecache
            .get_tile(self.tile + self.counter)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }

    fn receive_message(
        &mut self,
        _general: &mut ActorData,
        message: ActorMessageType,
        _hero_data: &mut HeroData,
        _level_data: &mut LevelData,
    ) {
        if message != ActorMessageType::OpenDoor {
            return;
        }
        self.state = State::Opening;
    }
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
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
