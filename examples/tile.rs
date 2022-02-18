// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use anyhow::{Error, Result};
use clap::Parser;
use freenukum::game;
use freenukum::settings::Settings;
use freenukum::tile::{self, TileHeader};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    surface::Surface,
};
use std::fs::File;
use std::path::PathBuf;

/// Show tiles from a Duke Nukem 1 grame graphics file.
#[derive(Parser, Debug)]
struct Arguments {
    /// The path to the file that should be shown.
    /// The file is usually named one of: `anim0.dn1` to `anim5.dn1`,
    /// `border.dn1`, `font1.dn1`, `font2.dn1`, `numbers.dn1`,
    /// `object0.dn1` to `object2.dn1` or `solid0.dn1` to `solid3.dn1`.
    filename: PathBuf,
}

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let args = Arguments::parse();

    let settings = Settings::load_or_create();

    let mut file = File::open(&args.filename)?;
    let header = TileHeader::load_from(&mut file)?;

    let mut r =
        Rect::new(0, 0, header.width as u32 * 8, header.height as u32);

    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(Error::msg)?;
    let window = game::create_window(
        r.width() * header.tiles as u32,
        r.height(),
        settings.fullscreen,
        &format!("Freenukum {} tile example", VERSION),
        &video_subsystem,
    )?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    for _ in 0..header.tiles {
        let tile: Surface = tile::load(&mut file, header, false)?;
        canvas
            .copy(&tile.as_texture(&texture_creator)?, None, r)
            .map_err(Error::msg)?;
        r.set_x(r.x() + header.width as i32 * 8);
    }
    canvas.present();

    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;

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
