// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use anyhow::{Error, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::load_default_font;
use freenukum::inputbox::{self, Answer};
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{game, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::pixels::Color;

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
        &format!("Freenukum {} backdrop example", VERSION),
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

    match inputbox::show(
        &mut canvas,
        &tilecache,
        "Please enter your name:",
        30,
        &mut event_pump,
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
