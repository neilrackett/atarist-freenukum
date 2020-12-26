use crate::{
    actor::{
        ActParameters, ActorInterface, CreateActor, RenderParameters,
        ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, HALFTILE_HEIGHT, OBJECT_ROCKET, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Flying,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Specific {
    state: State,
    position: Rect,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Specific {
        let tile_x = pos.x as u32 / TILE_WIDTH;
        let tile_y = pos.y as u32 / TILE_HEIGHT;
        tiles.copy_from_to(tile_x, tile_y - 1, tile_x, tile_y);

        Specific {
            state: State::Idle,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Idle => {}
            State::Flying => {
                self.position.offset(0, -(HALFTILE_HEIGHT as i32));
                if p.solids.collides(self.position) {
                    let tile_x = self.position.x() as u32 / TILE_WIDTH;
                    let tile_y = self.position.y() as u32 / TILE_HEIGHT;
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
        let mut pos = self
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

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        if self.state == State::Idle {
            // TODO: create animation
            self.state = State::Flying;
            let tile_x = self.position.x() as u32 / TILE_WIDTH;
            let tile_y = (self.position.y() as u32
                + self.position.height())
                / TILE_HEIGHT;

            p.solids.set(tile_x, tile_y, false);
            // TODO: trigger a re-rendering of the affected tiles
            p.tiles.copy_from_to(tile_x, tile_y + 1, tile_x, tile_y);
        }
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
