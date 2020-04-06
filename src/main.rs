#[macro_use]
extern crate serde_derive;

mod borders;
mod game;
mod settings;
mod text;
mod tile;

use game::Game;
use std::fs::create_dir_all;

const HALFTILE_WIDTH: usize = 8;
const HALFTILE_HEIGHT: usize = 8;
const TILE_WIDTH: usize = HALFTILE_WIDTH * 2;
const TILE_HEIGHT: usize = HALFTILE_HEIGHT * 2;
const MAX_TILES_PER_FILE: usize = 50;
const HEALTH_COUNT: usize = 8;
const INVENTORY_WIDTH: usize = HEALTH_COUNT / 2;
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 8;

fn main() -> Result<(), String> {
    println!("Starting FreeNukum…");

    let scale = 2f32;

    let settings = settings::Settings { scale: 2f32 };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let w = (TILE_WIDTH + 2) * MAX_TILES_PER_FILE;
    let h = (TILE_HEIGHT + 2) * tile::Category::all().len();

    let window = video_subsystem
        .window(
            "FreeNukum",
            (scale * w as f32) as u32,
            (scale * h as f32) as u32,
        )
        .position_centered()
        .vulkan()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas =
        window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_scale(scale, scale).unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.clear();

    let path = dirs::data_local_dir()
        .unwrap()
        .join("freenukum")
        .join("data");
    create_dir_all(&path).unwrap();

    let tiles = tile::load(&path, &texture_creator).unwrap();

    let mut game = Game::new(settings, canvas, sdl_context, &tiles);

    game.run()
}
