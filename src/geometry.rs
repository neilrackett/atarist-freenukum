pub trait RectExt {
    fn touches(&self, other: Self) -> bool;
    fn overlaps(&self, other: Self) -> bool;
    fn overlaps_vertically(&self, other: Self) -> bool;
    fn horizontal_distance(&self, other: Self) -> i32;
}

impl RectExt for transdl::video::Rect {
    fn touches(&self, other: Self) -> bool {
        let mut r1 = self.clone();
        r1.w += 1;
        r1.h += 1;
        let mut r2 = other.clone();
        r2.w += 1;
        r2.h += 1;
        r1.overlaps(r2)
    }

    fn overlaps(&self, other: Self) -> bool {
        if self.x as i32 + self.w as i32 <= other.x as i32 {
            return false;
        }
        if other.x as i32 + other.w as i32 <= self.x as i32 {
            return false;
        }
        if self.y as i32 + self.h as i32 <= other.y as i32 {
            return false;
        }
        if other.y as i32 + other.h as i32 <= self.y as i32 {
            return false;
        }
        true
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
        if (self.x as i32 + self.w as i32) < other.x as i32 {
            return other.x as i32 - self.w as i32 - self.x as i32;
        }
        if (other.x as i32 + other.w as i32) < self.x as i32 {
            return -(self.x as i32 - other.w as i32 - other.x as i32);
        }
        return 0;
    }
}
