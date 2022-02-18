// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType,
        CreateActorWithDetails, HeroInteractStartParameters,
        RenderParameters,
    },
    hero::InventoryItem,
    level::tiles::LevelTiles,
    sound::SoundIndex,
    Hero, KeyColor, RangedIterator, Result, Sizes, OBJECT_KEYHOLE_BLACK,
    OBJECT_KEYHOLE_BLUE, OBJECT_KEYHOLE_GREEN, OBJECT_KEYHOLE_PINK,
    OBJECT_KEYHOLE_RED,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
enum State {
    Initial,
    Opened,
}

#[derive(Debug)]
pub(crate) struct KeyHole {
    tile: usize,
    frame: RangedIterator,
    state: State,
    position: Rect,
    color: KeyColor,
}

impl CreateActorWithDetails for KeyHole {
    type Details = KeyColor;

    fn create_with_details(
        color: KeyColor,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::KeyHole(Self {
            tile: OBJECT_KEYHOLE_BLACK,
            frame: RangedIterator::new(8),
            state: State::Initial,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            color,
        })
    }
}

impl ActorExt for KeyHole {
    fn act(&mut self, _p: ActParameters) {
        self.frame.next();
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let color_tile = match self.color {
            KeyColor::Red => OBJECT_KEYHOLE_RED,
            KeyColor::Blue => OBJECT_KEYHOLE_BLUE,
            KeyColor::Pink => OBJECT_KEYHOLE_PINK,
            KeyColor::Green => OBJECT_KEYHOLE_GREEN,
        };
        let tile = if self.frame.current() < self.frame.max_value() / 2 {
            self.tile
        } else {
            color_tile
        };

        p.renderer.place_tile(tile, self.position.top_left())?;
        Ok(())
    }

    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        let required_item = InventoryItem::Key(self.color);

        match self.state {
            State::Initial => {
                if p.hero.inventory.is_set(required_item) {
                    p.actor_message_queue
                        .push_back(ActorMessageType::OpenDoor(self.color));
                    self.frame.reset(1);
                    self.state = State::Opened;
                    p.hero.inventory.unset(required_item);
                    p.game_commands.add_sound(SoundIndex::OPENKEYDOOR);
                } else {
                    p.info_message_queue.push_back(format!(
                        "You don't have the {} key.",
                        self.color.to_string()
                    ));
                }
            }
            State::Opened => {}
        }
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
