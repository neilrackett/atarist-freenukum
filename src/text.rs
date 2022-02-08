// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::rendering::{MovePositionRenderer, Renderer};
use crate::{
    Result, FONT_ASCII_LOWERCASE, FONT_ASCII_UPPERCASE, FONT_HEIGHT,
    FONT_QUESTIONMARK, FONT_WIDTH,
};
use sdl2::rect::Point;

fn render_letter(renderer: &mut dyn Renderer, letter: char) -> Result<()> {
    let tilenr = match letter {
        c if c >= ' ' && c <= 'Z' => {
            c as usize - ' ' as usize + FONT_ASCII_UPPERCASE
        }
        c if c >= 'a' && c <= 'z' => {
            c as usize - 'a' as usize + FONT_ASCII_LOWERCASE
        }
        _ => FONT_QUESTIONMARK,
    } as usize;
    renderer.place_tile(tilenr, Point::new(0, 0))
}

pub fn render(renderer: &mut dyn Renderer, text: &str) -> Result<()> {
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
            render_letter(&mut renderer, c)?;
            offset_x += FONT_WIDTH as i32;
        }
    }
    Ok(())
}
