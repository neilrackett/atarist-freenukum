use crate::{
    actor::{
        ActParameters, Actor, ActorMessageType, CreateActor,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, Sizes, SOLID_EXPANDINGFLOOR,
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
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            expanding: false,
            finished: false,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, p: ActParameters) {
        if self.expanding {
            let x = self.position.right() as u32 / p.sizes.width();
            let y = self.position.top() as u32 / p.sizes.height();
            let can_expand = !p.solids.get(x, y);
            if can_expand {
                p.solids.set(x, y, true);
                self.position
                    .set_width(self.position.width() + p.sizes.width());
            } else {
                self.expanding = false;
                self.finished = true;
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = SOLID_EXPANDINGFLOOR;
        let mut pos = self.position.top_left();
        for _ in 0..self.position.width() as u32 / p.sizes.width() {
            p.renderer.place_tile(tile, pos)?;
            pos.x += p.sizes.width() as i32;
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
