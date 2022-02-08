// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use anyhow::{Error, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::load_default_font;
use freenukum::rendering::{
    CanvasRenderer, MovePositionRenderer, Renderer,
};
use freenukum::settings::Settings;
use freenukum::text;
use freenukum::tilecache::{FileProperties, TileCache};
use freenukum::{game, DefaultSizes, Sizes};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    rect::Point,
};

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let file_properties = FileProperties::get_all();
    let max_tiles =
        file_properties.iter().map(|p| p.num_tiles).max().unwrap();

    let settings = Settings::load_or_create();
    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(Error::msg)?;
    let ttf_context = sdl2::ttf::init()?;
    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;

    let sizes = DefaultSizes;
    let window = game::create_window(
        (max_tiles + 2) as u32 * sizes.width(),
        (file_properties.len() as u32 + 2) * sizes.height(),
        settings.fullscreen,
        &format!("Freenukum {} tilecache example", VERSION),
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

    let mut dest = Point::new(sizes.width() as i32, 0);

    let mut renderer = CanvasRenderer {
        canvas: &mut canvas,
        texture_creator: &texture_creator,
        tileprovider: &tilecache,
    };

    let blithex = |value: usize,
                   dest: Point,
                   renderer: &mut dyn Renderer|
     -> Result<()> {
        let mut renderer = MovePositionRenderer {
            offset_x: dest.x(),
            offset_y: dest.y(),
            upstream: renderer,
        };
        text::render(&mut renderer, &format!("{:02X}", value))
    };

    for x in 0..max_tiles {
        blithex(x, dest, &mut renderer)?;
        dest.x += sizes.width() as i32;
    }
    dest.x = 0;
    dest.y += sizes.height() as i32;

    let mut i = 0;
    for (row, file) in FileProperties::get_all().iter().enumerate() {
        blithex(row, dest, &mut renderer)?;
        dest.x += sizes.width() as i32;
        for _ in 0..file.num_tiles {
            renderer.place_tile(i, dest)?;
            dest.x += sizes.width() as i32;
            i += 1;
        }
        dest.x = (sizes.width() * (max_tiles as u32 + 1)) as i32;
        blithex(row, dest, &mut renderer)?;
        dest.x = 0;
        dest.y += sizes.height() as i32;
    }
    dest.x += sizes.width() as i32;
    for x in 0..max_tiles {
        blithex(x, dest, &mut renderer)?;
        dest.x += sizes.width() as i32;
    }

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
