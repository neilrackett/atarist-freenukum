use super::super::super::hero::HeroData;
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorMessageQueue,
    ActorQueue,
};
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

impl ActorCreateInterface for Specific {
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
}

impl ActorInterface for Specific {
    fn hero_interact_start(
        &mut self,
        _general: &mut ActorData,
        level_data: &mut LevelData,
        _hero_data: &mut HeroData,
        _info_message_queue: &mut InfoMessageQueue,
        _actor_message_queue: &mut ActorMessageQueue,
    ) {
        if self.state == State::Closed {
            self.state = State::Opening;
        }
        level_data.level_passed = true;
    }

    fn act(
        &mut self,
        _general: &mut ActorData,
        level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        match self.state {
            State::Closed => {}
            State::Opening => {
                self.counter += 1;
                if self.counter >= 4 {
                    hero_data.hidden = true;
                    self.state = State::Closing;
                    self.counter -= 1;
                }
            }
            State::Closing => {
                if self.counter == 0 {
                    level_data.do_play = false;
                    hero_data.hidden = false;
                } else {
                    self.counter -= 1;
                }
            }
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let mut destrect = general.position;
        tilecache
            .get_tile(self.tile + self.counter * 4)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(self.tile + self.counter * 4 + 1)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
        destrect.x -= TILE_WIDTH as i16;
        destrect.y += TILE_HEIGHT as i16;
        tilecache
            .get_tile(self.tile + self.counter * 4 + 2)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(self.tile + self.counter * 4 + 3)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
    }
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
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
