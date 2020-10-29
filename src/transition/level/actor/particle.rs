use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::{
    HALFTILE_HEIGHT, HALFTILE_WIDTH, OBJECT_SPARK_BLUE,
    OBJECT_SPARK_GREEN, OBJECT_SPARK_PINK, OBJECT_SPARK_WHITE,
};
use transdl::video::Surface;

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
        _level_data: &mut LevelData,
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
    fn act(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        if self.countdown > 0 {
            self.countdown -= 1;
            general.position.x += self.hspeed;
            general.position.y += self.vspeed;
            self.vspeed += 2;
        } else {
            general.is_alive = false;
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = tilecache.get_tile((self.tile) as usize).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}
