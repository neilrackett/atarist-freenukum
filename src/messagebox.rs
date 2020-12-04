use super::text;
use super::tilecache::TileCache;
use crate::graphics::SurfaceCreator;
use crate::rendering::{MovePositionRenderer, Renderer, SurfaceRenderer};
use crate::{
    BORDER_BLUE_BOTTOM, BORDER_BLUE_BOTTOMLEFT, BORDER_BLUE_BOTTOMRIGHT,
    BORDER_BLUE_LEFT, BORDER_BLUE_MIDDLE, BORDER_BLUE_RIGHT,
    BORDER_BLUE_TOP, BORDER_BLUE_TOPLEFT, BORDER_BLUE_TOPRIGHT,
    FONT_HEIGHT, FONT_WIDTH,
};
use transdl::video::{Rect, Surface};

pub fn get_information(text: &str) -> (usize, usize) {
    let mut columns = 0;
    let mut rows = 0;

    for line in text.lines() {
        columns = std::cmp::max(columns, line.len());
        rows += 1;
    }
    (columns, rows)
}

pub fn messagebox(
    text: &str,
    tilecache: &TileCache,
    surface_creator: &mut dyn SurfaceCreator,
) -> Surface {
    let (columns, rows) = get_information(text);

    let mut messagebox = surface_creator.create(
        (FONT_WIDTH * (columns + 2)) as u32,
        (FONT_HEIGHT * (rows + 2)) as u32,
    );
    let mut renderer = SurfaceRenderer {
        target: &mut messagebox,
        tilecache,
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

            let r = Rect {
                x: col as i16 * FONT_WIDTH as i16,
                y: row as i16 * FONT_HEIGHT as i16,
                w: FONT_WIDTH as u16,
                h: FONT_HEIGHT as u16,
            };

            renderer.place_tile(tilenr, r);
        }
    }

    let mut move_renderer = MovePositionRenderer {
        offset_x: FONT_WIDTH as i32,
        offset_y: FONT_HEIGHT as i32,
        upstream: &mut renderer,
    };
    text::render(&mut move_renderer, text);

    messagebox
}
