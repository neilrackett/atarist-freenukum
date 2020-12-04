use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        HeroInteractEndParameters, HeroInteractStartParameters,
        RenderParameters,
    },
    geometry::RectExt,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, HALFTILE_HEIGHT, OBJECT_ELEVATOR_TOP, SOLID_ELEVATOR,
    TILE_HEIGHT, TILE_WIDTH,
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
        // TODO: check whether we *really* need the double width
        general.position.resize(TILE_WIDTH * 2, TILE_HEIGHT);
        general.is_in_foreground = true;

        Specific { state: State::Idle }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let hero_geometry = p.hero_data.position.geometry;

        if self.state == State::Ascending
            || self.state == State::Idle
                && p.general.position.height() as u32 > TILE_HEIGHT
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
                    p.general.position.x() as u32 / TILE_WIDTH,
                    p.general.position.y() as u32 / TILE_HEIGHT - 3,
                ) {
                    // hero touches solid with head
                    self.state = State::Idle;
                } else {
                    let offset = p
                        .hero_data
                        .position
                        .push_vertically(&p.solids, -(TILE_HEIGHT as i32));
                    if -offset < TILE_HEIGHT as i32 {
                        p.hero_data
                            .position
                            .push_vertically(&p.solids, -offset);
                        self.state = State::Idle;
                    } else {
                        p.general.position.offset(0, offset);
                        p.general.position.set_height(
                            p.general.position.height() + (-offset) as u32,
                        );

                        p.solids.set(
                            p.general.position.x() as u32 / TILE_WIDTH,
                            p.general.position.y() as u32 / TILE_HEIGHT,
                            true,
                        );
                    }
                }
            }
            State::Descending => {
                for _ in 0..2 {
                    if p.general.position.height() as u32 > TILE_HEIGHT {
                        p.solids.set(
                            p.general.position.x() as u32 / TILE_WIDTH,
                            p.general.position.y() as u32 / TILE_HEIGHT,
                            false,
                        );
                        p.general.position.offset(0, TILE_HEIGHT as i32);
                        p.general.position.set_height(
                            p.general.position.height() - TILE_HEIGHT,
                        );
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
            && p.hero_data.position.geometry.bottom()
                == p.general.position.top()
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

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = SOLID_ELEVATOR;
        let mut pos = p.general.position.top_left();
        for _ in 0..(p.general.position.height() / TILE_HEIGHT - 1) * 2 {
            pos.y += HALFTILE_HEIGHT as i32;
            p.renderer.place_tile(tile, pos)?;
        }
        pos = p.general.position.top_left();
        let tile = OBJECT_ELEVATOR_TOP;
        p.renderer.place_tile(tile, pos)?;
        Ok(())
    }
}
