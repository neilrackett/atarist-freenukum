use super::tile::{self, TileHeader};
use crate::graphics::SurfaceCreator;
use crate::{
    Result, BACKDROP_HEIGHT, BACKDROP_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use std::io::Read;
use transdl::video::{Rect, Surface};

pub fn load<R: Read>(
    r: &mut R,
    surface_creator: &dyn SurfaceCreator,
) -> Result<Surface> {
    let mut backdrop = surface_creator.create(
        BACKDROP_WIDTH as u32 * TILE_WIDTH as u32,
        BACKDROP_HEIGHT as u32 * TILE_HEIGHT as u32,
    );

    let mut geometry = Rect {
        x: 0,
        y: 0,
        w: TILE_WIDTH as u16,
        h: TILE_HEIGHT as u16,
    };

    let header = TileHeader {
        width: 2,
        height: 16,
        tiles: 0,
    };

    for _ in 0..BACKDROP_WIDTH * BACKDROP_HEIGHT {
        let tile = tile::load(r, surface_creator, header.clone(), false)?;
        tile.blit(None, &mut backdrop, Some(geometry));

        geometry.x += 16;
        if geometry.x == 16 * BACKDROP_WIDTH as i16 {
            geometry.x = 0;
            geometry.y += 16;
        }
    }

    Ok(backdrop)
}
