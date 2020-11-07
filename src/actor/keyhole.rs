use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageType, ActorType, HeroInteractStartParameters,
    RenderParameters,
};
use crate::hero::InventoryItem;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    OBJECT_KEYHOLE_BLACK, OBJECT_KEYHOLE_BLUE, OBJECT_KEYHOLE_GREEN,
    OBJECT_KEYHOLE_PINK, OBJECT_KEYHOLE_RED, TILE_HEIGHT, TILE_WIDTH,
};

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
    fn act(&mut self, _p: ActParameters) {
        if self.counter < 5 {
            self.counter += 1;
            self.counter %= 4;
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let tile = match (self.counter, p.general.actor_type) {
            (0, _) => self.tile,
            (_, ActorType::KeyholeRed) => OBJECT_KEYHOLE_RED,
            (_, ActorType::KeyholeBlue) => OBJECT_KEYHOLE_BLUE,
            (_, ActorType::KeyholePink) => OBJECT_KEYHOLE_PINK,
            (_, ActorType::KeyholeGreen) => OBJECT_KEYHOLE_GREEN,
            _ => unreachable!(),
        };

        p.tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            p.target,
            Some(p.general.position),
        );
    }

    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        let (required_item, door_actor_type) = match p.general.actor_type {
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

        if p.hero_data.inventory.is_set(required_item) {
            p.actor_message_queue
                .push_back(door_actor_type, ActorMessageType::OpenDoor);
            self.counter = 5;
            p.hero_data.inventory.unset(required_item);
        } else if self.counter < 5 {
            let color = match required_item {
                InventoryItem::KeyRed => "red",
                InventoryItem::KeyBlue => "blue",
                InventoryItem::KeyPink => "pink",
                InventoryItem::KeyGreen => "green",
                _ => unreachable!(),
            };
            p.info_message_queue
                .push_back(format!("You don't have the {} key.", color));
        }
    }
}
