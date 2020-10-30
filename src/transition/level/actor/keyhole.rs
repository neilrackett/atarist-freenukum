use super::super::super::hero::{HeroData, InventoryItem};
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::solids::LevelSolids;
use super::super::tiles::LevelTiles;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageQueue, ActorMessageType, ActorType,
};
use crate::{
    OBJECT_KEYHOLE_BLACK, OBJECT_KEYHOLE_BLUE, OBJECT_KEYHOLE_GREEN,
    OBJECT_KEYHOLE_PINK, OBJECT_KEYHOLE_RED, TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    counter: usize,
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
            tile: OBJECT_KEYHOLE_BLACK,
            counter: 0,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        _general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        if self.counter < 5 {
            self.counter += 1;
            self.counter %= 4;
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = match (self.counter, general.actor_type) {
            (0, _) => self.tile,
            (_, ActorType::KeyholeRed) => OBJECT_KEYHOLE_RED,
            (_, ActorType::KeyholeBlue) => OBJECT_KEYHOLE_BLUE,
            (_, ActorType::KeyholePink) => OBJECT_KEYHOLE_PINK,
            (_, ActorType::KeyholeGreen) => OBJECT_KEYHOLE_GREEN,
            _ => unreachable!(),
        };

        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            target,
            Some(general.position),
        );
    }

    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(
        &mut self,
        general: &mut ActorData,
        _level_passed: &mut bool,
        hero_data: &mut HeroData,
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        let (required_item, door_actor_type) = match general.actor_type {
            ActorType::KeyholeRed => {
                (InventoryItem::KeyRed, ActorType::DoorRed)
            }
            ActorType::KeyholeBlue => {
                (InventoryItem::KeyBlue, ActorType::DoorBlue)
            }
            ActorType::KeyholePink => {
                (InventoryItem::KeyPink, ActorType::DoorPink)
            }
            ActorType::KeyholeGreen => {
                (InventoryItem::KeyGreen, ActorType::DoorGreen)
            }
            _ => unreachable!(),
        };

        if hero_data.inventory.is_set(required_item) {
            actor_message_queue
                .push_back(door_actor_type, ActorMessageType::OpenDoor);
            self.counter = 5;
            hero_data.inventory.unset(required_item);
        } else if self.counter < 5 {
            let color = match required_item {
                InventoryItem::KeyRed => "red",
                InventoryItem::KeyBlue => "blue",
                InventoryItem::KeyPink => "pink",
                InventoryItem::KeyGreen => "green",
                _ => unreachable!(),
            };
            info_message_queue
                .push_back(format!("You don't have the {} key.", color));
        }
    }
}
