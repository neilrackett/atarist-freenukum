use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, BACKGROUND_LIGHT_GREY, SOLID_SHOOTABLE_WALL_BRICKS,
    TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    position: Rect,
    is_alive: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            is_alive: true,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {}

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.hero.score.add(10);
        p.actor_adder
            .add_actor(ActorType::Explosion, self.position.top_left());
        self.is_alive = false;
        p.solids.set(
            self.position.x() as u32 / TILE_WIDTH,
            self.position.y() as u32 / TILE_HEIGHT,
            false,
        );
        ShotProcessing::Absorb
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer
            .place_tile(BACKGROUND_LIGHT_GREY, self.position.top_left())?;
        p.renderer.place_tile(
            SOLID_SHOOTABLE_WALL_BRICKS,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}
