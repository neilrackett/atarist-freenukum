use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        HeroInteractStartParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_NOTEBOOK, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = false;
        Specific {}
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {}

    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        // TODO: implement functionality.
        p.info_message_queue
            .push_back("Not implemented yet.".to_string());
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer
            .place_tile(OBJECT_NOTEBOOK, p.general.position.top_left())?;
        Ok(())
    }
}
