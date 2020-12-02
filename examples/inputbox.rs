use anyhow::Result;
use freenukum::data::original_data_dir;
use freenukum::graphics::SurfaceCreatorProvider;
use freenukum::inputbox::{self, Answer};
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
        format!("Freenukum {} inputbox example", VERSION),
        format!("Freenukum {} inputbox example", VERSION),
    )?;

    game::check_episodes(&mut screen)?;
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        &screen.surface_creator(),
    )?;

    match inputbox::show(
        &mut screen,
        &tilecache,
        "Please enter your name:",
        30,
    )? {
        Answer::Ok(name) => {
            println!("OK, your name is {:?}", name);
        }
        Answer::Quit => {
            println!("Quit");
        }
    }

    Ok(())
}
