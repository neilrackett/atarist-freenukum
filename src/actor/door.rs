use crate::{
    actor::{
        ActParameters, Actor, ActorMessageType, CreateActorWithDetails,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{
        solids::LevelSolids, tiles::LevelTiles, BackgroundTileStrategy,
    },
    KeyColor, Result, Sizes, OBJECT_DOOR,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Closed,
    Opening,
    Open,
}

#[derive(Debug)]
pub(crate) struct Door {
    tile: usize,
    counter: usize,
    state: State,
    position: Rect,
    color: KeyColor,
}

impl CreateActorWithDetails for Door {
    type Details = KeyColor;

    fn create_with_details(
        color: KeyColor,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        Self {
            tile: OBJECT_DOOR,
            counter: 0,
            state: State::Closed,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            color,
        }
    }
}

impl Actor for Door {
    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Closed => {}
            State::Opening => {
                if self.counter == 0 {
                    p.solids.set(
                        self.position.x() / p.sizes.width() as i32,
                        self.position.y() / p.sizes.height() as i32,
                        false,
                    );
                }
                self.counter += 1;
                if self.counter == 8 {
                    self.state = State::Open;
                }
            }
            State::Open => {}
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.counter,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::OpenDoor(self.color) {
            return;
        }
        self.state = State::Opening;
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        self.state != State::Open
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }
}
