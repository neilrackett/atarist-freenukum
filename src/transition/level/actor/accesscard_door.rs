use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::solids::LevelSolids;
use super::super::tiles::LevelTiles;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageType,
};
use crate::{OBJECT_LASERBEAM, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        Specific {
            tile: OBJECT_LASERBEAM,
            current_frame: 0,
            num_frames: 4,
        }
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
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        tilecache
            .get_tile(self.tile + self.current_frame)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }

    fn receive_message(
        &mut self,
        general: &mut ActorData,
        message: ActorMessageType,
        _hero_data: &mut HeroData,
        solids: &mut LevelSolids,
    ) {
        if message != ActorMessageType::OpenDoor {
            return;
        }
        let x = general.position.x as usize / TILE_WIDTH;
        let y = general.position.y as usize / TILE_HEIGHT;
        solids.set(x, y, false);
        general.is_alive = false;
    }
}
