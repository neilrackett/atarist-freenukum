// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{Sizes, LEVEL_HEIGHT, LEVEL_WIDTH};
use anyhow::{bail, Result};
use sdl2::rect::Rect;

#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub raw_number: u16,
    pub effective_number: u16,
    pub solid: bool,
}

#[derive(Debug)]
pub struct LevelTiles {
    tiles: [[Tile; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
}

impl Default for LevelTiles {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelTiles {
    pub fn new() -> Self {
        LevelTiles {
            tiles: [[Tile::default(); LEVEL_WIDTH as usize];
                LEVEL_HEIGHT as usize],
        }
    }

    pub fn new_all_solid() -> Self {
        LevelTiles {
            tiles: [[Tile {
                raw_number: 0,
                effective_number: 0,
                solid: true,
            }; LEVEL_WIDTH as usize];
                LEVEL_HEIGHT as usize],
        }
    }

    pub fn width(&self) -> u32 {
        LEVEL_WIDTH
    }

    pub fn height(&self) -> u32 {
        LEVEL_HEIGHT
    }

    pub fn is_in_range(&self, x: i32, y: i32) -> bool {
        x >= 0
            && x < self.width() as i32
            && y >= 0
            && y < self.height() as i32
    }

    pub fn get(&self, x: i32, y: i32) -> Result<Tile> {
        if !self.is_in_range(x, y) {
            bail!("Invalid level index {}/{}", x, y)
        }
        Ok(self.tiles[y as usize][x as usize])
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Result<&mut Tile> {
        if !self.is_in_range(x, y) {
            bail!("Invalid level index {}/{}", x, y)
        }
        Ok(&mut self.tiles[y as usize][x as usize])
    }

    pub fn copy_effective_number_from_to(
        &mut self,
        x_from: i32,
        y_from: i32,
        x_to: i32,
        y_to: i32,
    ) -> Result<()> {
        self.get_mut(x_to, y_to)?.effective_number =
            self.get(x_from, y_from)?.effective_number;
        Ok(())
    }

    pub fn dump(&self) {
        print!("    ");
        for i in 0..self.width() {
            print!("{}", i % 10);
        }
        println!();

        print!("   ┏");
        for _ in 0..self.width() {
            print!("━");
        }
        println!("┓");

        for (i, row) in self.tiles.iter().enumerate() {
            print!("{:>3}┃", i);
            for col in row {
                if col.solid {
                    print!("█");
                } else {
                    print!("░");
                }
            }
            println!("┃");
        }

        print!("   ┗");
        for _ in 0..self.width() {
            print!("━");
        }
        println!("┛");

        print!("    ");
        for i in 0..self.width() {
            print!("{}", i % 10);
        }
        println!();

        print!("    ");
        for i in 0..self.width() {
            if i % 10 == 0 {
                print!("^{:<9}", i / 10);
            }
        }
        println!();
    }

    pub fn collides(&self, sizes: &dyn Sizes, rect: Rect) -> bool {
        if rect.left() < 0
            || rect.top() < 0
            || rect.right() >= LEVEL_WIDTH as i32 * sizes.width() as i32
            || rect.bottom() >= LEVEL_HEIGHT as i32 * sizes.height() as i32
        {
            return true;
        }

        let mut solidrect = Rect::new(0, 0, sizes.width(), sizes.height());
        let left_edge = rect.left() / sizes.width() as i32;
        let right_edge = rect.right() / sizes.width() as i32 + 1;
        let top_edge = rect.top() / sizes.height() as i32;
        let bottom_edge = rect.bottom() / sizes.height() as i32 + 1;

        for i in left_edge..right_edge {
            for j in top_edge..bottom_edge {
                if self.get(i, j).map(|t| t.solid).unwrap_or(true) {
                    solidrect.x = i * sizes.width() as i32;
                    solidrect.y = j * sizes.height() as i32;
                    if rect.has_intersection(solidrect) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn push_rect_vertically(
        &self,
        sizes: &dyn Sizes,
        geometry: &mut Rect,
        offset: i32,
    ) -> i32 {
        if offset == 0 {
            return 0;
        }
        geometry.y += offset;

        if !self.collides(sizes, *geometry) {
            return offset;
        }

        let offset_absolute = offset.abs();
        let direction = offset / offset_absolute;

        for i in 0..offset_absolute {
            geometry.offset(0, -direction);
            if !self.collides(sizes, *geometry) {
                return i * direction;
            }
        }
        0
    }

    pub fn push_rect_horizontally(
        &self,
        sizes: &dyn Sizes,
        geometry: &mut Rect,
        offset: i32,
    ) -> i32 {
        if offset == 0 {
            return 0;
        }
        geometry.x += offset;

        if !self.collides(sizes, *geometry) {
            return offset;
        }

        let offset_absolute = offset.abs();
        let direction = offset / offset_absolute;

        for i in 0..offset_absolute {
            geometry.offset(-direction, 0);
            if !self.collides(sizes, *geometry) {
                return i * direction;
            }
        }
        0
    }

    pub fn push_rect_standing_on_ground(
        &self,
        sizes: &dyn Sizes,
        rect: &mut Rect,
        offset: i32,
        gravity: u8,
    ) -> bool {
        if self.collides(sizes, *rect) {
            // locked in, can't move at all.
            return false;
        }
        // fall down as far as possible
        self.rect_fall_down(sizes, rect, gravity);

        // check if we stand on solid ground before movement
        let stood_solid =
            self.rect_stands_on_ground_completely(sizes, *rect);

        rect.offset(offset, 0);

        if self.collides(sizes, *rect) {
            rect.offset(-offset, 0);
            false
        } else if stood_solid
            && self.rect_stands_on_ground_completely(sizes, *rect)
        {
            // stood on solid ground before, still does.
            true
        } else if stood_solid {
            // stood on solid ground before, doesn't anymore.
            rect.offset(-offset, 0);
            false
        } else {
            // walk on partial solid ground as long as possible.
            true
        }
    }

    pub fn rect_fall_down(
        &self,
        sizes: &dyn Sizes,
        rect: &mut Rect,
        distance: u8,
    ) -> u8 {
        if self.collides(sizes, *rect) {
            // can't fall down because collides with solid ground.
            return 0;
        }
        if self.rect_stands_on_ground_partially(sizes, *rect) {
            // partially stands on solid ground, can't fall down.
            return 0;
        }
        for i in 0..distance {
            // check how far we can fall down.
            rect.y += 1;
            if self.rect_stands_on_ground_partially(sizes, *rect) {
                return i;
            }
        }
        distance
    }

    pub fn rect_stands_on_ground_partially(
        &self,
        sizes: &dyn Sizes,
        rect: Rect,
    ) -> bool {
        if (rect.bottom() as u32 % sizes.height()) > 0 {
            return false;
        }
        let j = rect.bottom() / sizes.height() as i32;
        for i in (rect.left() / sizes.width() as i32)
            ..1 + rect.right() / sizes.width() as i32
        {
            if self.get(i, j).map(|t| t.solid).unwrap_or(true) {
                let tile = Rect::new(
                    i * sizes.width() as i32,
                    j * sizes.height() as i32,
                    sizes.width(),
                    sizes.height(),
                );
                if (tile.left() < rect.right()
                    && tile.right() >= rect.right())
                    || (tile.left() <= rect.left()
                        && tile.right() > rect.left())
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn rect_stands_on_ground_completely(
        &self,
        sizes: &dyn Sizes,
        rect: Rect,
    ) -> bool {
        if (rect.bottom() % sizes.height() as i32) > 0 {
            return false;
        }
        let j = rect.bottom() / sizes.height() as i32;
        for i in (rect.left() / sizes.width() as i32)
            ..(rect.right() + 1) / sizes.width() as i32 + 1
        {
            if !self.get(i, j).map(|t| t.solid).unwrap_or(true) {
                return false;
            }
        }
        true
    }
}
