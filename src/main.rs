#[macro_use]
extern crate serde_derive;

mod borders;
mod game;
mod settings;
mod text;
mod tile;

use game::Game;

const HALFTILE_WIDTH: usize = 8;
const HALFTILE_HEIGHT: usize = 8;
const TILE_WIDTH: usize = HALFTILE_WIDTH * 2;
const TILE_HEIGHT: usize = HALFTILE_HEIGHT * 2;
const MAX_TILES_PER_FILE: usize = 50;
const HEALTH_COUNT: usize = 8;
const INVENTORY_WIDTH: usize = HEALTH_COUNT / 2;
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 8;

fn main() {
    println!("Starting FreeNukum…");

    let settings = settings::Settings { scale: 2f32 };

    let game = Game::new(settings);

    if let Err(e) = game.run() {
        eprintln!("{}", e);
    }
}
