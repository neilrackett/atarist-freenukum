use super::text;
use crate::rendering::{CanvasRenderer, MovePositionRenderer, Renderer};
use crate::{
    Result, TileProvider, BORDER_BLUE_BOTTOM, BORDER_BLUE_BOTTOMLEFT,
    BORDER_BLUE_BOTTOMRIGHT, BORDER_BLUE_LEFT, BORDER_BLUE_MIDDLE,
    BORDER_BLUE_RIGHT, BORDER_BLUE_TOP, BORDER_BLUE_TOPLEFT,
    BORDER_BLUE_TOPRIGHT, FONT_HEIGHT, FONT_WIDTH,
};
use anyhow::Error;
use sdl2::{
    rect::Point,
    render::{Canvas, TextureCreator},
    surface::Surface,
};

pub fn get_information(text: &str) -> (usize, usize) {
    let mut columns = 0;
    let mut rows = 0;

    for line in text.lines() {
        columns = std::cmp::max(columns, line.len());
        rows += 1;
    }
    (columns, rows)
}

pub fn messagebox<'t, T>(
    text: &str,
    tileprovider: &dyn TileProvider,
    texture_creator: &TextureCreator<T>,
) -> Result<Surface<'t>> {
    let (columns, rows) = get_information(text);

    let surface = Surface::new(
        FONT_WIDTH * (columns as u32 + 2),
        FONT_HEIGHT * (rows as u32 + 2),
        texture_creator.default_pixel_format(),
    )
    .map_err(Error::msg)?;

    let mut canvas = Canvas::from_surface(surface).map_err(Error::msg)?;

    {
        let texture_creator = canvas.texture_creator();
        let mut renderer = CanvasRenderer {
            canvas: &mut canvas,
            texture_creator: &texture_creator,
            tileprovider,
        };

        for row in 0..=rows {
            for col in 0..=columns {
                let tilenr = match (row, col) {
                    (0, 0) => BORDER_BLUE_TOPLEFT,
                    (0, c) if c == columns => BORDER_BLUE_TOPRIGHT,
                    (r, 0) if r == rows => BORDER_BLUE_BOTTOMLEFT,
                    (r, c) if r == rows && c == columns => {
                        BORDER_BLUE_BOTTOMRIGHT
                    }
                    (0, _) => BORDER_BLUE_TOP,
                    (_, 0) => BORDER_BLUE_LEFT,
                    (r, _) if r == rows => BORDER_BLUE_BOTTOM,
                    (_, c) if c == columns => BORDER_BLUE_RIGHT,
                    _ => BORDER_BLUE_MIDDLE,
                };

                let point = Point::new(
                    col as i32 * FONT_WIDTH as i32,
                    row as i32 * FONT_HEIGHT as i32,
                );

                renderer.place_tile(tilenr, point)?;
            }
        }

        let mut move_renderer = MovePositionRenderer {
            offset_x: FONT_WIDTH as i32,
            offset_y: FONT_HEIGHT as i32,
            upstream: &mut renderer,
        };
        text::render(&mut move_renderer, text)?;
    }
    canvas.present();

    let surface = canvas.into_surface();

    Ok(surface)
}
