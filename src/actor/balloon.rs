// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, ScoreType, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::tiles::{LevelTiles, Tile},
    RangedIterator, Result, Sizes, SoundIndex, OBJECT_BALLOON,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Balloon {
    destroyed: bool,
    frame: RangedIterator,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for Balloon {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::Balloon(Self {
            destroyed: false,
            frame: RangedIterator::new(9),
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height() * 2,
            ),
            is_alive: true,
        })
    }
}

impl ActorExt for Balloon {
    fn act(&mut self, p: ActParameters) {
        self.frame.next();
        self.is_alive = !self.destroyed;

        if self.position.has_intersection(p.hero.position.geometry) {
            self.is_alive = false;
            p.hero.score.add(10000);
            p.game_commands.add_actor(
                ActorType::Score(ScoreType::Score10000),
                self.position.top_left(),
            );
            p.game_commands.add_sound(SoundIndex::GETBALLON);
        } else {
            self.position.y -= 1;
            if let Ok(Tile { solid: true, .. }) = p.tiles.get(
                self.position.x() / p.sizes.width() as i32,
                self.position.y() / p.sizes.height() as i32,
            ) {
                // balloon bumps against wall
                self.destroyed = true;
                p.game_commands.add_actor(
                    ActorType::SingleAnimation(SingleAnimationType::Steam),
                    self.position.top_left(),
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();

        let tile = if self.destroyed {
            OBJECT_BALLOON + 4
        } else {
            OBJECT_BALLOON
        };
        p.renderer.place_tile(tile, pos)?;

        pos.y += p.sizes.height() as i32;
        p.renderer.place_tile(
            OBJECT_BALLOON + 1 + self.frame.current() / 3,
            pos,
        )?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.destroyed = true;
        p.game_commands.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Steam),
            self.position.top_left(),
        );
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}
