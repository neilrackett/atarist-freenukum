use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, ScoreType, SingleAnimationType,
    },
    level::tiles::{LevelTiles, Tile},
    Result, Sizes, ANIMATION_SODAFLY,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct SodaFlying {
    position: Rect,
    is_alive: bool,
}

impl CreateActor for SodaFlying {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::SodaFlying(Self {
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
        })
    }
}

impl ActorExt for SodaFlying {
    fn act(&mut self, p: ActParameters) {
        self.position.offset(0, -(p.sizes.half_height() as i32));
        if let Ok(Tile { solid: true, .. }) = p.tiles.get(
            self.position.x() / p.sizes.width() as i32,
            self.position.y() / p.sizes.height() as i32,
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
            + ((self.position.y() as usize
                / p.sizes.half_height() as usize)
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
