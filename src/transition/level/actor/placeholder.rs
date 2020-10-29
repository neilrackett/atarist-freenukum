use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorAdder, ActorCreateInterface, ActorData, ActorInterface};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        println!(
            "Warning: creating placeholder for unimplemented \
                actor type {:?}",
            general.actor_type
        );
        general.is_alive = false;
        Specific {}
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
    }

    fn blit(
        &mut self,
        _general: &mut ActorData,
        _hero_data: &mut HeroData,
        _tilecache: &TileCache,
        _target: &mut Surface,
    ) {
    }
}
