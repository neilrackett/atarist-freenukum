// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType,
        CreateActorWithDetails, HeroInteractStartParameters,
        ReceiveMessageParameters, RenderParameters,
    },
    level::tiles::LevelTiles,
    sound::SoundIndex,
    Hero, RangedIterator, Result, Sizes, ANIMATION_TELEPORTER1,
};
use sdl2::rect::{Point, Rect};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Idle,
    Sending,
    Receiving,
}

#[derive(Debug)]
pub(crate) struct Teleporter {
    position: Rect,
    index: TeleporterIndex,
    frame: RangedIterator,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeleporterIndex {
    First,
    Second,
}

impl TeleporterIndex {
    fn other(self) -> Self {
        match self {
            TeleporterIndex::First => TeleporterIndex::Second,
            TeleporterIndex::Second => TeleporterIndex::First,
        }
    }
}

impl CreateActorWithDetails for Teleporter {
    type Details = TeleporterIndex;

    fn create_with_details(
        index: TeleporterIndex,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::Teleporter(Self {
            position: Rect::new(
                pos.x - sizes.width() as i32,
                pos.y - 2 * sizes.height() as i32,
                sizes.width() * 3,
                sizes.height() * 3,
            ),
            index,
            frame: RangedIterator::new(12),
            state: State::Idle,
        })
    }
}

impl ActorExt for Teleporter {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        if self.state == State::Idle {
            self.state = State::Sending;
        }
        p.game_commands.add_sound(SoundIndex::TELEPORT);
    }

    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Idle => {}
            State::Sending => {
                self.frame.next();
                if self.frame.is_first() {
                    p.actor_message_queue.push_back(
                        ActorMessageType::TeleportTo(self.index.other()),
                    );
                    self.state = State::Idle;
                }
            }
            State::Receiving => {
                self.frame.next();
                if self.frame.is_first() {
                    self.state = State::Idle;
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        for i in 0..3 {
            for j in 0..3 {
                let pos = self.position.top_left().offset(
                    j * p.sizes.width() as i32,
                    i * p.sizes.height() as i32,
                );
                let extra_offset = if i == 1 && j == 1 {
                    // Center part with buttons
                    match self.frame.current() % 3 {
                        0 => 0,
                        1 => 8,
                        2 => 9,
                        _ => unreachable!(),
                    }
                } else if i == 1 && j == 2 {
                    // Center right part with lights
                    match self.frame.current() % 3 {
                        0 => 0,
                        1 => 9,
                        2 => 10,
                        _ => unreachable!(),
                    }
                } else if i == 0
                    && j == 1
                    && (self.state == State::Sending
                        || self.state == State::Receiving)
                {
                    // Center top part with electric beam while teleporting
                    match self.frame.current() % 4 {
                        0 | 2 => 0,
                        1 => 8,
                        3 => 9,
                        _ => unreachable!(),
                    }
                } else {
                    0
                };

                // Base tiles
                let tile_offset = i as usize * 3 + j as usize;
                let tile = ANIMATION_TELEPORTER1 + tile_offset;
                p.renderer.place_tile(tile, pos)?;

                // Extra tiles as overlay
                if extra_offset != 0 {
                    let tile_offset =
                        i as usize * 3 + j as usize + extra_offset;
                    let tile = ANIMATION_TELEPORTER1 + tile_offset;
                    p.renderer.place_tile(tile, pos)?;
                }
            }
        }
        Ok(())
    }

    fn can_receive_message(&self, message: ActorMessageType) -> bool {
        message == ActorMessageType::TeleportTo(self.index)
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        self.state = State::Receiving;
        p.hero.position.move_to(
            p.sizes,
            self.position.x() + p.sizes.width() as i32,
            self.position.y() + p.sizes.height() as i32,
        );
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }
}
