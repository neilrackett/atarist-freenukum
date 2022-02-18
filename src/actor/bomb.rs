// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, SingleAnimationType,
    },
    level::tiles::LevelTiles,
    sound::SoundIndex,
    RangedIterator, Result, Sizes, ANIMATION_BOMB,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq)]
pub(crate) struct Bomb {
    tile: usize,
    frame: RangedIterator,
    counter: u32,
    explode_left: bool,
    explode_right: bool,
    explode_threshold: u32,
    num_flames: u32,
    position: Rect,
}

impl CreateActor for Bomb {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::Bomb(Self {
            tile: ANIMATION_BOMB,
            frame: RangedIterator::new(2),
            counter: 0,
            explode_left: true,
            explode_right: true,
            explode_threshold: 12,
            num_flames: 4,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for Bomb {
    fn act(&mut self, p: ActParameters) {
        self.frame.next();

        self.counter += 1;

        if self.counter < self.explode_threshold {
        } else if self.counter < self.explode_threshold + self.num_flames {
            let distance =
                self.counter as i32 - self.explode_threshold as i32;
            if self.explode_left {
                // explode to the left if possible
                let space_is_free = p
                    .tiles
                    .get(
                        self.position.x() / p.sizes.width() as i32
                            - distance,
                        self.position.y() / p.sizes.height() as i32,
                    )
                    .map(|t| !t.solid)
                    .unwrap_or(false);
                let space_has_solid_below = p
                    .tiles
                    .get(
                        self.position.x() / p.sizes.width() as i32
                            - distance,
                        self.position.y() / p.sizes.height() as i32 + 1,
                    )
                    .map(|t| t.solid)
                    .unwrap_or(false);
                if space_is_free && space_has_solid_below {
                    p.game_commands.add_sound(SoundIndex::BOMBEXPLODE);
                    p.game_commands.add_actor(
                        ActorType::SingleAnimation(
                            SingleAnimationType::BombFire,
                        ),
                        self.position.top_left().offset(
                            -(distance * p.sizes.width() as i32),
                            0,
                        ),
                    );
                } else {
                    self.explode_left = false;
                }
            }
            if self.explode_right {
                // explode to the right if possible
                let space_is_free = p
                    .tiles
                    .get(
                        self.position.x() / p.sizes.width() as i32
                            + distance,
                        self.position.y() / p.sizes.height() as i32,
                    )
                    .map(|t| !t.solid)
                    .unwrap_or(false);
                let space_has_solid_below = p
                    .tiles
                    .get(
                        self.position.x() / p.sizes.width() as i32
                            + distance,
                        self.position.y() / p.sizes.height() as i32 + 1,
                    )
                    .map(|t| t.solid)
                    .unwrap_or(false);
                if space_is_free && space_has_solid_below {
                    p.game_commands.add_actor(
                        ActorType::SingleAnimation(
                            SingleAnimationType::BombFire,
                        ),
                        self.position
                            .top_left()
                            .offset(distance * p.sizes.width() as i32, 0),
                    );
                } else {
                    self.explode_right = false;
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        if self.counter < self.explode_threshold {
            p.renderer.place_tile(
                self.tile + self.frame.current(),
                self.position.top_left(),
            )?;
        }
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn is_alive(&self) -> bool {
        self.counter < self.explode_threshold + self.num_flames
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }
}
