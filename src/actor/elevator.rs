// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, CreateActor,
        HeroInteractEndParameters, HeroInteractStartParameters,
        RenderParameters,
    },
    geometry::RectExt,
    level::tiles::LevelTiles,
    sound::SoundIndex,
    Hero, Result, Sizes, OBJECT_ELEVATOR_TOP, SOLID_ELEVATOR,
};
use sdl2::rect::{Point, Rect};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Idle,
    Ascending,
    Descending,
}

#[derive(Debug)]
pub(crate) struct Elevator {
    state: State,
    position: Rect,
}

impl CreateActor for Elevator {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::Elevator(Self {
            state: State::Idle,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for Elevator {
    fn act(&mut self, p: ActParameters) {
        let hero_geometry = p.hero.position.geometry;

        if self.state == State::Ascending
            || self.state == State::Idle
                && self.position.height() as u32 > p.sizes.height()
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
                if p.tiles
                    .get(
                        self.position.x() / p.sizes.width() as i32,
                        self.position.y() / p.sizes.height() as i32 - 3,
                    )
                    .map(|t| t.solid)
                    .unwrap_or(true)
                {
                    // hero touches solid with head
                    self.state = State::Idle;
                } else {
                    p.game_commands.add_sound(SoundIndex::ELEVATOR);
                    let offset = p.hero.position.push_vertically(
                        p.sizes,
                        p.tiles,
                        -(p.sizes.height() as i32),
                    );
                    if -offset < p.sizes.height() as i32 {
                        p.hero
                            .position
                            .push_vertically(p.sizes, p.tiles, -offset);
                        self.state = State::Idle;
                    } else {
                        self.position.offset(0, offset);
                        self.position.set_height(
                            self.position.height() + (-offset) as u32,
                        );

                        if let Ok(ref mut t) = p.tiles.get_mut(
                            self.position.x() / p.sizes.width() as i32,
                            self.position.y() / p.sizes.height() as i32,
                        ) {
                            t.solid = true;
                        }
                    }
                }
            }
            State::Descending => {
                for _ in 0..2 {
                    if self.position.height() > p.sizes.height() {
                        if let Ok(ref mut t) = p.tiles.get_mut(
                            self.position.x() / p.sizes.width() as i32,
                            self.position.y() / p.sizes.height() as i32,
                        ) {
                            t.solid = false;
                        }
                        self.position.offset(0, p.sizes.height() as i32);
                        self.position.set_height(
                            self.position.height() - p.sizes.height(),
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
        for _ in 0..(self.position.height() / p.sizes.height() - 1) * 2 {
            pos.y += p.sizes.half_height() as i32;
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

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }
}
