use crate::{
    actor::{
        ActParameters, Actor, ActorExt, CreateActorWithDetails,
        RenderParameters,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Hero, HorizontalDirection, Result, Sizes, OBJECT_FIRELEFT,
    OBJECT_FIRERIGHT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Off,
    Ignition,
    Burning,
}

#[derive(Debug)]
pub(crate) struct Fire {
    tile: usize,
    direction: HorizontalDirection,
    state: State,
    counter: usize,
    position: Rect,
}

impl CreateActorWithDetails for Fire {
    type Details = HorizontalDirection;

    fn create_with_details(
        direction: HorizontalDirection,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        let position =
            Rect::new(pos.x, pos.y, sizes.width(), sizes.height());
        let tile = match direction {
            HorizontalDirection::Right => OBJECT_FIRERIGHT,
            HorizontalDirection::Left => OBJECT_FIRELEFT,
        };

        Actor::Fire(Self {
            tile,
            direction,
            state: State::Off,
            counter: 0,
            position,
        })
    }
}

impl ActorExt for Fire {
    fn act(&mut self, _p: ActParameters) {
        match self.state {
            State::Off => {
                if self.counter == 40 {
                    self.counter = 0;
                    self.state = State::Ignition;
                }
            }
            State::Ignition => {
                if self.counter == 20 {
                    self.counter = 0;
                    self.state = State::Burning;
                }
            }
            State::Burning => {
                if self.counter == 20 {
                    self.counter = 0;
                    self.state = State::Off;
                }
            }
        }

        self.counter += 1;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let (tile0, tile1, tile2) = match self.state {
            State::Off => (None, None, None),
            State::Ignition => {
                if (self.counter % 2) > 0 {
                    match self.direction {
                        HorizontalDirection::Left => {
                            (None, None, Some(self.tile))
                        }
                        HorizontalDirection::Right => {
                            (Some(self.tile), None, None)
                        }
                    }
                } else {
                    (None, None, None)
                }
            }
            State::Burning => {
                let offset = self.counter % 2;
                match self.direction {
                    HorizontalDirection::Left => (
                        Some(self.tile + 3 + offset),
                        Some(self.tile + 1 + offset),
                        Some(self.tile + 1 + offset),
                    ),
                    HorizontalDirection::Right => (
                        Some(self.tile + 1 + offset),
                        Some(self.tile + 1 + offset),
                        Some(self.tile + 3 + offset),
                    ),
                }
            }
        };

        let mut pos = self.position.top_left();
        if let Some(tile) = tile0 {
            p.renderer.place_tile(tile, pos)?;
        }
        pos.x += p.sizes.width() as i32;
        if let Some(tile) = tile1 {
            p.renderer.place_tile(tile, pos)?;
        }
        pos.x += p.sizes.width() as i32;
        if let Some(tile) = tile2 {
            p.renderer.place_tile(tile, pos)?;
        }
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.state == State::Burning
            && self.position.has_intersection(hero.position.geometry)
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        match self.direction {
            HorizontalDirection::Right => {
                BackgroundTileStrategy::CopyFromRight
            }
            HorizontalDirection::Left => {
                BackgroundTileStrategy::CopyFromLeft
            }
        }
    }
}
