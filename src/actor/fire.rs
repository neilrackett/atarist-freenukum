use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
    RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::HorizontalDirection;
use crate::{OBJECT_FIRELEFT, OBJECT_FIRERIGHT, TILE_HEIGHT, TILE_WIDTH};

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
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16 * 3;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        let (tile, direction) = match general.actor_type {
            ActorType::FireRight => {
                (OBJECT_FIRERIGHT, HorizontalDirection::Right)
            }
            ActorType::FireLeft => {
                general.position.x -= 2 * TILE_WIDTH as i16;
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

    fn render(&mut self, p: RenderParameters) {
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

        let mut destrect = p.general.position;
        if let Some(tile) = tile0 {
            p.tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
                None,
                p.target,
                Some(destrect),
            );
        }
        destrect.x += TILE_WIDTH as i16;
        if let Some(tile) = tile1 {
            p.tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
                None,
                p.target,
                Some(destrect),
            );
        }
        destrect.x += TILE_WIDTH as i16;
        if let Some(tile) = tile2 {
            p.tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
                None,
                p.target,
                Some(destrect),
            );
        }
    }
}
