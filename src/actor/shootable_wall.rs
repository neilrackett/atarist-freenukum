use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActor, RenderParameters,
        ShotParameters, ShotProcessing, SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, Sizes, BACKGROUND_LIGHT_GREY, SOLID_SHOOTABLE_WALL_BRICKS,
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
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, _p: ActParameters) {}

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.hero.score.add(10);
        p.actor_adder.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Explosion),
            self.position.top_left(),
        );
        self.is_alive = false;
        p.solids.set(
            self.position.x() as u32 / p.sizes.width(),
            self.position.y() as u32 / p.sizes.height(),
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
