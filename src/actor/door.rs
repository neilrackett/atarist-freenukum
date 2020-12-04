use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_DOOR, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Closed,
    Opening,
    Open,
}

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    counter: usize,
    state: State,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);

        Specific {
            tile: OBJECT_DOOR,
            counter: 0,
            state: State::Closed,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Closed => {}
            State::Opening => {
                if self.counter == 0 {
                    p.solids.set(
                        p.general.position.x() as u32 / TILE_WIDTH,
                        p.general.position.y() as u32 / TILE_HEIGHT,
                        false,
                    );
                }
                self.counter += 1;
                if self.counter == 8 {
                    self.state = State::Open;
                    p.general.is_alive = false;
                }
            }
            State::Open => {}
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.counter,
            p.general.position.top_left(),
        )?;
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::OpenDoor {
            return;
        }
        self.state = State::Opening;
    }
}
