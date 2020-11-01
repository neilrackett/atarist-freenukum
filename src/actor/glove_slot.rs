use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageQueue, ActorMessageType, ActorType,
};
use crate::hero::{HeroData, InventoryItem};
use crate::infobox::InfoMessageQueue;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{OBJECT_GLOVE_SLOT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
enum State {
    Idle,
    Shooting,
    Expanded,
}

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    state: State,
    countdown: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
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
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(
        &mut self,
        _general: &mut ActorData,
        _level_passed: &mut bool,
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
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
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
                    actor_adder.add_actor(
                        ActorType::HostileShotRight,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                } else if self.countdown % 4 == 2 {
                    actor_adder.add_actor(
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
