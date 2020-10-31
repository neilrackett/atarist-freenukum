use anyhow::{anyhow, Result};
use freenukum::transition::data::original_data_dir;
use freenukum::transition::messagebox::messagebox;
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
        format!("Freenukum {} messagebox example", VERSION),
        format!("Freenukum {} messagebox example", VERSION),
    )?;

    game::check_episodes(&mut screen);
    let texture_creation_params = sdl_surface_creation_params(&screen);
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        texture_creation_params,
    )?;

    let msg = r" FREENUKUM MAIN MENU
 ------------------- 

S)tart a new game
R)estore an old game
I)nstructions
O)rdering information
G)ame setup
H)igh scores
P)reviews/Main Demo!
V)iew user demo
T)itle screen
C)redits
Q)it to DOS";

    let msgbox = messagebox(&msg, &tilecache, texture_creation_params);
    msgbox.blit_to_sdl_surface(None, &mut screen, None);
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
