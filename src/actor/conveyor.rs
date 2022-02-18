// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, CreateActorWithDetails,
        RenderParameters,
    },
    level::tiles::LevelTiles,
    HorizontalDirection, RangedIterator, Result, Sizes, SOLID_BLACK,
    SOLID_CONVEYORBELT_CENTER, SOLID_CONVEYORBELT_LEFTEND,
    SOLID_CONVEYORBELT_RIGHTEND,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Conveyor {
    frame: RangedIterator,
    direction: HorizontalDirection,
    position: Rect,
}

impl CreateActorWithDetails for Conveyor {
    type Details = HorizontalDirection;

    fn create_with_details(
        direction: HorizontalDirection,
        pos: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Actor {
        // find the beginning of the conveyor belt
        let mut found_begin = false;
        let mut position =
            Rect::new(pos.x, pos.y, sizes.width(), sizes.height());
        while !found_begin {
            position.offset(-(sizes.width() as i32), 0);
            position.set_width(position.width() + sizes.width());

            if let Ok(ref mut t) = tiles.get_mut(
                position.x() / sizes.width() as i32,
                position.y() / sizes.height() as i32,
            ) {
                if t.effective_number as usize
                    == SOLID_CONVEYORBELT_LEFTEND
                    || position.x() <= 0
                    || t.effective_number == 0
                {
                    found_begin = true;
                    t.effective_number = SOLID_BLACK as u16;
                }
            } else {
                found_begin = true;
            }
        }

        Actor::Conveyor(Self {
            frame: RangedIterator::new(4),
            direction,
            position,
        })
    }
}

impl ActorExt for Conveyor {
    fn act(&mut self, p: ActParameters) {
        self.frame.next();
        let hero_push_offset = match self.direction {
            HorizontalDirection::Left => -(p.sizes.half_width() as i32),
            HorizontalDirection::Right => p.sizes.half_width() as i32,
        };

        let hero_geometry = p.hero.position.geometry;

        if hero_geometry.right() > self.position.left()
            && hero_geometry.left() < self.position.right()
            && hero_geometry.bottom() == self.position.top()
        {
            p.hero.position.push_horizontally(
                p.sizes,
                p.tiles,
                hero_push_offset,
            );
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile_number_offset = match self.direction {
            HorizontalDirection::Left => self.frame.current_reverse(),
            HorizontalDirection::Right => self.frame.current(),
        };
        let mut tile = SOLID_CONVEYORBELT_LEFTEND + tile_number_offset;
        let mut pos = self.position.top_left();

        let num_elements = self.position.width() / p.sizes.width();
        for i in 0..num_elements {
            if i == num_elements - 1 {
                // right end of the conveyor
                tile = SOLID_CONVEYORBELT_RIGHTEND + tile_number_offset;
            } else if i == 1 {
                // center parts of the conveyor
                tile = SOLID_CONVEYORBELT_CENTER + tile_number_offset % 2;
            }
            p.renderer.place_tile(tile, pos)?;
            pos.x += p.sizes.width() as i32;
        }
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
