use crate::rendering::{MovePositionRenderer, Renderer};
use crate::{
    FONT_ASCII_LOWERCASE, FONT_ASCII_UPPERCASE, FONT_HEIGHT,
    FONT_QUESTIONMARK, FONT_WIDTH,
};
use transdl::video::Rect;

fn render_letter(renderer: &mut dyn Renderer, letter: char) {
    let tilenr = match letter {
        c if c >= ' ' && c <= 'Z' => {
            c as usize - ' ' as usize + FONT_ASCII_UPPERCASE
        }
        c if c >= 'a' && c <= 'z' => {
            c as usize - 'a' as usize + FONT_ASCII_LOWERCASE
        }
        _ => FONT_QUESTIONMARK,
    } as usize;
    renderer.place_tile(
        tilenr,
        Rect {
            x: 0,
            y: 0,
            w: FONT_WIDTH as u16,
            h: FONT_HEIGHT as u16,
        },
    );
}

pub fn render(renderer: &mut dyn Renderer, text: &str) {
    let mut offset_x = 0;
    let mut offset_y = 0;

    for c in text.chars() {
        if c == '\n' {
            offset_x = 0;
            offset_y += FONT_HEIGHT as i32;
        } else {
            let mut renderer = MovePositionRenderer {
                offset_x,
                offset_y,
                upstream: renderer,
            };
            render_letter(&mut renderer, c);
            offset_x += FONT_WIDTH as i32;
        }
    }
}
