use super::messagebox::messagebox;
use crate::event::{ConfirmEvent, WaitEvent};
use crate::{
    Result, TileProvider, PICTURE_HEIGHT, PICTURE_WIDTH, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use anyhow::anyhow;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, WindowCanvas},
    surface::Surface,
    EventPump,
};
use std::fs::File;
use std::io::Read;

pub fn load<'t>(input: &mut File) -> Result<Surface<'t>> {
    let surface =
        Surface::new(WINDOW_WIDTH, WINDOW_HEIGHT, PixelFormatEnum::RGB888)
            .map_err(|s| anyhow!(s))?;

    const NUM_LOADS: usize = (PICTURE_WIDTH * PICTURE_HEIGHT) as usize;
    let mut buffer = [0u8; NUM_LOADS];

    let mut data =
        [0u8; PICTURE_WIDTH as usize * PICTURE_HEIGHT as usize * 4 * 8];
    let mut index;

    // read blue
    index = 2;
    input.read_exact(&mut buffer)?;
    for item in buffer.iter().take(NUM_LOADS) {
        for j in 0..8 {
            let blue_pixel: u8 = (item >> (7 - j)) & 1;
            data[index] += blue_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read green
    index = 1;
    input.read_exact(&mut buffer)?;
    for item in buffer.iter().take(NUM_LOADS) {
        for j in 0..8 {
            let green_pixel: u8 = (item >> (7 - j)) & 1;
            data[index] += green_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read red
    index = 0;
    input.read_exact(&mut buffer)?;
    for item in buffer.iter().take(NUM_LOADS) {
        for j in 0..8 {
            let red_pixel: u8 = (item >> (7 - j)) & 1;
            data[index] += red_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read brighten, and set pixels opaque
    index = 0;
    input.read_exact(&mut buffer)?;
    for item in buffer.iter().take(NUM_LOADS) {
        for j in 0..8 {
            let bright_pixel: u8 = (item >> (7 - j)) & 1;

            // brighten red
            data[index] += bright_pixel * 0x54;
            index += 1;

            // brighten blue
            data[index] += bright_pixel * 0x54;
            index += 1;

            // brighten green
            data[index] += bright_pixel * 0x54;
            index += 1;

            // set opaque
            data[index] = 0xff;
            index += 1;
        }
    }

    use crate::graphics::SurfaceExt;
    let mut canvas =
        Canvas::from_surface(surface).map_err(|s| anyhow!(s))?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_data(&data, PICTURE_WIDTH * 8, PICTURE_HEIGHT)?;
    let surface = canvas.into_surface();

    Ok(surface)
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
    let picture = load(file)?;

    let texture_creator = canvas.texture_creator();

    canvas
        .copy(&picture.as_texture(&texture_creator)?, None, None)
        .map_err(|s| anyhow!(s))?;

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
            .map_err(|s| anyhow!(s))?;
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
