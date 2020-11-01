use anyhow::{anyhow, Result};
use freenukum::picture;
use freenukum::settings::Settings;
use freenukum::{
    game, sdl_surface_creation_params, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use transdl::event::{Event, KeyCode};

/// Show an original Duke Nukem 1 game picture.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the file that should be shown.
    /// The file is usually named one of: `badguy.dn1`, `credits.dn1`,
    /// `dn.dn1`, `duke.dn1`, `end.dn1`.
    filename: PathBuf,
}

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let args = Arguments::from_args();

    let mut file = File::open(&args.filename)?;

    let settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
        settings.fullscreen,
        format!("Freenukum {} picture example", VERSION),
        format!("Freenukum {} picture example", VERSION),
    )?;

    let texture_creation_params = sdl_surface_creation_params(&screen);
    let picture = picture::load(&mut file, texture_creation_params)?;

    picture.blit_to_sdl_surface(None, &mut screen, None);
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
