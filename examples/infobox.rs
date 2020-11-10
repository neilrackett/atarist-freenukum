use anyhow::Result;
use freenukum::data::original_data_dir;
use freenukum::infobox;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{
    game, sdl_surface_creation_params, WINDOW_HEIGHT, WINDOW_WIDTH,
};

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
        settings.fullscreen,
        format!("Freenukum {} infobox example", VERSION),
        format!("Freenukum {} infobox example", VERSION),
    )?;

    game::check_episodes(&mut screen)?;
    let texture_creation_params = sdl_surface_creation_params(&screen);
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        texture_creation_params,
    )?;

    infobox::show(
        &mut screen,
        &tilecache,
        texture_creation_params,
        "This is...",
    )?;
    infobox::show(
        &mut screen,
        &tilecache,
        texture_creation_params,
        "...the great\nInfobox example.\n",
    )?;
    infobox::show(
        &mut screen,
        &tilecache,
        texture_creation_params,
        "now\nwith\neven\nmore\nlines.",
    )?;

    Ok(())
}
