use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{ANIMATION_MINE, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    counter: u16,
    base_y: u16,
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
            counter: 0,
            base_y: general.position.y as u16,
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
    }

    fn hero_touch_end(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = false;
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        let distance = match self.counter {
            0 => 0,
            1 | 11 => 16,
            2 | 10 => 28,
            3 | 9 => 36,
            4 | 8 => 40,
            5 | 7 => 41,
            6 => 42,
            _ => unreachable!(),
        };
        general.position.y = self.base_y as i16 - distance as i16;

        self.counter += 1;
        self.counter %= 12;
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

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        /*
         * We don't need to do anything, this is just to absorb
         * the bullet when the actor is shot.
         */
        true
    }
}
