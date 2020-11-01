use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{ANIMATION_MINE, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    touching_hero: u8,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {
            tile: ANIMATION_MINE,
            touching_hero: 0,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = true;
        self.touching_hero = 1;
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        if !solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            general.position.y += HALFTILE_HEIGHT as i16;
        }

        match self.touching_hero {
            1 => self.touching_hero += 1,
            2 => {
                general.hurts_hero = false;
                general.is_alive = false;
                actor_adder.add_actor(
                    ActorType::BombFire,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            _ => {}
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = tilecache.get_tile(self.tile as usize).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}
