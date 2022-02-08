// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, SingleAnimationType,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Result, Sizes, ANIMATION_MINE,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct MineLying {
    tile: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for MineLying {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::MineLying(Self {
            tile: ANIMATION_MINE,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
        })
    }
}

impl ActorExt for MineLying {
    fn act(&mut self, p: ActParameters) {
        if p.tiles
            .get(
                self.position.x() / p.sizes.width() as i32,
                self.position.y() / p.sizes.height() as i32 + 1,
            )
            .map(|t| !t.solid)
            .unwrap_or(false)
        {
            self.position.offset(0, p.sizes.half_height() as i32);
        }

        if p.hero.position.geometry.has_intersection(self.position) {
            self.is_alive = false;
            p.game_commands.add_actor(
                ActorType::SingleAnimation(SingleAnimationType::BombFire),
                self.position.top_left(),
            );
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
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

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromAbove
    }
}
