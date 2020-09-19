use freenukum::{
    tile, Game, Settings, MAX_TILES_PER_FILE, TILE_HEIGHT, TILE_WIDTH,
};
use std::fs::create_dir_all;

fn main() -> Result<(), String> {
    println!("Starting FreeNukum…");

    let scale = 2f32;

    let settings = Settings { scale: 2f32 };

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
