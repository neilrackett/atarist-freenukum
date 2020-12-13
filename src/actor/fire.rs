use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    HorizontalDirection, Result, LEVEL_WIDTH, OBJECT_FIRELEFT,
    OBJECT_FIRERIGHT, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Off,
    Ignition,
    Burning,
}

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    direction: HorizontalDirection,
    state: State,
    counter: usize,
    touching_hero: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = true;

        let x = general.position.x() as u32 / TILE_WIDTH;
        let y = general.position.y() as u32 / TILE_HEIGHT;

        let (tile, direction) = match general.actor_type {
            ActorType::FireRight => {
                if x < LEVEL_WIDTH + 1 {
                    tiles.copy_from_to(x + 1, y, x, y);
                }
                (OBJECT_FIRERIGHT, HorizontalDirection::Right)
            }
            ActorType::FireLeft => {
                if x > 0 {
                    tiles.copy_from_to(x - 1, y, x, y);
                }
                general.position.offset(-2 * TILE_WIDTH as i32, 0);
                (OBJECT_FIRELEFT, HorizontalDirection::Left)
            }
            _ => unreachable!(),
        };

        Specific {
            tile,
            direction,
            state: State::Off,
            counter: 0,
            touching_hero: false,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.general.hurts_hero = self.state == State::Burning;
        self.touching_hero = true;
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        p.general.hurts_hero = false;
        self.touching_hero = false;
    }

    fn act(&mut self, p: ActParameters) {
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
                    if self.touching_hero {
                        p.general.hurts_hero = true;
                    }
                }
            }
            State::Burning => {
                if self.counter == 20 {
                    self.counter = 0;
                    self.state = State::Off;
                    if self.touching_hero {
                        p.general.hurts_hero = false;
                    }
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
                        HorizontalDirection::Center => unreachable!(),
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
                    HorizontalDirection::Center => unreachable!(),
                }
            }
        };

        let mut pos = p.general.position.top_left();
        if let Some(tile) = tile0 {
            p.renderer.place_tile(tile, pos)?;
        }
        pos.x += TILE_WIDTH as i32;
        if let Some(tile) = tile1 {
            p.renderer.place_tile(tile, pos)?;
        }
        pos.x += TILE_WIDTH as i32;
        if let Some(tile) = tile2 {
            p.renderer.place_tile(tile, pos)?;
        }
        Ok(())
    }
}
