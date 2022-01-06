use anyhow::{Error, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::load_default_font;
use freenukum::menu::{Menu, MenuEntry};
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{game, UserEvent, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::pixels::Color;

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let settings = Settings::load_or_create();
    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(Error::msg)?;
    let ttf_context = sdl2::ttf::init()?;
    let event_subsystem = sdl_context.event().map_err(Error::msg)?;
    let timer_subsystem = sdl_context.timer().map_err(Error::msg)?;
    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;

    event_subsystem
        .register_custom_event::<UserEvent>()
        .map_err(Error::msg)?;

    let window = game::create_window(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        settings.fullscreen,
        &format!("Freenukum {} menu example", VERSION),
        &video_subsystem,
    )?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    game::check_episodes(
        &mut canvas,
        &load_default_font(&ttf_context)?,
        &texture_creator,
        &mut event_pump,
    )?;
    let tilecache = TileCache::load_from_path(&original_data_dir())?;

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
        menu.get_choice(
            &mut canvas,
            &tilecache,
            &mut event_pump,
            &event_subsystem.event_sender(),
            &timer_subsystem
        )
    );

    Ok(())
}
