use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActor, RenderParameters,
        SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, Sizes, ANIMATION_MINE,
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
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            tile: ANIMATION_MINE,
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
    fn act(&mut self, p: ActParameters) {
        if !p.solids.get(
            self.position.x() as u32 / p.sizes.width(),
            self.position.y() as u32 / p.sizes.height() + 1,
        ) {
            self.position.offset(0, p.sizes.half_height() as i32);
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
