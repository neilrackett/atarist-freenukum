use crate::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug)]
pub struct LevelRaw {
    raw: [[u16; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
}

impl Default for LevelRaw {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelRaw {
    pub fn new() -> Self {
        LevelRaw {
            raw: [[0u16; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
        }
    }

    pub fn set(&mut self, x: i32, y: i32, tile: u16) {
        if x >= 0
            && x < LEVEL_WIDTH as i32
            && y >= 0
            && y < LEVEL_HEIGHT as i32
        {
            self.raw[y as usize][x as usize] = tile;
        }
    }

    pub fn get(&self, x: i32, y: i32) -> u16 {
        if x >= 0
            && x < LEVEL_WIDTH as i32
            && y >= 0
            && y < LEVEL_HEIGHT as i32
        {
            self.raw[y as usize][x as usize]
        } else {
            0
        }
    }
}
