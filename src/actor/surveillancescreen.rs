use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        HeroInteractStartParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_BADGUYSCREEN, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH * 2, TILE_HEIGHT);
        Specific {}
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        // TODO: implement functionality.
        p.info_message_queue
            .push_back("Not implemented yet.".to_string());
    }

    fn act(&mut self, _p: ActParameters) {}

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p.general.position.top_left();
        p.renderer.place_tile(ANIMATION_BADGUYSCREEN, pos)?;
        pos = pos.offset(TILE_WIDTH as i32, 0);
        p.renderer.place_tile(ANIMATION_BADGUYSCREEN + 1, pos)?;
        Ok(())
    }
}
