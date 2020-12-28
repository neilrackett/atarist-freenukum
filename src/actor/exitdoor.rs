use crate::{
    actor::{
        ActParameters, Actor, CreateActor, HeroInteractStartParameters,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles, PlayState},
    Hero, Result, Sizes, ANIMATION_EXITDOOR,
};
use sdl2::rect::{Point, Rect};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Closed,
    Opening,
    Closing,
}

#[derive(Debug)]
pub(crate) struct ExitDoor {
    tile: usize,
    counter: usize,
    state: State,
    position: Rect,
}

impl CreateActor for ExitDoor {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        Self {
            tile: ANIMATION_EXITDOOR,
            counter: 0,
            state: State::Closed,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width() * 2,
                sizes.height() * 2,
            ),
        }
    }
}

impl Actor for ExitDoor {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, _p: HeroInteractStartParameters) {
        if self.state == State::Closed {
            self.state = State::Opening;
        }
    }

    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Closed => {}
            State::Opening => {
                self.counter += 1;
                if self.counter >= 4 {
                    p.hero.hidden = true;
                    self.state = State::Closing;
                    self.counter -= 1;
                }
            }
            State::Closing => {
                if self.counter == 0 {
                    *p.play_state = PlayState::LevelFinished;
                    p.hero.hidden = false;
                } else {
                    self.counter -= 1;
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();
        p.renderer.place_tile(self.tile + self.counter * 4, pos)?;
        pos.x += p.sizes.width() as i32;
        p.renderer
            .place_tile(self.tile + self.counter * 4 + 1, pos)?;
        pos.x -= p.sizes.width() as i32;
        pos.y += p.sizes.height() as i32;
        p.renderer
            .place_tile(self.tile + self.counter * 4 + 2, pos)?;
        pos.x += p.sizes.width() as i32;
        p.renderer
            .place_tile(self.tile + self.counter * 4 + 3, pos)?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
