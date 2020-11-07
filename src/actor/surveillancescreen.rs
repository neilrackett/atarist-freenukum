use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    HeroInteractStartParameters, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{ANIMATION_BADGUYSCREEN, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;

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

    fn render(&mut self, p: RenderParameters) {
        let mut destrect = p.general.position;
        p.tilecache
            .get_tile(ANIMATION_BADGUYSCREEN)
            .unwrap()
            .blit_to_sdl_surface(None, p.target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        p.tilecache
            .get_tile(ANIMATION_BADGUYSCREEN + 1)
            .unwrap()
            .blit_to_sdl_surface(None, p.target, Some(destrect));
    }
}
