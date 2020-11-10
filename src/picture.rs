use super::geometry::Geometry;
use super::messagebox::messagebox;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::event::{ConfirmEvent, WaitEvent};
use crate::{
    Result, PICTURE_HEIGHT, PICTURE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use std::fs::File;
use std::io::Read;
use transdl::video::Surface;

pub fn load(
    input: &mut File,
    params: TextureCreationParams,
) -> Result<Texture> {
    let mut picture = Texture::create_with_params(
        WINDOW_WIDTH as u16,
        WINDOW_HEIGHT as u16,
        params,
    );
    const NUM_LOADS: usize = PICTURE_WIDTH * PICTURE_HEIGHT;
    let mut buffer = [0u8; NUM_LOADS];

    let mut data = [0u8; PICTURE_WIDTH * PICTURE_HEIGHT * 4 * 8];
    let mut index;

    // read blue
    index = 2;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let blue_pixel: u8 = (buffer[i] >> (7 - j)) & 1;
            data[index] += blue_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read green
    index = 1;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let green_pixel: u8 = (buffer[i] >> (7 - j)) & 1;
            data[index] += green_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read red
    index = 0;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let red_pixel: u8 = (buffer[i] >> (7 - j)) & 1;
            data[index] += red_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read brighten, and set pixels opaque
    index = 0;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let bright_pixel: u8 = (buffer[i] >> (7 - j)) & 1;

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

    picture.set_data(&data, params.transparent);

    Ok(picture)
}

pub fn show_splash(
    tilecache: &TileCache,
    texture_creation_params: TextureCreationParams,
    target: &mut Surface,
    file: &mut File,
) -> Result<()> {
    show_splash_with_message(
        tilecache,
        texture_creation_params,
        target,
        file,
        None,
        0,
        0,
    )
}

pub fn show_splash_with_message(
    tilecache: &TileCache,
    texture_creation_params: TextureCreationParams,
    target: &mut Surface,
    file: &mut File,
    message: Option<&str>,
    x: i16,
    y: i16,
) -> Result<()> {
    let picture = load(file, texture_creation_params)?;
    picture.blit_to_sdl_surface(None, target, None);

    if let Some(message) = message {
        let messagebox =
            messagebox(message, tilecache, texture_creation_params);
        let destrect = Geometry {
            x,
            y,
            w: messagebox.width(),
            h: messagebox.height(),
        };
        messagebox.blit_to_sdl_surface(None, target, Some(destrect));
    }
    target.update_rect(0, 0, 0, 0);

    loop {
        match ConfirmEvent::wait()? {
            ConfirmEvent::Confirmed | ConfirmEvent::Aborted => {
                return Ok(())
            }
            ConfirmEvent::RefreshScreen => {
                target.update_rect(0, 0, 0, 0);
            }
        }
    }
}
