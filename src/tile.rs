use crate::{graphics::Picture, Result};
use rgb::RGBA8;
use std::io::Read;

#[derive(Clone, Copy)]
pub struct TileHeader {
    pub tiles: u8,
    pub width: u8,
    pub height: u8,
}

impl TileHeader {
    pub fn load_from<R: Read>(r: &mut R) -> Result<Self> {
        let mut buf = [0u8; 3];
        r.read_exact(&mut buf)?;
        Ok(TileHeader {
            tiles: buf[0],
            width: buf[1],
            height: buf[2],
        })
    }
}

pub fn load<R: Read, P: Picture>(
    r: &mut R,
    header: TileHeader,
    has_transparency: bool,
) -> Result<P> {
    let width: u32 = header.width as u32 * 8;
    let height: u32 = header.height as u32;

    let mut pixels: Vec<RGBA8> =
        Vec::with_capacity(width as usize * height as usize);

    let mut readbuf = [0u8; 5];

    let num_loads = width as usize * height as usize / 8;

    for _ in 0..num_loads {
        r.read_exact(&mut readbuf)?;

        let opaque_row = readbuf[0];
        let blue_row = readbuf[1];
        let green_row = readbuf[2];
        let red_row = readbuf[3];
        let bright_row = readbuf[4];

        for i in 0..8 {
            let bright_pixel = (bright_row >> (7 - i)) & 1;
            let red_pixel = (red_row >> (7 - i)) & 1;
            let green_pixel = (green_row >> (7 - i)) & 1;
            let blue_pixel = (blue_row >> (7 - i)) & 1;
            let opaque_pixel = if has_transparency {
                (opaque_row >> (7 - i)) & 1
            } else {
                1
            };
            let ugly_yellow = if red_pixel == 1
                && green_pixel == 1
                && blue_pixel == 0
                && bright_pixel == 0
            {
                1
            } else {
                0
            };

            let pixel = RGBA8::new_alpha(
                0x54 * (red_pixel * 2 + bright_pixel),
                0x54 * (green_pixel * 2 + bright_pixel - ugly_yellow),
                0x54 * (blue_pixel * 2 + bright_pixel),
                opaque_pixel * 0xff,
            );
            pixels.push(pixel);
        }
    }

    P::from_data(width, height, &pixels)
}
