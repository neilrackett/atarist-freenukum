use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageQueue,
};
use crate::hero::HeroData;
use crate::infobox::InfoMessageQueue;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{OBJECT_NOTEBOOK, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;
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

    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(
        &mut self,
        _general: &mut ActorData,
        _level_passed: &mut bool,
        _hero_data: &mut HeroData,
        info_message_queue: &mut InfoMessageQueue,
        _actor_message_queue: &mut ActorMessageQueue,
    ) {
        // TODO: implement functionality.
        info_message_queue.push_back("Not implemented yet.".to_string());
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        tilecache
            .get_tile(OBJECT_NOTEBOOK)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }
}
