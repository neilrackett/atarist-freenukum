use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        HeroInteractStartParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_EXITDOOR, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Closed,
    Opening,
    Closing,
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
        general.position.resize(TILE_WIDTH * 2, TILE_HEIGHT * 2);
        general.is_in_foreground = false;

        Specific {
            tile: ANIMATION_EXITDOOR,
            counter: 0,
            state: State::Closed,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        if self.state == State::Closed {
            self.state = State::Opening;
        }
        *p.level_passed = true;
    }

    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Closed => {}
            State::Opening => {
                self.counter += 1;
                if self.counter >= 4 {
                    p.hero_data.hidden = true;
                    self.state = State::Closing;
                    self.counter -= 1;
                }
            }
            State::Closing => {
                if self.counter == 0 {
                    *p.do_play = false;
                    p.hero_data.hidden = false;
                } else {
                    self.counter -= 1;
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p.general.position.top_left();
        p.renderer.place_tile(self.tile + self.counter * 4, pos)?;
        pos.x += TILE_WIDTH as i32;
        p.renderer
            .place_tile(self.tile + self.counter * 4 + 1, pos)?;
        pos.x -= TILE_WIDTH as i32;
        pos.y += TILE_HEIGHT as i32;
        p.renderer
            .place_tile(self.tile + self.counter * 4 + 2, pos)?;
        pos.x += TILE_WIDTH as i32;
        p.renderer
            .place_tile(self.tile + self.counter * 4 + 3, pos)?;
        Ok(())
    }
}
