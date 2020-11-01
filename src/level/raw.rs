use crate::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug)]
pub struct LevelRaw {
    raw: [[u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
}

impl LevelRaw {
    pub fn new() -> Self {
        LevelRaw {
            raw: [[0u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: u16) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.raw[y][x] = tile;
    }

    pub fn get(&self, x: usize, y: usize) -> u16 {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.raw[y][x]
    }
}
