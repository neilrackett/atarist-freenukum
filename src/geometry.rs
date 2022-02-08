// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use sdl2::rect::Rect;

pub trait RectExt {
    fn touches(&self, other: Self) -> bool;
    fn overlaps_vertically(&self, other: Self) -> bool;
    fn horizontal_distance(&self, other: Self) -> i32;
}

impl RectExt for Rect {
    fn touches(&self, other: Self) -> bool {
        let mut r1 = *self;
        r1.w += 1;
        r1.h += 1;
        let mut r2 = other;
        r2.w += 1;
        r2.h += 1;
        r1.has_intersection(r2)
    }

    fn overlaps_vertically(&self, other: Self) -> bool {
        if self.y as i32 + self.h as i32 <= other.y as i32 {
            return false;
        }
        if other.y as i32 + other.h as i32 <= self.y as i32 {
            return false;
        }
        true
    }

    fn horizontal_distance(&self, other: Self) -> i32 {
        if self.right() < other.left() {
            self.right() - other.left()
        } else if other.right() < self.left() {
            self.left() - other.right()
        } else {
            0
        }
    }
}
