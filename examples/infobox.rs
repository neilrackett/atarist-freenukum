use anyhow::Result;
use freenukum::data::original_data_dir;
use freenukum::graphics::SurfaceCreatorProvider;
use freenukum::infobox;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{game, WINDOW_HEIGHT, WINDOW_WIDTH};

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
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        &screen.surface_creator(),
    )?;

    infobox::show(&mut screen, &tilecache, "This is...")?;
    infobox::show(
        &mut screen,
        &tilecache,
        "...the great\nInfobox example.\n",
    )?;
    infobox::show(
        &mut screen,
        &tilecache,
        "now\nwith\neven\nmore\nlines.",
    )?;

    Ok(())
}
