use anyhow::{anyhow, Result};
use freenukum::backdrop;
use freenukum::settings::Settings;
use freenukum::tile::TileHeader;
use freenukum::{
    game, BACKDROP_HEIGHT, BACKDROP_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    surface::Surface,
};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

/// Show an original Duke Nukem 1 backdrop.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the file that should be shown.
    /// The file is usually named `drop1.dn1` or similar.
    filename: PathBuf,
}

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let args = Arguments::from_args();

    let mut file = File::open(&args.filename)?;

    let settings = Settings::load_or_create();
    let sdl_context = sdl2::init().map_err(|s| anyhow!(s))?;
    let video_subsystem = sdl_context.video().map_err(|s| anyhow!(s))?;
    let mut event_pump =
        sdl_context.event_pump().map_err(|s| anyhow!(s))?;

    let window = game::create_window(
        BACKDROP_WIDTH * TILE_WIDTH,
        BACKDROP_HEIGHT * TILE_HEIGHT,
        settings.fullscreen,
        &format!("Freenukum {} backdrop example", VERSION),
        &video_subsystem,
    )?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    TileHeader::load_from(&mut file)?;
    let backdrop: Surface = backdrop::load(&mut file)?;
    canvas
        .copy(&backdrop.as_texture(&texture_creator)?, None, None)
        .map_err(|s| anyhow!(s))?;
    canvas.present();

    'event_loop: loop {
        match event_pump.wait_event() {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => break 'event_loop,
            Event::Window {
                win_event: WindowEvent::Exposed,
                ..
            }
            | Event::Window {
                win_event: WindowEvent::Shown,
                ..
            } => canvas.present(),
            _ => {}
        }
    }
    Ok(())
}
