// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActorWithDetails,
        RenderParameters,
    },
    level::tiles::LevelTiles,
    Hero, HorizontalDirection, RangedIterator, Result, Sizes,
    ANIMATION_BOMBFIRE, ANIMATION_EXPLOSION, ANIMATION_ROBOT,
    OBJECT_DUSTCLOUD, OBJECT_ENEMY_GUNFIRE_LEFT,
    OBJECT_ENEMY_GUNFIRE_RIGHT, OBJECT_HERO_GUNFIRE_LEFT,
    OBJECT_HERO_GUNFIRE_RIGHT, OBJECT_STEAM,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct SingleAnimation {
    tile: usize,
    frame: RangedIterator,
    can_hurt_hero: bool,
    replaced_by: Option<ActorType>,
    position: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleAnimationType {
    BombFire,
    Explosion,
    DustCloud,
    Steam,
    RobotDisappearing,
    HeroGunFire(HorizontalDirection),
    EnemyGunFire(HorizontalDirection),
}

impl CreateActorWithDetails for SingleAnimation {
    type Details = SingleAnimationType;

    fn create_with_details(
        animation_type: SingleAnimationType,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        let (tile, num_frames, can_hurt_hero, replaced_by) =
            match animation_type {
                SingleAnimationType::BombFire => {
                    (ANIMATION_BOMBFIRE, 6, true, None)
                }
                SingleAnimationType::Explosion => {
                    (ANIMATION_EXPLOSION, 6, false, None)
                }
                SingleAnimationType::DustCloud => {
                    (OBJECT_DUSTCLOUD, 5, false, None)
                }
                SingleAnimationType::Steam => {
                    (OBJECT_STEAM, 5, false, None)
                }
                SingleAnimationType::RobotDisappearing => (
                    ANIMATION_ROBOT + 3,
                    7,
                    false,
                    Some(ActorType::SingleAnimation(
                        SingleAnimationType::Explosion,
                    )),
                ),
                SingleAnimationType::HeroGunFire(orientation) => {
                    let tile = match orientation {
                        HorizontalDirection::Left => {
                            OBJECT_HERO_GUNFIRE_LEFT
                        }
                        HorizontalDirection::Right => {
                            OBJECT_HERO_GUNFIRE_RIGHT
                        }
                    };
                    (tile, 1, false, None)
                }
                SingleAnimationType::EnemyGunFire(orientation) => {
                    let tile = match orientation {
                        HorizontalDirection::Left => {
                            OBJECT_ENEMY_GUNFIRE_LEFT
                        }
                        HorizontalDirection::Right => {
                            OBJECT_ENEMY_GUNFIRE_RIGHT
                        }
                    };
                    (tile, 1, false, None)
                }
            };

        Actor::SingleAnimation(Self {
            tile,
            frame: RangedIterator::new(num_frames),
            can_hurt_hero,
            replaced_by,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for SingleAnimation {
    fn act(&mut self, p: ActParameters) {
        self.frame.next();
        if !self.is_alive() {
            if let Some(successor) = self.replaced_by {
                p.game_commands
                    .add_actor(successor, self.position.top_left());
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.frame.current(),
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.can_hurt_hero
            && self.position.has_intersection(hero.position.geometry)
    }

    fn is_alive(&self) -> bool {
        self.frame.finished_cycles() == 0
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }
}
