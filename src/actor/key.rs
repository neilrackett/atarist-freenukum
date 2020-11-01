use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::{HeroData, InventoryItem};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{
    OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK, OBJECT_KEY_RED,
    TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

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
    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
    ) {
        let item = match general.actor_type {
            ActorType::KeyRed => InventoryItem::KeyRed,
            ActorType::KeyBlue => InventoryItem::KeyBlue,
            ActorType::KeyPink => InventoryItem::KeyPink,
            ActorType::KeyGreen => InventoryItem::KeyGreen,
            _ => unreachable!(),
        };

        hero_data.inventory.set(item);
        hero_data.score.add(1000);
        actor_adder.add_actor(
            ActorType::Score1000,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
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
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = match general.actor_type {
            ActorType::KeyRed => OBJECT_KEY_RED,
            ActorType::KeyBlue => OBJECT_KEY_BLUE,
            ActorType::KeyPink => OBJECT_KEY_PINK,
            ActorType::KeyGreen => OBJECT_KEY_GREEN,
            _ => unreachable!(),
        };

        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            target,
            Some(general.position),
        );
    }
}
