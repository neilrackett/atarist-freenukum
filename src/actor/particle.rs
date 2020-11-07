use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    HALFTILE_HEIGHT, HALFTILE_WIDTH, OBJECT_SPARK_BLUE,
    OBJECT_SPARK_GREEN, OBJECT_SPARK_PINK, OBJECT_SPARK_WHITE,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    countdown: usize,
    hspeed: i16,
    vspeed: i16,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.is_in_foreground = true;
        general.position.w = HALFTILE_WIDTH as u16;
        general.position.h = HALFTILE_HEIGHT as u16;

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
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        if self.countdown > 0 {
            self.countdown -= 1;
            p.general.position.x += self.hspeed;
            p.general.position.y += self.vspeed;
            self.vspeed += 2;
        } else {
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) {
        p.renderer.place_tile(self.tile, p.general.position);
    }
}
