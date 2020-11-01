use crate::{LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug)]
pub struct LevelTiles {
    tiles: [[u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
}

impl LevelTiles {
    pub fn new() -> Self {
        LevelTiles {
            tiles: [[0u16; LEVEL_WIDTH]; LEVEL_HEIGHT],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u16) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.tiles[y][x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> u16 {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.tiles[y][x]
    }

    pub fn copy_from_to(
        &mut self,
        x_from: usize,
        y_from: usize,
        x_to: usize,
        y_to: usize,
    ) {
        self.set(x_to, y_to, self.get(x_from, y_from));
    }
}
