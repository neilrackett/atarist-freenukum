use super::tile::{self, TileHeader};
use crate::{
    Result, BACKDROP_HEIGHT, BACKDROP_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use anyhow::anyhow;
use sdl2::{
    pixels::PixelFormatEnum, rect::Rect, render::Canvas, surface::Surface,
};
use std::io::Read;

pub fn load<'t, R: Read>(r: &mut R) -> Result<Surface<'t>> {
    let surface = Surface::new(
        BACKDROP_WIDTH * TILE_WIDTH,
        BACKDROP_HEIGHT * TILE_HEIGHT,
        PixelFormatEnum::RGB888,
    )
    .map_err(|s| anyhow!(s))?;

    let mut geometry = Rect::new(0, 0, TILE_WIDTH, TILE_HEIGHT);

    let header = TileHeader {
        width: 2,
        height: 16,
        tiles: 0,
    };

    let mut canvas =
        Canvas::from_surface(surface).map_err(|s| anyhow!(s))?;
    {
        let texture_creator = canvas.texture_creator();

        for _ in 0..BACKDROP_WIDTH * BACKDROP_HEIGHT {
            let tile = tile::load(r, header, false)?;
            canvas
                .copy(&tile.as_texture(&texture_creator)?, None, geometry)
                .map_err(|s| anyhow!(s))?;

            geometry.x += 16;
            if geometry.x == 16 * BACKDROP_WIDTH as i32 {
                geometry.x = 0;
                geometry.y += 16;
            }
        }
    }
    canvas.present();
    let surface = canvas.into_surface();
    Ok(surface)
}
