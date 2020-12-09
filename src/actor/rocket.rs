use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, HALFTILE_HEIGHT, OBJECT_ROCKET, TILE_HEIGHT, TILE_WIDTH,
};

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
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);

        let tile_x = general.position.x() as u32 / TILE_WIDTH;
        let tile_y = general.position.y() as u32 / TILE_HEIGHT;
        tiles.copy_from_to(tile_x, tile_y - 1, tile_x, tile_y);

        Specific { state: State::Idle }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Idle => {}
            State::Flying => {
                p.general.position.offset(0, -(HALFTILE_HEIGHT as i32));
                if p.solids.collides(p.general.position) {
                    let tile_x =
                        p.general.position.x() as u32 / TILE_WIDTH;
                    let tile_y =
                        p.general.position.y() as u32 / TILE_HEIGHT;
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

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p
            .general
            .position
            .top_left()
            .offset(0, -(TILE_HEIGHT as i32 * 3));

        let tile = OBJECT_ROCKET;
        p.renderer.place_tile(tile, pos)?;

        let tile = OBJECT_ROCKET + 1;
        for _ in 0..2 {
            pos.y += TILE_HEIGHT as i32;
            p.renderer.place_tile(tile, pos)?;
        }

        let tile = OBJECT_ROCKET + 2;
        pos.y += TILE_HEIGHT as i32;
        p.renderer.place_tile(tile, pos)?;

        let tile = OBJECT_ROCKET + 3;
        pos.x -= TILE_WIDTH as i32;
        p.renderer.place_tile(tile, pos)?;

        let tile = OBJECT_ROCKET + 4;
        pos.x += 2 * TILE_WIDTH as i32;
        p.renderer.place_tile(tile, pos)?;

        if self.state == State::Flying {
            let tile = OBJECT_ROCKET + 6;
            pos.x -= TILE_WIDTH as i32;
            pos.y += TILE_HEIGHT as i32;
            p.renderer.place_tile(tile, pos)?;
        }
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        if self.state == State::Idle {
            // TODO: create animation
            self.state = State::Flying;
            let tile_x = p.general.position.x() as u32 / TILE_WIDTH;
            let tile_y = (p.general.position.y() as u32
                + p.general.position.height())
                / TILE_HEIGHT;

            p.solids.set(tile_x, tile_y, false);
            // TODO: trigger a re-rendering of the affected tiles
            p.tiles.copy_from_to(tile_x, tile_y + 1, tile_x, tile_y);
        }
        ShotProcessing::Absorb
    }
}
