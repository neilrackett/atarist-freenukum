use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result,
};

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
    fn act(&mut self, _p: ActParameters) {}

    fn render(&mut self, _p: RenderParameters) -> Result<()> {
        Ok(())
    }
}
