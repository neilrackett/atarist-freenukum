// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Hero, HorizontalDirection, RangedIterator, Result, Sizes,
    ANIMATION_JUMPBOT, OBJECT_ENEMY_GUNFIRE_LEFT,
    OBJECT_ENEMY_GUNFIRE_RIGHT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Health {
    Healthy,
    Hurt1,
    Hurt2,
    Dead,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    StandingBeforeShot,
    LoadingShot,
    FiringShot,
    StandingBeforeJump,
    Jumping,
    Falling,
    Landing,
}

const JUMP_PROFILE: [(i32, i32); 13] = [
    (10, 36),
    (16, 16),
    (0, 8),
    (8, 4),
    (0, 2),
    (8, 1),
    (0, 1),
    (8, 0),
    (0, 0),
    (8, 0),
    (0, 0),
    (8, -1),
    (0, -1),
];

#[derive(Debug)]
pub(crate) struct JumpBot {
    orientation: HorizontalDirection,
    tile: usize,
    frame: RangedIterator,
    steam_frame: RangedIterator,
    state: State,
    health: Health,
    position: Rect,
}

impl JumpBot {
    fn turn_towards_hero(&mut self, hero_position: &Rect) {
        let hero_x = hero_position.center().x;
        let bot_x = self.position.center().x;

        use std::cmp::Ordering;
        self.orientation = match hero_x.cmp(&bot_x) {
            Ordering::Less => HorizontalDirection::Left,
            Ordering::Greater => HorizontalDirection::Right,
            Ordering::Equal => self.orientation,
        };
    }
}

const STANDING_DURATION: usize = 10;

impl CreateActor for JumpBot {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::JumpBot(Self {
            orientation: HorizontalDirection::Left,
            tile: ANIMATION_JUMPBOT,
            frame: RangedIterator::new(STANDING_DURATION),
            steam_frame: RangedIterator::new(8),
            state: State::Falling,
            health: Health::Healthy,
            position: Rect::new(
                pos.x,
                pos.y - (sizes.height() as i32),
                sizes.width() * 2,
                sizes.height() * 2,
            ),
        })
    }
}

impl ActorExt for JumpBot {
    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::StandingBeforeShot => {
                if self.frame.is_first() {
                    self.state = State::LoadingShot;
                    self.frame.reset(1);
                    self.turn_towards_hero(&p.hero.position.geometry);
                }
            }
            State::LoadingShot => {
                if self.frame.is_first() {
                    self.state = State::FiringShot;
                    self.frame.reset(1);
                }
            }
            State::FiringShot => {
                if self.frame.is_first() {
                    self.state = State::StandingBeforeJump;
                    self.frame.reset(STANDING_DURATION);
                    self.turn_towards_hero(&p.hero.position.geometry);

                    let shot_position = match self.orientation {
                        HorizontalDirection::Left => self
                            .position
                            .top_left()
                            .offset(-(p.sizes.width() as i32), 6),
                        HorizontalDirection::Right => {
                            self.position.top_right().offset(0, 6)
                        }
                    };
                    p.game_commands.add_actor(
                        ActorType::HostileShot(self.orientation),
                        shot_position,
                    );
                }
            }
            State::StandingBeforeJump => {
                if self.frame.is_first() {
                    self.state = State::Jumping;
                    self.frame.reset(JUMP_PROFILE.len());
                    self.turn_towards_hero(&p.hero.position.geometry);
                }
            }
            State::Jumping => {
                if self.frame.is_first() {
                    self.state = State::Falling;
                    self.frame.reset(1);
                    self.turn_towards_hero(&p.hero.position.geometry);
                }
            }
            State::Falling => {
                if p.tiles.rect_stands_on_ground_partially(
                    p.sizes,
                    self.position,
                ) {
                    self.state = State::Landing;
                    self.frame.reset(1);
                    self.turn_towards_hero(&p.hero.position.geometry);
                }
            }
            State::Landing => {
                if self.frame.is_first() {
                    self.state = State::StandingBeforeShot;
                    self.frame.reset(STANDING_DURATION);
                    self.turn_towards_hero(&p.hero.position.geometry);
                }
            }
        }

        match self.state {
            State::StandingBeforeShot => {}
            State::LoadingShot => {}
            State::FiringShot => {}
            State::StandingBeforeJump => {}
            State::Jumping => {
                let (x, y) = JUMP_PROFILE[self.frame.current()];
                p.tiles.push_rect_vertically(
                    p.sizes,
                    &mut self.position,
                    -y,
                );
                let horizontal_direction =
                    self.orientation.as_factor_i32();
                p.tiles.push_rect_horizontally(
                    p.sizes,
                    &mut self.position,
                    horizontal_direction * x,
                );
            }
            State::Falling => {
                p.tiles.rect_fall_down(
                    p.sizes,
                    &mut self.position,
                    p.sizes.height() as u8,
                );
            }
            State::Landing => {}
        }

        if self.steam_frame.is_first() {
            let mut steam_position1 = self.position.top_left().offset(
                p.sizes.width() as i32,
                -(p.sizes.height() as i32) / 2,
            );
            let mut steam_position2 = self
                .position
                .top_left()
                .offset(0, -(p.sizes.height() as i32) / 2);
            if self.orientation == HorizontalDirection::Right {
                std::mem::swap(&mut steam_position1, &mut steam_position2);
            }

            match self.health {
                Health::Hurt1 => {
                    p.game_commands.add_actor(
                        ActorType::SingleAnimation(
                            SingleAnimationType::Steam,
                        ),
                        steam_position1,
                    );
                }
                Health::Hurt2 => {
                    p.game_commands.add_actor(
                        ActorType::SingleAnimation(
                            SingleAnimationType::Steam,
                        ),
                        steam_position1,
                    );
                    p.game_commands.add_actor(
                        ActorType::SingleAnimation(
                            SingleAnimationType::Steam,
                        ),
                        steam_position2,
                    );
                }
                Health::Healthy | Health::Dead => {}
            }
        }

        self.frame.next();
        self.steam_frame.next();

        self.tile = match self.orientation {
            HorizontalDirection::Left => ANIMATION_JUMPBOT,
            HorizontalDirection::Right => ANIMATION_JUMPBOT + 12,
        };
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        match self.state {
            State::StandingBeforeShot | State::StandingBeforeJump => {
                let mut pos = self.position.top_left();
                let tile = self.tile;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 1;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 2;
                pos.x -= p.sizes.width() as i32;
                pos.y += p.sizes.height() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 3;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;
            }
            State::LoadingShot => {
                let mut pos = self.position.top_left();
                let tile = self.tile;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 1;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 4;
                pos.x -= p.sizes.width() as i32;
                pos.y += p.sizes.height() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 5;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;
            }
            State::FiringShot => {
                let mut pos = self.position.top_left();
                let tile = self.tile;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 1;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 2;
                pos.x -= p.sizes.width() as i32;
                pos.y += p.sizes.height() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 3;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;

                match self.orientation {
                    HorizontalDirection::Left => {
                        let tile = OBJECT_ENEMY_GUNFIRE_LEFT;
                        let pos = self
                            .position
                            .top_left()
                            .offset(-(p.sizes.width() as i32), 8);
                        p.renderer.place_tile(tile, pos)?;
                    }
                    HorizontalDirection::Right => {
                        let tile = OBJECT_ENEMY_GUNFIRE_RIGHT;
                        let pos = self.position.top_right().offset(0, 8);
                        p.renderer.place_tile(tile, pos)?;
                    }
                }
            }
            State::Jumping | State::Falling => {
                let mut pos = self
                    .position
                    .top_left()
                    .offset(0, p.sizes.height() as i32 / -2);
                let tile = self.tile + 6;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 7;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 8;
                pos.x -= p.sizes.width() as i32;
                pos.y += p.sizes.height() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 9;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 10;
                pos.x -= p.sizes.width() as i32;
                pos.y += p.sizes.height() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 11;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;
            }
            State::Landing => {
                let mut pos = self.position.top_left().offset(0, 1);
                let tile = self.tile;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 1;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 2;
                pos.x -= p.sizes.width() as i32;
                pos.y += p.sizes.height() as i32;
                p.renderer.place_tile(tile, pos)?;

                let tile = self.tile + 3;
                pos.x += p.sizes.width() as i32;
                p.renderer.place_tile(tile, pos)?;
            }
        }

        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.health = match self.health {
            Health::Healthy => Health::Hurt1,
            Health::Hurt1 => {
                p.game_commands
                    .add_particle_firework(self.position.top_left(), 4);
                p.hero.score.add(2500);
                p.game_commands.add_sound(SoundIndex::SMALLDEATH);

                Health::Hurt2
            }
            Health::Hurt2 => {
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

                Health::Dead
            }
            Health::Dead => Health::Dead,
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
        self.health != Health::Dead
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }

    fn acts_while_invisible(&self) -> bool {
        matches!(self.state, State::Jumping | State::Falling)
    }
}
