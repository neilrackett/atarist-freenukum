// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::tile::{self, TileHeader};
use crate::{
    graphics::Picture, Result, BACKDROP_HEIGHT, BACKDROP_WIDTH,
    TILE_HEIGHT, TILE_WIDTH,
};
use rgb::RGBA8;
use std::io::Read;

pub fn load<R: Read, P: Picture>(r: &mut R) -> Result<P> {
    const W: u32 = BACKDROP_WIDTH * TILE_WIDTH;
    const H: u32 = BACKDROP_HEIGHT * TILE_HEIGHT;
    let mut pixels = [RGBA8::default(); (W * H) as usize];

    let header = TileHeader {
        width: 2,
        height: 16,
        tiles: 0,
    };

    for tile_y in 0..BACKDROP_HEIGHT {
        for tile_x in 0..BACKDROP_WIDTH {
            let x = tile_x * TILE_WIDTH;
            let y = tile_y * TILE_HEIGHT;

            let tile: Vec<RGBA8> = tile::load(r, header, false)?;

            for row in 0..TILE_HEIGHT {
                let target_start = ((y + row) * W + x) as usize;
                let target_end = target_start + TILE_WIDTH as usize;

                let source_start = (row * TILE_WIDTH) as usize;
                let source_end = source_start + TILE_WIDTH as usize;
                let source = &tile[source_start..source_end];

                pixels[target_start..target_end].copy_from_slice(source)
            }
        }
    }
    P::from_data(W, H, &pixels)
}
