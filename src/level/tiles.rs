use crate::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug)]
pub struct LevelTiles {
    tiles: [[u16; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
}

impl Default for LevelTiles {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelTiles {
    pub fn new() -> Self {
        LevelTiles {
            tiles: [[0u16; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: u16) {
        if x >= 0
            && x < LEVEL_WIDTH as i32
            && y >= 0
            && y < LEVEL_HEIGHT as i32
        {
            self.tiles[y as usize][x as usize] = value;
        }
    }

    pub fn get(&self, x: i32, y: i32) -> u16 {
        if x >= 0
            && x < LEVEL_WIDTH as i32
            && y >= 0
            && y < LEVEL_HEIGHT as i32
        {
            self.tiles[y as usize][x as usize]
        } else {
            0
        }
    }

    pub fn copy_from_to(
        &mut self,
        x_from: i32,
        y_from: i32,
        x_to: i32,
        y_to: i32,
    ) {
        self.set(x_to, y_to, self.get(x_from, y_from));
    }
}
