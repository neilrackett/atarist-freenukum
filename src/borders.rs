use super::{HALFTILE_HEIGHT, HALFTILE_WIDTH};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery};
use sdl2::surface::Surface;

pub struct Borders {}

fn borders_blit_tile(
    pixelsize: u8,
    target: &Surface,
    tile: &Texture,
    x: i32,
    y: i32,
) {
    let TextureQuery { width, height, .. } = tile.query();

    let dst = Rect::new(
        HALFTILE_WIDTH as i32 * x,
        HALFTILE_HEIGHT as i32 * y,
        width,
        height,
    );

    //tile.blit(None, target, dst);

    unimplemented!();
}

enum BorderType {
    None,
    TopLeft,
    TopRight,
    TopT,
    BottomLeft,
    BottomRight,
    BottomT,
    LeftT,
    RightT,
}
