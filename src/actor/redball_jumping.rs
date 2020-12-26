use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, ANIMATION_MINE, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    counter: u16,
    base_y: i32,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            tile: ANIMATION_MINE,
            counter: 0,
            base_y: pos.y,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {
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
        self.position.set_y(self.base_y - distance);

        self.counter += 1;
        self.counter %= 12;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        /*
         * We don't need to do anything, this is just to absorb
         * the bullet when the actor is shot.
         */
        true
    }

    fn shot(&mut self, _p: ShotParameters) -> ShotProcessing {
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.position.has_intersection(hero.position.geometry)
    }
}
