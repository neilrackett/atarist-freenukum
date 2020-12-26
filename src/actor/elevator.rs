use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        HeroInteractEndParameters, HeroInteractStartParameters,
        RenderParameters,
    },
    geometry::RectExt,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, HALFTILE_HEIGHT, OBJECT_ELEVATOR_TOP, SOLID_ELEVATOR,
    TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Idle,
    Ascending,
    Descending,
}

#[derive(Debug)]
pub(crate) struct Specific {
    state: State,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.is_in_foreground = true;
        general.acts_while_invisible = true;

        // TODO: check whether we *really* need the double width
        Specific {
            state: State::Idle,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH * 2, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let hero_geometry = p.hero.position.geometry;

        if self.state == State::Ascending
            || self.state == State::Idle
                && self.position.height() as u32 > TILE_HEIGHT
        {
            // check if hero leaves elevator
            if !hero_geometry.touches(self.position)
                || self.position.x != hero_geometry.x
            {
                self.state = State::Descending;
            }
        }

        match self.state {
            State::Ascending => {
                if p.solids.get(
                    self.position.x() as u32 / TILE_WIDTH,
                    self.position.y() as u32 / TILE_HEIGHT - 3,
                ) {
                    // hero touches solid with head
                    self.state = State::Idle;
                } else {
                    let offset = p
                        .hero
                        .position
                        .push_vertically(&p.solids, -(TILE_HEIGHT as i32));
                    if -offset < TILE_HEIGHT as i32 {
                        p.hero
                            .position
                            .push_vertically(&p.solids, -offset);
                        self.state = State::Idle;
                    } else {
                        self.position.offset(0, offset);
                        self.position.set_height(
                            self.position.height() + (-offset) as u32,
                        );

                        p.solids.set(
                            self.position.x() as u32 / TILE_WIDTH,
                            self.position.y() as u32 / TILE_HEIGHT,
                            true,
                        );
                    }
                }
            }
            State::Descending => {
                for _ in 0..2 {
                    if self.position.height() as u32 > TILE_HEIGHT {
                        p.solids.set(
                            self.position.x() as u32 / TILE_WIDTH,
                            self.position.y() as u32 / TILE_HEIGHT,
                            false,
                        );
                        self.position.offset(0, TILE_HEIGHT as i32);
                        self.position.set_height(
                            self.position.height() - TILE_HEIGHT,
                        );
                    } else {
                        self.state = State::Idle;
                    }
                }
            }
            State::Idle => {}
        }
    }

    fn hero_can_interact(&self, hero: &Hero) -> bool {
        self.position.x() == hero.position.geometry.x()
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        if p.hero.position.geometry.touches(self.position)
            && p.hero.position.geometry.bottom() == self.position.top()
        {
            self.state = State::Ascending;
        }
    }

    fn hero_interact_end(&mut self, p: HeroInteractEndParameters) {
        if p.hero.position.geometry.touches(self.position)
            && p.hero.position.geometry.x == self.position.x
        {
            self.state = State::Idle;
        } else {
            self.state = State::Descending;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = SOLID_ELEVATOR;
        let mut pos = self.position.top_left();
        for _ in 0..(self.position.height() / TILE_HEIGHT - 1) * 2 {
            pos.y += HALFTILE_HEIGHT as i32;
            p.renderer.place_tile(tile, pos)?;
        }
        pos = self.position.top_left();
        let tile = OBJECT_ELEVATOR_TOP;
        p.renderer.place_tile(tile, pos)?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }
}
