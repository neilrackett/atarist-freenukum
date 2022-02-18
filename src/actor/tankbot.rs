// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::tiles::LevelTiles,
    sound::SoundIndex,
    Hero, HorizontalDirection, RangedIterator, Result, Sizes,
    ANIMATION_CARBOT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    Healthy,
    Hurt,
    Dead,
}

#[derive(Debug)]
pub(crate) struct TankBot {
    orientation: HorizontalDirection,
    tile: usize,
    frame: RangedIterator,
    state: State,
    position: Rect,
}

impl CreateActor for TankBot {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::TankBot(Self {
            orientation: HorizontalDirection::Left,
            tile: ANIMATION_CARBOT,
            frame: RangedIterator::new(4),
            state: State::Healthy,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width() * 2,
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for TankBot {
    fn act(&mut self, p: ActParameters) {
        self.frame.next();

        let solid_below = p
            .tiles
            .get(
                self.position.x() / p.sizes.width() as i32,
                self.position.y() / p.sizes.height() as i32 + 1,
            )
            .map(|t| t.solid)
            .unwrap_or(true);

        if !solid_below {
            // still in the air, falling down
            self.position.offset(0, p.sizes.half_height() as i32);
        } else {
            // on the floor, walking
            let mut direction = match self.orientation {
                HorizontalDirection::Left => -1,
                HorizontalDirection::Right => 4,
            };

            let next_place_is_solid = p
                .tiles
                .get(
                    (self.position.x()
                        + direction * p.sizes.half_width() as i32)
                        / p.sizes.width() as i32,
                    self.position.y() / p.sizes.height() as i32,
                )
                .map(|t| t.solid)
                .unwrap_or(true);
            let next_place_is_solid_below = p
                .tiles
                .get(
                    // check if the tile below is solid
                    (self.position.x()
                        + direction * p.sizes.half_width() as i32)
                        / p.sizes.width() as i32,
                    (self.position.y() + p.sizes.height() as i32)
                        / p.sizes.height() as i32,
                )
                .map(|t| t.solid)
                .unwrap_or(true);

            if !next_place_is_solid && next_place_is_solid_below {
                // place is free and it's possible to move on
                if direction > 0 {
                    direction = 1;
                }
                self.position.offset(
                    (direction as f64 * p.sizes.half_width() as f64 * 0.7)
                        as i32,
                    0,
                );
            } else {
                // reached the end, turning around
                self.orientation.reverse();
                if direction > 0 {
                    direction = 1;
                }
                direction *= -1;
                self.position
                    .offset(direction * p.sizes.half_width() as i32, 0);
                self.tile = (self.tile as i32 + 4 * direction) as usize;

                p.game_commands.add_actor(
                    ActorType::HostileShot(self.orientation),
                    self.position.top_left().offset(0, -6),
                );
                p.game_commands.add_sound(SoundIndex::ENEMYSHOT);
            }
        }
        if self.state == State::Hurt {
            // create steam clouds
            if self.frame.is_first() {
                p.game_commands.add_actor(
                    ActorType::SingleAnimation(SingleAnimationType::Steam),
                    self.position.top_left().offset(
                        p.sizes.half_width() as i32,
                        -(p.sizes.height() as i32),
                    ),
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();
        let tile = self.tile + (self.frame.current() / 2) * 2;
        p.renderer.place_tile(tile, pos)?;

        let tile = self.tile + (self.frame.current() / 2) * 2 + 1;
        pos = pos.offset(p.sizes.width() as i32, 0);
        p.renderer.place_tile(tile, pos)?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.state = match self.state {
            State::Healthy => State::Hurt,
            State::Hurt => {
                p.game_commands.add_actor(
                    ActorType::SingleAnimation(
                        SingleAnimationType::Explosion,
                    ),
                    self.position
                        .top_left()
                        .offset(p.sizes.half_width() as i32, 0),
                );
                p.game_commands
                    .add_particle_firework(self.position.top_left(), 4);
                p.hero.score.add(2500);
                p.game_commands.add_sound(SoundIndex::SMALLDEATH);

                State::Dead
            }
            State::Dead => State::Dead,
        };

        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.position.has_intersection(hero.position.geometry)
    }

    fn is_alive(&self) -> bool {
        self.state != State::Dead
    }
}
