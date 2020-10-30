use super::super::super::hero::HeroData;
use super::super::super::level::solids::LevelSolids;
use super::super::super::level::tiles::LevelTiles;
use super::super::super::tilecache::TileCache;
use super::{ActorAdder, ActorCreateInterface, ActorData, ActorInterface};
use crate::{HALFTILE_HEIGHT, OBJECT_ROCKET, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Flying,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Specific {
    state: State,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let tile_x = general.position.x as usize / TILE_WIDTH;
        let tile_y = general.position.y as usize / TILE_HEIGHT;
        tiles.copy_from_to(tile_x, tile_y - 1, tile_x, tile_y);

        Specific { state: State::Idle }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        match self.state {
            State::Idle => {}
            State::Flying => {
                general.position.y -= HALFTILE_HEIGHT as i16;
                if solids.collides(general.position) {
                    let tile_x = general.position.x as usize / TILE_WIDTH;
                    let tile_y = general.position.y as usize / TILE_HEIGHT;
                    solids.set(tile_x, tile_y + 1, false);
                    // TODO: trigger a re-rendering of the affected tiles
                    tiles.copy_from_to(tile_x, tile_y - 1, tile_x, tile_y);
                }
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
        let mut destrect = general.position;
        destrect.y -= TILE_HEIGHT as i16 * 3;

        let tile = tilecache.get_tile(OBJECT_ROCKET).unwrap();
        tile.blit_to_sdl_surface(None, target, Some(destrect));

        let tile = tilecache.get_tile(OBJECT_ROCKET + 1).unwrap();
        for _ in 0..2 {
            destrect.y += TILE_HEIGHT as i16;
            tile.blit_to_sdl_surface(None, target, Some(destrect));
        }

        let tile = tilecache.get_tile(OBJECT_ROCKET + 2).unwrap();
        destrect.y += TILE_HEIGHT as i16;
        tile.blit_to_sdl_surface(None, target, Some(destrect));

        let tile = tilecache.get_tile(OBJECT_ROCKET + 3).unwrap();
        destrect.x -= TILE_WIDTH as i16;
        tile.blit_to_sdl_surface(None, target, Some(destrect));

        let tile = tilecache.get_tile(OBJECT_ROCKET + 4).unwrap();
        destrect.x += 2 * TILE_WIDTH as i16;
        tile.blit_to_sdl_surface(None, target, Some(destrect));

        if self.state == State::Flying {
            let tile = tilecache.get_tile(OBJECT_ROCKET + 6).unwrap();
            destrect.x -= TILE_WIDTH as i16;
            destrect.y += TILE_HEIGHT as i16;
            tile.blit_to_sdl_surface(None, target, Some(destrect));
        }
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        if self.state == State::Idle {
            // TODO: create animation
            self.state = State::Flying;
            let tile_x = general.position.x as usize / TILE_WIDTH;
            let tile_y = (general.position.y as usize
                + general.position.h as usize)
                / TILE_HEIGHT;

            solids.set(tile_x, tile_y, false);
            // TODO: trigger a re-rendering of the affected tiles
            tiles.copy_from_to(tile_x, tile_y + 1, tile_x, tile_y);
        }
    }
}
