use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        println!(
            "Warning: creating placeholder for unimplemented \
                actor type {:?} at {:?}",
            general.actor_type, pos
        );
        Specific {
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {}

    fn render(&mut self, _p: RenderParameters) -> Result<()> {
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        false
    }
}
