// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::messagebox::messagebox;
use crate::{
    event::{ConfirmEvent, WaitEvent},
    graphics::Picture,
    Result, TileProvider, PICTURE_HEIGHT, PICTURE_WIDTH, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use anyhow::Error;
use rgb::RGBA8;
use sdl2::{
    rect::Rect, render::WindowCanvas, surface::Surface, EventPump,
};
use std::fs::File;
use std::io::Read;

pub fn load<P: Picture>(input: &mut File) -> Result<P> {
    const NUM_LOADS: usize = (PICTURE_WIDTH * PICTURE_HEIGHT) as usize;
    let mut buffer = [0u8; NUM_LOADS];

    let mut pixels = [RGBA8::default();
        PICTURE_WIDTH as usize * PICTURE_HEIGHT as usize * 8];

    // read blue
    input.read_exact(&mut buffer)?;
    for (i, item) in buffer.iter().take(NUM_LOADS).enumerate() {
        for j in 0..8 {
            let p = &mut pixels[i * 8 + j];
            let blue_pixel: u8 = (item >> (7 - j)) & 1;
            p.b = blue_pixel * 0x54 * 2;
        }
    }

    // read green
    input.read_exact(&mut buffer)?;
    for (i, item) in buffer.iter().take(NUM_LOADS).enumerate() {
        for j in 0..8 {
            let p = &mut pixels[i * 8 + j];
            let green_pixel: u8 = (item >> (7 - j)) & 1;
            p.g = green_pixel * 0x54 * 2;
        }
    }

    // read red
    input.read_exact(&mut buffer)?;
    for (i, item) in buffer.iter().take(NUM_LOADS).enumerate() {
        for j in 0..8 {
            let p = &mut pixels[i * 8 + j];
            let red_pixel: u8 = (item >> (7 - j)) & 1;
            p.r = red_pixel * 0x54 * 2;
        }
    }

    // read brighten, and set pixels opaque
    input.read_exact(&mut buffer)?;
    for (i, item) in buffer.iter().take(NUM_LOADS).enumerate() {
        for j in 0..8 {
            let p = &mut pixels[i * 8 + j];
            let bright_pixel: u8 = (item >> (7 - j)) & 1;
            p.r += bright_pixel * 0x54;
            p.g += bright_pixel * 0x54;
            p.b += bright_pixel * 0x54;
            p.a = 0xff;
        }
    }

    P::from_data(WINDOW_WIDTH, WINDOW_HEIGHT, &pixels)
}

pub fn show_splash(
    canvas: &mut WindowCanvas,
    tileprovider: &dyn TileProvider,
    file: &mut File,
    event_pump: &mut EventPump,
) -> Result<()> {
    show_splash_with_message(
        canvas,
        tileprovider,
        file,
        event_pump,
        None,
        0,
        0,
    )
}

pub fn show_splash_with_message(
    canvas: &mut WindowCanvas,
    tileprovider: &dyn TileProvider,
    file: &mut File,
    event_pump: &mut EventPump,
    message: Option<&str>,
    x: i32,
    y: i32,
) -> Result<()> {
    let picture = load::<Surface>(file)?;

    let texture_creator = canvas.texture_creator();

    canvas
        .copy(&picture.as_texture(&texture_creator)?, None, None)
        .map_err(Error::msg)?;

    if let Some(message) = message {
        let messagebox =
            messagebox(message, tileprovider, &texture_creator)?;
        let destrect =
            Rect::new(x, y, messagebox.width(), messagebox.height());

        canvas
            .copy(
                &messagebox.as_texture(&texture_creator)?,
                None,
                destrect,
            )
            .map_err(Error::msg)?;
    }
    canvas.present();

    loop {
        match ConfirmEvent::wait(event_pump)? {
            ConfirmEvent::Confirmed | ConfirmEvent::Aborted => {
                return Ok(())
            }
            ConfirmEvent::RefreshScreen => {
                canvas.present();
            }
        }
    }
}
