// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType, CreateActor,
        ReceiveMessageParameters, RenderParameters,
    },
    level::tiles::LevelTiles,
    Result, Sizes, SOLID_EXPANDINGFLOOR,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct ExpandingFloor {
    expanding: bool,
    finished: bool,
    position: Rect,
}

impl CreateActor for ExpandingFloor {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::ExpandingFloor(Self {
            expanding: false,
            finished: false,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for ExpandingFloor {
    fn act(&mut self, p: ActParameters) {
        if self.expanding {
            let x = self.position.right() / p.sizes.width() as i32;
            let y = self.position.top() / p.sizes.height() as i32;

            let can_expand =
                p.tiles.get(x, y).map(|t| !t.solid).unwrap_or(false);
            if can_expand {
                if let Ok(ref mut t) = p.tiles.get_mut(x, y) {
                    t.solid = true;
                }
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

    fn can_receive_message(&self, message: ActorMessageType) -> bool {
        message == ActorMessageType::ExpandFloor
    }

    fn receive_message(&mut self, _p: ReceiveMessageParameters) {
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
