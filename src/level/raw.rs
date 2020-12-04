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

    pub fn set(&mut self, x: u32, y: u32, tile: u16) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.raw[y as usize][x as usize] = tile;
    }

    pub fn get(&self, x: u32, y: u32) -> u16 {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.raw[y as usize][x as usize]
    }
}
