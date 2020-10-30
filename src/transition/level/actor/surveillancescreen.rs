use super::super::super::hero::HeroData;
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::solids::LevelSolids;
use super::super::tiles::LevelTiles;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageQueue,
};
use crate::{ANIMATION_BADGUYSCREEN, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

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
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let mut destrect = general.position;
        tilecache
            .get_tile(ANIMATION_BADGUYSCREEN)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(ANIMATION_BADGUYSCREEN + 1)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
    }
}
