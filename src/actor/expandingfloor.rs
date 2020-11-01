use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{SOLID_EXPANDINGFLOOR, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    expanding: bool,
    finished: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {
            expanding: false,
            finished: false,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_queue: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        if self.expanding {
            let x = (general.position.x as usize
                + general.position.w as usize)
                / TILE_WIDTH;
            let y = general.position.y as usize / TILE_HEIGHT;
            let can_expand = !solids.get(x, y);
            if can_expand {
                solids.set(x, y, true);
                general.position.w += TILE_WIDTH as u16;
            } else {
                self.expanding = false;
                self.finished = true;
            }
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile =
            tilecache.get_tile(SOLID_EXPANDINGFLOOR as usize).unwrap();
        let mut destrect = general.position;
        for _ in 0..general.position.w as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }

    fn receive_message(
        &mut self,
        _general: &mut ActorData,
        message: ActorMessageType,
        _hero_data: &mut HeroData,
        _solids: &mut LevelSolids,
    ) {
        if message != ActorMessageType::Expand {
            return;
        }
        if !self.expanding && !self.finished {
            self.expanding = true;
        }
    }
}
