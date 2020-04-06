#[macro_use]
extern crate serde_derive;

mod borders;
mod game;
mod settings;

use game::Game;

const HALFTILE_WIDTH: u8 = 8;
const HALFTILE_HEIGHT: u8 = 8;

fn main() {
    println!("Starting FreeNukum…");

    let settings = settings::Settings { scale: 2f32 };

    let game = Game::new(settings);

    if let Err(e) = game.run() {
        eprintln!("{}", e);
    }
}
