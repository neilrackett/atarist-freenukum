use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    RenderParameters, ShotParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{HALFTILE_HEIGHT, OBJECT_ROCKET, TILE_HEIGHT, TILE_WIDTH};

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
    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Idle => {}
            State::Flying => {
                p.general.position.y -= HALFTILE_HEIGHT as i16;
                if p.solids.collides(p.general.position) {
                    let tile_x =
                        p.general.position.x as usize / TILE_WIDTH;
                    let tile_y =
                        p.general.position.y as usize / TILE_HEIGHT;
                    p.solids.set(tile_x, tile_y + 1, false);
                    // TODO: trigger a re-rendering of the affected tiles
                    p.tiles.copy_from_to(
                        tile_x,
                        tile_y - 1,
                        tile_x,
                        tile_y,
                    );
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let mut destrect = p.general.position;
        destrect.y -= TILE_HEIGHT as i16 * 3;

        let tile = OBJECT_ROCKET;
        p.renderer.place_tile(tile, destrect);

        let tile = OBJECT_ROCKET + 1;
        for _ in 0..2 {
            destrect.y += TILE_HEIGHT as i16;
            p.renderer.place_tile(tile, destrect);
        }

        let tile = OBJECT_ROCKET + 2;
        destrect.y += TILE_HEIGHT as i16;
        p.renderer.place_tile(tile, destrect);

        let tile = OBJECT_ROCKET + 3;
        destrect.x -= TILE_WIDTH as i16;
        p.renderer.place_tile(tile, destrect);

        let tile = OBJECT_ROCKET + 4;
        destrect.x += 2 * TILE_WIDTH as i16;
        p.renderer.place_tile(tile, destrect);

        if self.state == State::Flying {
            let tile = OBJECT_ROCKET + 6;
            destrect.x -= TILE_WIDTH as i16;
            destrect.y += TILE_HEIGHT as i16;
            p.renderer.place_tile(tile, destrect);
        }
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        if self.state == State::Idle {
            // TODO: create animation
            self.state = State::Flying;
            let tile_x = p.general.position.x as usize / TILE_WIDTH;
            let tile_y = (p.general.position.y as usize
                + p.general.position.h as usize)
                / TILE_HEIGHT;

            p.solids.set(tile_x, tile_y, false);
            // TODO: trigger a re-rendering of the affected tiles
            p.tiles.copy_from_to(tile_x, tile_y + 1, tile_x, tile_y);
        }
    }
}
