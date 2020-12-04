use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, SOLID_EXPANDINGFLOOR, TILE_HEIGHT, TILE_WIDTH,
};

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
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);

        Specific {
            expanding: false,
            finished: false,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        if self.expanding {
            let x = p.general.position.right() as u32 / TILE_WIDTH;
            let y = p.general.position.top() as u32 / TILE_HEIGHT;
            let can_expand = !p.solids.get(x, y);
            if can_expand {
                p.solids.set(x, y, true);
                p.general.position.set_right(
                    p.general.position.right() + TILE_WIDTH as i32,
                );
            } else {
                self.expanding = false;
                self.finished = true;
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = SOLID_EXPANDINGFLOOR;
        let mut pos = p.general.position.top_left();
        for _ in 0..p.general.position.width() as u32 / TILE_WIDTH {
            p.renderer.place_tile(tile, pos)?;
            pos.x += TILE_WIDTH as i32;
        }
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::Expand {
            return;
        }
        if !self.expanding && !self.finished {
            self.expanding = true;
        }
    }
}
