use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActor,
        RenderParameters, ScoreType, SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_SODAFLY, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
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
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            is_alive: true,
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, p: ActParameters) {
        self.position.offset(0, -(HALFTILE_HEIGHT as i32));
        if p.solids.get(
            self.position.x() as u32 / TILE_WIDTH,
            self.position.y() as u32 / TILE_HEIGHT,
        ) {
            p.actor_adder.add_actor(
                ActorType::SingleAnimation(SingleAnimationType::Explosion),
                self.position.top_left(),
            );
            self.is_alive = false;
        } else if self.position.has_intersection(p.hero.position.geometry)
        {
            p.hero.score.add(1000);
            p.actor_adder.add_actor(
                ActorType::Score(ScoreType::Score1000),
                self.position.top_left(),
            );
            self.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = ANIMATION_SODAFLY
            + ((self.position.y() as usize / HALFTILE_HEIGHT as usize)
                % 4);
        p.renderer.place_tile(tile, self.position.top_left())?;
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
