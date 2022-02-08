// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::text;
use crate::rendering::Renderer;
use crate::Result;
use crate::{FONT_HEIGHT, FONT_WIDTH};
use sdl2::{pixels::Color, rect::Rect};

pub struct InputField {
    text: String,
    max_length: usize,
    cursor_position: usize,
}

impl InputField {
    pub fn new(max_length: usize) -> Self {
        InputField {
            text: String::new(),
            max_length,
            cursor_position: 0,
        }
    }

    pub fn backspace_pressed(&mut self) {
        if self.cursor_position > 0
            && self.cursor_position <= self.text.len()
        {
            self.cursor_position -= 1;
            self.text.remove(self.cursor_position);
        }
    }

    pub fn delete_pressed(&mut self) {
        if !self.text.is_empty()
            && self.cursor_position < self.text.len() - 1
        {
            self.text.remove(self.cursor_position);
        }
    }

    pub fn left_pressed(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn right_pressed(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }

    pub fn symbol_pressed(&mut self, symbol: char) {
        if self.text.len() < self.max_length {
            if self.cursor_position == self.text.len() {
                self.text.push(symbol);
            } else {
                self.text.insert(self.cursor_position, symbol);
            }
            self.cursor_position += 1;
        }
    }

    pub fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        let bgrect = Rect::new(
            0,
            0,
            FONT_WIDTH * self.max_length as u32,
            FONT_HEIGHT,
        );
        renderer.fill_rect(bgrect, Color::RGB(0, 0, 0))?;
        text::render(renderer, &self.text)?;
        let cursorrect = Rect::new(
            self.cursor_position as i32 * FONT_WIDTH as i32,
            1,
            1,
            FONT_HEIGHT,
        );
        renderer.fill_rect(cursorrect, Color::RGB(0x88, 0x88, 0x88))?;
        Ok(())
    }

    pub fn get_text(&self) -> &str {
        self.text.as_ref()
    }
}
