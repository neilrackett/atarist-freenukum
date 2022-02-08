// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType, ActorType,
        CreateActor, HeroInteractStartParameters, RenderParameters,
    },
    hero::InventoryItem,
    level::tiles::LevelTiles,
    sound::SoundIndex,
    Hero, HorizontalDirection, Result, Sizes, OBJECT_GLOVE_SLOT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
enum State {
    Idle,
    Shooting,
    Expanded,
}

#[derive(Debug)]
pub(crate) struct GloveSlot {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    state: State,
    countdown: usize,
    position: Rect,
}

impl CreateActor for GloveSlot {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::GloveSlot(Self {
            tile: OBJECT_GLOVE_SLOT,
            current_frame: 0,
            num_frames: 4,
            state: State::Idle,
            countdown: 0,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for GloveSlot {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        match self.state {
            State::Idle => {
                if p.hero.inventory.is_set(InventoryItem::Glove) {
                    p.actor_message_queue
                        .push_back(ActorMessageType::ExpandFloor);
                    p.game_commands.add_sound(SoundIndex::BRIDGEXTEND);
                    self.state = State::Expanded;
                } else {
                    self.state = State::Shooting;
                    self.countdown = 20;
                }
            }
            State::Shooting => {}
            State::Expanded => {}
        }
    }

    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Idle => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
            }
            State::Shooting => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
                self.countdown -= 1;
                if self.countdown % 4 == 0 {
                    p.game_commands.add_actor(
                        ActorType::HostileShot(HorizontalDirection::Right),
                        self.position.top_left(),
                    );
                    p.game_commands.add_sound(SoundIndex::ENEMYSHOT);
                } else if self.countdown % 4 == 2 {
                    p.game_commands.add_actor(
                        ActorType::HostileShot(HorizontalDirection::Left),
                        self.position.top_left(),
                    );
                    p.game_commands.add_sound(SoundIndex::ENEMYSHOT);
                }
                if self.countdown == 0 {
                    self.state = State::Idle;
                }
            }
            State::Expanded => {}
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let adder = if self.current_frame == 0 { 0 } else { 1 };
        let mut pos = self.position.top_left();
        p.renderer.place_tile(self.tile + adder, pos)?;

        pos.x -= p.sizes.width() as i32;
        p.renderer.place_tile(self.tile + 2, pos)?;

        pos.x += 2 * p.sizes.width() as i32;
        p.renderer.place_tile(self.tile + 3, pos)?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
