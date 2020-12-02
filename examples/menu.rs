use anyhow::Result;
use freenukum::data::original_data_dir;
use freenukum::graphics::SurfaceCreatorProvider;
use freenukum::menu::{Menu, MenuEntry};
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
        format!("Freenukum {} menu example", VERSION),
        format!("Freenukum {} menu example", VERSION),
    )?;

    game::check_episodes(&mut screen)?;
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        &screen.surface_creator(),
    )?;

    let mut menu = Menu::new("Testmenu\nTest\nTest".to_string());
    menu.append(MenuEntry {
        shortcut: 's',
        name: "S)tart game".to_string(),
    });
    menu.append(MenuEntry {
        shortcut: 'h',
        name: "H)ello".to_string(),
    });
    menu.append(MenuEntry {
        shortcut: 'd',
        name: "D)emo game".to_string(),
    });

    println!(
        "Menu choice: {:?}",
        menu.get_choice(&mut screen, &tilecache,)
    );

    Ok(())
}
