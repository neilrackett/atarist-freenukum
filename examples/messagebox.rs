// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use anyhow::{Error, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::load_default_font;
use freenukum::messagebox::messagebox;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{game, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};

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

    let msgbox = messagebox(&msg, &tilecache, &texture_creator)?;
    let destrect = Rect::from_center(
        Point::new(WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2),
        msgbox.width(),
        msgbox.height(),
    );
    canvas
        .copy(&msgbox.as_texture(&texture_creator)?, None, destrect)
        .map_err(Error::msg)?;

    canvas.present();

    'event_loop: loop {
        match event_pump.wait_event() {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => break 'event_loop,
            Event::Window {
                win_event: WindowEvent::Exposed,
                ..
            }
            | Event::Window {
                win_event: WindowEvent::Shown,
                ..
            } => canvas.present(),
            _ => {}
        }
    }
    Ok(())
}
