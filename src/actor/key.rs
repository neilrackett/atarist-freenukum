use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchStartParameters, RenderParameters,
};
use crate::hero::InventoryItem;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK, OBJECT_KEY_RED,
    TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        Specific {}
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        let item = match p.general.actor_type {
            ActorType::KeyRed => InventoryItem::KeyRed,
            ActorType::KeyBlue => InventoryItem::KeyBlue,
            ActorType::KeyPink => InventoryItem::KeyPink,
            ActorType::KeyGreen => InventoryItem::KeyGreen,
            _ => unreachable!(),
        };

        p.hero_data.inventory.set(item);
        p.hero_data.score.add(1000);
        p.actor_adder.add_actor(
            ActorType::Score1000,
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
        p.general.is_alive = false;
    }

    fn act(&mut self, _p: ActParameters) {}

    fn render(&mut self, p: RenderParameters) {
        let tile = match p.general.actor_type {
            ActorType::KeyRed => OBJECT_KEY_RED,
            ActorType::KeyBlue => OBJECT_KEY_BLUE,
            ActorType::KeyPink => OBJECT_KEY_PINK,
            ActorType::KeyGreen => OBJECT_KEY_GREEN,
            _ => unreachable!(),
        };

        p.renderer.place_tile(tile, p.general.position);
    }
}
