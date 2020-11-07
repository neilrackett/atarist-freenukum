use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    HeroInteractEndParameters, HeroInteractStartParameters,
    RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    HALFTILE_HEIGHT, OBJECT_ELEVATOR_TOP, SOLID_ELEVATOR, TILE_HEIGHT,
    TILE_WIDTH,
};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Idle,
    Ascending,
    Descending,
}

#[derive(Debug)]
pub(crate) struct Specific {
    state: State,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        Specific { state: State::Idle }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let hero_geometry = p.hero_data.position.geometry;

        if self.state == State::Ascending
            || self.state == State::Idle
                && p.general.position.h as usize > TILE_HEIGHT
        {
            // check if hero leaves elevator
            if !hero_geometry.touches(p.general.position)
                || p.general.position.x != hero_geometry.x
            {
                self.state = State::Descending;
            }
        }

        match self.state {
            State::Ascending => {
                if p.solids.get(
                    p.general.position.x as usize / TILE_WIDTH,
                    p.general.position.y as usize / TILE_HEIGHT - 3,
                ) {
                    // hero touches solid with head
                    self.state = State::Idle;
                } else {
                    let offset = p
                        .hero_data
                        .position
                        .push_vertically(&p.solids, -(TILE_HEIGHT as i16));
                    if -offset < TILE_HEIGHT as i16 {
                        p.hero_data
                            .position
                            .push_vertically(&p.solids, -offset);
                        self.state = State::Idle;
                    } else {
                        p.general.position.h += (-offset) as u16;
                        p.general.position.y += offset as i16;

                        p.solids.set(
                            p.general.position.x as usize / TILE_WIDTH,
                            p.general.position.y as usize / TILE_HEIGHT,
                            true,
                        );
                    }
                }
            }
            State::Descending => {
                for _ in 0..2 {
                    if p.general.position.h as usize > TILE_HEIGHT {
                        p.solids.set(
                            p.general.position.x as usize / TILE_WIDTH,
                            p.general.position.y as usize / TILE_HEIGHT,
                            false,
                        );
                        p.general.position.y += TILE_HEIGHT as i16;
                        p.general.position.h -= TILE_HEIGHT as u16;
                    } else {
                        self.state = State::Idle;
                    }
                }
            }
            State::Idle => {}
        }
    }

    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        if p.hero_data.position.geometry.touches(p.general.position)
            && p.hero_data.position.geometry.y
                + p.hero_data.position.geometry.h as i16
                == p.general.position.y
        {
            self.state = State::Ascending;
        }
    }

    fn hero_interact_end(&mut self, p: HeroInteractEndParameters) {
        if p.hero_data.position.geometry.touches(p.general.position)
            && p.hero_data.position.geometry.x == p.general.position.x
        {
            self.state = State::Idle;
        } else {
            self.state = State::Descending;
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let tile = SOLID_ELEVATOR;
        let mut destrect = p.general.position;
        for _ in 0..(p.general.position.h as usize / TILE_HEIGHT - 1) * 2 {
            destrect.y += HALFTILE_HEIGHT as i16;
            p.renderer.place_tile(tile, destrect);
        }
        destrect = p.general.position;
        let tile = OBJECT_ELEVATOR_TOP;
        p.renderer.place_tile(tile, destrect);
    }
}
