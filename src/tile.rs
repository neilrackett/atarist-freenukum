use crate::Result;
use anyhow::anyhow;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::Canvas,
    surface::Surface,
};
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

pub fn load<'t, R: Read>(
    r: &mut R,
    header: TileHeader,
    has_transparency: bool,
) -> Result<Surface<'t>> {
    let width: u32 = header.width as u32 * 8;
    let height: u32 = header.height as u32;

    let surface = Surface::new(width, height, PixelFormatEnum::RGBA8888)
        .map_err(|s| anyhow!(s))?;
    let mut data: Vec<u8> =
        Vec::with_capacity(width as usize * height as usize * 4);

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

            data.push(0x54 * (red_pixel * 2 + bright_pixel));
            data.push(
                0x54 * (green_pixel * 2 + bright_pixel - ugly_yellow),
            );
            data.push(0x54 * (blue_pixel * 2 + bright_pixel));
            data.push(opaque_pixel * 0xff);
        }
    }

    use crate::graphics::SurfaceExt;
    let mut canvas =
        Canvas::from_surface(surface).map_err(|s| anyhow!(s))?;
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    canvas.clear();
    canvas.set_data(&data, width, height)?;
    let surface = canvas.into_surface();

    Ok(surface)
}
