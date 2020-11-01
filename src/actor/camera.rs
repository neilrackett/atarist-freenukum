use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{
    ANIMATION_CAMERA_CENTER, ANIMATION_CAMERA_LEFT,
    ANIMATION_CAMERA_RIGHT, TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self {
        general.is_in_foreground = false;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let x = general.position.x as usize / TILE_WIDTH;
        let y = general.position.y as usize / TILE_HEIGHT;
        tiles.copy_from_to(x, y + 1, x, y);

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
        general: &mut ActorData,
        hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let x = hero_data.position.geometry.x;
        let tile = if x - 1 > general.position.x {
            ANIMATION_CAMERA_RIGHT
        } else if x + 1 < general.position.x {
            ANIMATION_CAMERA_LEFT
        } else {
            ANIMATION_CAMERA_CENTER
        };

        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            target,
            Some(general.position),
        );
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
    ) {
        general.is_alive = false;
        hero_data.score.add(100);
        actor_adder.add_actor(
            ActorType::Score100,
            general.position.x as u16,
            general.position.y as u16,
        );
        actor_adder.add_actor(
            ActorType::Explosion,
            general.position.x as u16,
            general.position.y as u16,
        );
    }
}
