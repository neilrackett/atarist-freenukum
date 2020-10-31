use anyhow::{anyhow, Result};
use freenukum::transition::borders::Borders;
use freenukum::transition::data::original_data_dir;
use freenukum::transition::settings::Settings;
use freenukum::transition::tilecache::TileCache;
use freenukum::transition::{game, sdl_surface_creation_params};
use freenukum::{WINDOW_HEIGHT, WINDOW_WIDTH};
use transdl::event::{Event, KeyCode};

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
        settings.fullscreen,
        format!("Freenukum {} borders example", VERSION),
        format!("Freenukum {} borders example", VERSION),
    )?;

    game::check_episodes(&mut screen);
    let texture_creation_params = sdl_surface_creation_params(&screen);
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        texture_creation_params,
    )?;

    let borders = Borders {};
    borders.blit(&mut screen, texture_creation_params, &tilecache);
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
