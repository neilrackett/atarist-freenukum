use crate::{
    actor::{
        ActParameters, ActorInterface, ActorType, CreateActor,
        RenderParameters, SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_MINE, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            tile: ANIMATION_MINE,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            is_alive: true,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        if !p.solids.get(
            self.position.x() as u32 / TILE_WIDTH,
            self.position.y() as u32 / TILE_HEIGHT + 1,
        ) {
            self.position.offset(0, HALFTILE_HEIGHT as i32);
        }

        if p.hero.position.geometry.has_intersection(self.position) {
            self.is_alive = false;
            p.actor_adder.add_actor(
                ActorType::SingleAnimation(SingleAnimationType::BombFire),
                self.position.top_left(),
            );
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}
