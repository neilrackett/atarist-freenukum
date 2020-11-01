use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
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
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
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
