use crate::{
    actor::{
        ActParameters, Actor, ActorMessageType, CreateActor,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, SOLID_EXPANDINGFLOOR, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    expanding: bool,
    finished: bool,
    position: Rect,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            expanding: false,
            finished: false,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, p: ActParameters) {
        if self.expanding {
            let x = self.position.right() as u32 / TILE_WIDTH;
            let y = self.position.top() as u32 / TILE_HEIGHT;
            let can_expand = !p.solids.get(x, y);
            if can_expand {
                p.solids.set(x, y, true);
                self.position
                    .set_width(self.position.width() + TILE_WIDTH);
            } else {
                self.expanding = false;
                self.finished = true;
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = SOLID_EXPANDINGFLOOR;
        let mut pos = self.position.top_left();
        for _ in 0..self.position.width() as u32 / TILE_WIDTH {
            p.renderer.place_tile(tile, pos)?;
            pos.x += TILE_WIDTH as i32;
        }
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::ExpandFloor {
            return;
        }
        if !self.expanding && !self.finished {
            self.expanding = true;
        }
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
