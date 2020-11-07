use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    HeroTouchEndParameters, HeroTouchStartParameters, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{ANIMATION_MINE, TILE_HEIGHT, TILE_WIDTH};

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
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.general.hurts_hero = true;
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        p.general.hurts_hero = false;
    }

    fn act(&mut self, p: ActParameters) {
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
        p.general.position.y = self.base_y as i16 - distance as i16;

        self.counter += 1;
        self.counter %= 12;
    }

    fn render(&mut self, p: RenderParameters) {
        p.renderer.place_tile(self.tile, p.general.position);
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        /*
         * We don't need to do anything, this is just to absorb
         * the bullet when the actor is shot.
         */
        true
    }
}
