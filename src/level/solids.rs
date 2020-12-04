use crate::{LEVEL_HEIGHT, LEVEL_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use sdl2::rect::Rect;

#[derive(Debug)]
pub struct LevelSolids {
    solids: [[bool; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
}

impl Default for LevelSolids {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelSolids {
    pub fn new() -> Self {
        LevelSolids {
            solids: [[false; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
        }
    }

    pub fn new_all_solid() -> Self {
        LevelSolids {
            solids: [[true; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
        }
    }

    pub fn dump(&self) {
        print!("    ");
        for i in 0..LEVEL_WIDTH {
            print!("{}", i % 10);
        }
        println!();

        print!("   ┏");
        for _ in 0..LEVEL_WIDTH {
            print!("━");
        }
        println!("┓");

        for (i, row) in self.solids.iter().enumerate() {
            print!("{:>3}┃", i);
            for col in row {
                if *col {
                    print!("█");
                } else {
                    print!("░");
                }
            }
            println!("┃");
        }

        print!("   ┗");
        for _ in 0..LEVEL_WIDTH {
            print!("━");
        }
        println!("┛");

        print!("    ");
        for i in 0..LEVEL_WIDTH {
            print!("{}", i % 10);
        }
        println!();

        print!("    ");
        for i in 0..LEVEL_WIDTH {
            if i % 10 == 0 {
                print!("^{:<9}", i / 10);
            }
        }
        println!();
    }

    pub fn set(&mut self, x: u32, y: u32, value: bool) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.solids[y as usize][x as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.solids[y as usize][x as usize]
    }

    pub fn collides(&self, rect: Rect) -> bool {
        let mut solidrect = Rect::new(0, 0, TILE_WIDTH, TILE_HEIGHT);
        let left_edge = rect.left() as u32 / TILE_WIDTH;
        let right_edge = rect.right() as u32 / TILE_WIDTH + 1;
        let top_edge = rect.top() as u32 / TILE_HEIGHT;
        let bottom_edge = rect.bottom() as u32 / TILE_HEIGHT + 1;

        for i in left_edge..right_edge {
            for j in top_edge..bottom_edge {
                if self.get(i, j) {
                    solidrect.x = (i * TILE_WIDTH) as i32;
                    solidrect.y = (j * TILE_HEIGHT) as i32;
                    if rect.has_intersection(solidrect) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn push_rect_standing_on_ground(
        &self,
        rect: &mut Rect,
        offset: i32,
        gravity: u8,
    ) -> bool {
        if self.collides(*rect) {
            // locked in, can't move at all.
            return false;
        }
        // fall down as far as possible
        self.rect_fall_down(rect, gravity);

        // check if we stand on solid ground before movement
        let stood_solid = self.rect_stands_on_ground_completely(*rect);

        rect.offset(offset, 0);

        if self.collides(*rect) {
            rect.offset(-offset, 0);
            false
        } else if stood_solid
            && self.rect_stands_on_ground_completely(*rect)
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

    fn rect_fall_down(&self, rect: &mut Rect, distance: u8) -> u8 {
        if self.collides(*rect) {
            // can't fall down because collides with solid ground.
            return 0;
        }
        if self.rect_stands_on_ground_partially(*rect) {
            // partially stands on solid ground, can't fall down.
            return 0;
        }
        for i in 0..distance {
            // check how far we can fall down.
            rect.y += 1;
            if self.rect_stands_on_ground_partially(*rect) {
                return i;
            }
        }
        distance
    }

    fn rect_stands_on_ground_partially(&self, rect: Rect) -> bool {
        if (rect.bottom() as u32 % TILE_HEIGHT) > 0 {
            return false;
        }
        let j = rect.bottom() as u32 / TILE_HEIGHT;
        for i in (rect.left() as u32 / TILE_WIDTH)
            ..(rect.right() as u32 + 1) / TILE_WIDTH + 1
        {
            if self.get(i, j) {
                return true;
            }
        }
        false
    }

    fn rect_stands_on_ground_completely(&self, rect: Rect) -> bool {
        if (rect.bottom() % TILE_HEIGHT as i32) > 0 {
            return false;
        }
        let j = rect.bottom() as u32 / TILE_HEIGHT;
        for i in (rect.left() as u32 / TILE_WIDTH)
            ..(rect.right() as u32 + 1) / TILE_WIDTH + 1
        {
            if !self.get(i, j) {
                return false;
            }
        }
        true
    }
}
