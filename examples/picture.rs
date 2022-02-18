// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use anyhow::{Error, Result};
use clap::Parser;
use freenukum::picture;
use freenukum::settings::Settings;
use freenukum::{game, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    surface::Surface,
};
use std::fs::File;
use std::path::PathBuf;

/// Show an original Duke Nukem 1 game picture.
#[derive(Parser, Debug)]
struct Arguments {
    /// The path to the file that should be shown.
    /// The file is usually named one of: `badguy.dn1`, `credits.dn1`,
    /// `dn.dn1`, `duke.dn1`, `end.dn1`.
    filename: PathBuf,
}

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let args = Arguments::parse();

    let mut file = File::open(&args.filename)?;

    let settings = Settings::load_or_create();
    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(Error::msg)?;
    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;

    let window = game::create_window(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        settings.fullscreen,
        &format!("Freenukum {} picture example", VERSION),
        &video_subsystem,
    )?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    let picture: Surface = picture::load(&mut file)?;

    canvas
        .copy(&picture.as_texture(&texture_creator)?, None, None)
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
