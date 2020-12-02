use anyhow::{anyhow, Result};
use freenukum::borders::Borders;
use freenukum::data::original_data_dir;
use freenukum::graphics::SurfaceCreatorProvider;
use freenukum::rendering::SurfaceRenderer;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{game, WINDOW_HEIGHT, WINDOW_WIDTH};
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

    game::check_episodes(&mut screen)?;
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        &screen.surface_creator(),
    )?;

    let borders = Borders {};
    let mut border_renderer = SurfaceRenderer {
        target: &mut screen,
        tilecache: &tilecache,
    };
    borders.render(&mut border_renderer);
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
