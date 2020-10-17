use super::super::super::hero::{HeroData, InventoryItem};
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorData, ActorInterface, ActorMessageQueue, ActorMessageType,
    ActorQueue, ActorType,
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

impl ActorInterface for Specific {
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
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        hero_data: &mut HeroData,
        _info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        match self.state {
            State::Idle => {
                if hero_data.inventory.is_set(InventoryItem::Glove) {
                    actor_message_queue.push_back(
                        ActorType::ExpandingFloor,
                        ActorMessageType::Expand,
                    );
                    self.state = State::Expanded;
                } else {
                    self.state = State::Shooting;
                    self.countdown = 20;
                }
            }
            State::Shooting => {}
            State::Expanded => {}
        }
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        match self.state {
            State::Idle => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
            }
            State::Shooting => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
                self.countdown -= 1;
                if self.countdown % 4 == 0 {
                    actor_queue.push_back(
                        ActorType::HostileShotRight,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                } else if self.countdown % 4 == 2 {
                    actor_queue.push_back(
                        ActorType::HostileShotLeft,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                }
                if self.countdown == 0 {
                    self.state = State::Idle;
                }
            }
            State::Expanded => {}
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let adder = if self.current_frame == 0 { 0 } else { 1 };
        let mut destrect = general.position;
        tilecache
            .get_tile(self.tile + adder)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));

        destrect.x -= TILE_WIDTH as i16;
        tilecache
            .get_tile(self.tile + 2)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));

        destrect.x += 2 * TILE_WIDTH as i16;
        tilecache
            .get_tile(self.tile + 3)
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
    pub extern "C" fn fn_level_actor_function_glove_slot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
