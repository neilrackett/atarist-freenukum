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

    pub fn set(&mut self, x: u32, y: u32, value: u16) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.tiles[y as usize][x as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> u16 {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.tiles[y as usize][x as usize]
    }

    pub fn copy_from_to(
        &mut self,
        x_from: u32,
        y_from: u32,
        x_to: u32,
        y_to: u32,
    ) {
        self.set(x_to, y_to, self.get(x_from, y_from));
    }
}
