use super::super::hero::{HeroData, InventoryItem};
use super::super::infobox::InfoMessageQueue;
use super::super::level::solids::LevelSolids;
use super::super::level::tiles::LevelTiles;
use super::super::tilecache::TileCache;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageQueue, ActorMessageType, ActorType,
};
use crate::{OBJECT_ACCESS_CARD_SLOT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        Specific {
            tile: OBJECT_ACCESS_CARD_SLOT,
            current_frame: 0,
            num_frames: 8,
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
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        if hero_data.inventory.is_set(InventoryItem::AccessCard) {
            actor_message_queue.push_back(
                ActorType::AccessCardDoor,
                ActorMessageType::OpenDoor,
            );
            self.current_frame = 0;
            self.num_frames = 1;
            self.tile = OBJECT_ACCESS_CARD_SLOT + 8;
            hero_data.inventory.unset(InventoryItem::AccessCard);
        } else {
            info_message_queue
                .push_back("You don't have the access card\n".to_string());
        }
    }

    fn act(
        &mut self,
        _general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        tilecache
            .get_tile(self.tile + self.current_frame)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }
}
