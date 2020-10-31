use anyhow::{anyhow, Result};
use freenukum::transition::backdrop;
use freenukum::transition::settings::Settings;
use freenukum::transition::tile::TileHeader;
use freenukum::transition::{game, sdl_surface_creation_params};
use freenukum::{
    BACKDROP_HEIGHT, BACKDROP_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use transdl::event::{Event, KeyCode};

/// Show an original Duke Nukem 1 backdrop.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the file that should be shown.
    /// The file is usually named `drop1.dn1` or similar.
    filename: PathBuf,
}

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let args = Arguments::from_args();

    let mut file = File::open(&args.filename)?;

    let settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        (BACKDROP_WIDTH * TILE_WIDTH) as i32,
        (BACKDROP_HEIGHT * TILE_HEIGHT) as i32,
        settings.fullscreen,
        format!("Freenukum {} backdrop example", VERSION),
        format!("Freenukum {} backdrop example", VERSION),
    )?;

    TileHeader::load_from(&mut file)?;
    let texture_creation_params = sdl_surface_creation_params(&screen);
    let backdrop = backdrop::load(&mut file, texture_creation_params)?;
    backdrop.blit_to_sdl_surface(None, &mut screen, None);
    screen.update();

    'event_loop: loop {
        match Event::wait().map_err(|e| anyhow!("{}", e))? {
            Event::Quit
            | Event::KeyDown {
                key: Some(KeyCode::Escape),
                ..
            }
            | Event::KeyDown {
                key: Some(KeyCode::Q),
                ..
            } => break 'event_loop,
            Event::VideoExpose => screen.update(),
            _ => {}
        }
    }
    Ok(())
}
