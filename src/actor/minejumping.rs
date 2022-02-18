// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, CreateActor, RenderParameters,
        ShotParameters, ShotProcessing,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Hero, RangedIterator, Result, Sizes, ANIMATION_MINE,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct MineJumping {
    tile: usize,
    frame: RangedIterator,
    base_y: i32,
    position: Rect,
}

impl CreateActor for MineJumping {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::MineJumping(Self {
            tile: ANIMATION_MINE,
            frame: RangedIterator::new(12),
            base_y: pos.y,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for MineJumping {
    fn act(&mut self, p: ActParameters) {
        let relative_distance = match self.frame.current() {
            0 => 0f32,
            1 | 11 => 1f32,
            2 | 10 => 1.75f32,
            3 | 9 => 2.25f32,
            4 | 8 => 2.5f32,
            5 | 7 => 2.56f32,
            6 => 2.64f32,
            _ => unreachable!(),
        };
        let distance =
            ((p.sizes.height() as f32) * relative_distance) as i32;
        self.position.set_y(self.base_y - distance);

        if self.frame.is_first() {
            p.game_commands.add_sound(SoundIndex::MINEBOUNCE);
        }

        self.frame.next();
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        /*
         * We don't need to do anything, this is just to absorb
         * the bullet when the actor is shot.
         */
        true
    }

    fn shot(&mut self, _p: ShotParameters) -> ShotProcessing {
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

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }
}
