use anyhow::{Error, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::load_default_font;
use freenukum::picture;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{game, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::pixels::Color;
use std::fs::File;

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let settings = Settings::load_or_create();
    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(Error::msg)?;
    let ttf_context = sdl2::ttf::init()?;
    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;

    let window = game::create_window(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        settings.fullscreen,
        &format!("Freenukum {} borders example", VERSION),
        &video_subsystem,
    )?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    let episodes = game::check_episodes(
        &mut canvas,
        &load_default_font(&ttf_context)?,
        &texture_creator,
        &mut event_pump,
    )?;
    let tilecache = TileCache::load_from_path(&original_data_dir())?;

    let items = vec![
        ("badguy", "I am the\nBAD GUY!", 50, 144),
        ("duke", "Hello BAD GUY!\nI'm the GOOD GUY.", 100, 144),
        ("dn", "", 0, 0),
        ("end", "The end.", 200, 144),
    ];

    for item in items {
        let (file, text, x, y) = item;

        let filename = format!("{}.{}", file, episodes.file_extension());
        let filepath = original_data_dir().join(filename);
        let mut file = File::open(filepath)?;

        if text == "" {
            picture::show_splash(
                &mut canvas,
                &tilecache,
                &mut file,
                &mut event_pump,
            )?;
        } else {
            picture::show_splash_with_message(
                &mut canvas,
                &tilecache,
                &mut file,
                &mut event_pump,
                Some(text),
                x,
                y,
            )?;
        }
    }
    Ok(())
}
