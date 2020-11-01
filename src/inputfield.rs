use super::geometry::Geometry;
use super::text;
use super::texture::Texture;
use super::tilecache::TileCache;
use crate::{FONT_HEIGHT, FONT_WIDTH};

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
        if self.text.len() > 0
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

    pub fn blit(&self, target: &mut Texture, tilecache: &TileCache) {
        target.fill_area(None, 0, 0, 0);
        let sourcerect = Geometry {
            x: 0,
            y: 0,
            w: (FONT_WIDTH * self.cursor_position) as u16,
            h: FONT_HEIGHT as u16,
        };
        text::print(target, sourcerect, tilecache, &self.text);
        let cursorrect = Geometry {
            x: (self.cursor_position * FONT_WIDTH) as i16,
            y: 1,
            w: 1,
            h: FONT_HEIGHT as u16 - 2,
        };
        target.fill_area(Some(cursorrect), 0x88, 0x88, 0x88);
    }

    pub fn get_text(&self) -> &str {
        self.text.as_ref()
    }
}
