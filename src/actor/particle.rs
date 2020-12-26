use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, HALFTILE_HEIGHT, HALFTILE_WIDTH, OBJECT_SPARK_BLUE,
    OBJECT_SPARK_GREEN, OBJECT_SPARK_PINK, OBJECT_SPARK_WHITE,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    countdown: usize,
    hspeed: i32,
    vspeed: i32,
    position: Rect,
    is_alive: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.acts_while_invisible = true;

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let vspeed = rng.gen_range(-12, 5);
        let hspeed = rng.gen_range(-8, 9);

        let tile = match general.actor_type {
            ActorType::ParticlePink => OBJECT_SPARK_PINK,
            ActorType::ParticleBlue => OBJECT_SPARK_BLUE,
            ActorType::ParticleWhite => OBJECT_SPARK_WHITE,
            ActorType::ParticleGreen => OBJECT_SPARK_GREEN,
            _ => unreachable!(),
        };

        Specific {
            tile,
            countdown: 20,
            hspeed,
            vspeed,
            position: Rect::new(
                pos.x,
                pos.y,
                HALFTILE_WIDTH,
                HALFTILE_HEIGHT,
            ),
            is_alive: true,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {
        if self.countdown > 0 {
            self.countdown -= 1;
            self.position.offset(self.hspeed, self.vspeed);
            self.vspeed += 2;
        } else {
            self.is_alive = false;
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
