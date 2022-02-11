// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::Result;
use anyhow::{bail, Error};
use log::debug;
use rgb::RGBA8;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    surface::Surface,
    ttf::{Font, FontStyle, Sdl2TtfContext},
};
use std::path::Path;

trait AsSdlColor {
    fn as_sdl_color(&self) -> Color;
}

impl AsSdlColor for RGBA8 {
    fn as_sdl_color(&self) -> Color {
        Color::RGBA(self.r, self.g, self.b, self.a)
    }
}

pub fn load_default_font(ttf_context: &Sdl2TtfContext) -> Result<Font> {
    #[cfg(target_os = "windows")]
    let (base_paths, font_paths) = (
        vec![Path::new(
            &std::env::var("WINDIR").unwrap_or("C:/Windows".to_string()),
        )
        .to_path_buf()],
        vec![
            Path::new("Fonts").join("arial.ttf").to_path_buf(),
            Path::new("Fonts").join("verdana.ttf").to_path_buf(),
            Path::new("Fonts").join("courier.ttf").to_path_buf(),
            Path::new("Fonts").join("tahoma.ttf").to_path_buf(),
            Path::new("Fonts").join("times.ttf").to_path_buf(),
        ],
    );

    #[cfg(target_os = "linux")]
    let (base_paths, font_paths) = (
        vec![
            Path::new(
                &std::env::var("WINDIR")
                    .unwrap_or_else(|_| "C:/Windows".to_string()),
            )
            .to_path_buf(),
            Path::new("/usr/share/fonts").to_path_buf(),
            Path::new("/usr/share/fonts/truetype").to_path_buf(),
        ],
        vec![
            Path::new("dejavu/DejaVuSans.ttf").to_path_buf(),
            Path::new("dejavu-sans-fonts/DejaVuSans.ttf").to_path_buf(),
        ],
    );

    #[cfg(target_os = "macos")]
    let (base_paths, font_paths) = (
        vec![
            Path::new("/System/Library/Fonts/Base").to_path_buf(),
            Path::new("/System/Library/Fonts").to_path_buf(),
            Path::new("/Library/Fonts").to_path_buf(),
        ],
        vec![
            Path::new("SFCompact.ttf").to_path_buf(),
            Path::new("NewYork.ttf").to_path_buf(),
            Path::new("SFCompactText.ttf").to_path_buf(),
            Path::new("Georgia.ttf").to_path_buf(),
            Path::new("Courier New.ttf").to_path_buf(),
            Path::new("Arial.ttf").to_path_buf(),
        ],
    );

    for base_path in base_paths.iter() {
        for font_path in font_paths.iter() {
            let full_path = base_path.join(font_path);
            debug!("Attempting to load font from {:?}", full_path);
            if full_path.exists() {
                if let Ok(mut font) = ttf_context.load_font(full_path, 10)
                {
                    debug!("Font loaded");
                    font.set_style(FontStyle::BOLD);
                    return Ok(font);
                }
                debug!("Not a valid font file");
            } else {
                debug!("File doesn't exist");
            }
        }
    }
    bail!("No usable font found");
}

pub trait Picture: Sized {
    fn create(width: u32, height: u32) -> Result<Self>;
    fn load_data(&mut self, pixels: &[RGBA8]) -> Result<()>;

    fn from_data(
        width: u32,
        height: u32,
        pixels: &[RGBA8],
    ) -> Result<Self> {
        let mut p = Self::create(width, height)?;
        p.load_data(pixels)?;
        Ok(p)
    }
}

impl Picture for Surface<'_> {
    fn create(width: u32, height: u32) -> Result<Self> {
        let mut surface =
            Surface::new(width, height, PixelFormatEnum::RGBA8888)
                .map_err(Error::msg)?;
        surface
            .fill_rect(None, RGBA8::default().as_sdl_color())
            .map_err(Error::msg)?;
        Ok(surface)
    }

    fn load_data(&mut self, pixels: &[RGBA8]) -> Result<()> {
        for i in 0..self.height() {
            for j in 0..self.width() {
                let color =
                    pixels[(i * self.width() + j) as usize].as_sdl_color();
                let rect = Rect::new(j as i32, i as i32, 1, 1);
                self.fill_rect(rect, color).map_err(Error::msg)?;
            }
        }
        Ok(())
    }
}

impl Picture for Vec<RGBA8> {
    fn create(width: u32, height: u32) -> Result<Self> {
        Ok(std::iter::repeat(RGBA8::default())
            .take((width * height) as usize)
            .collect())
    }

    fn load_data(&mut self, pixels: &[RGBA8]) -> Result<()> {
        self.copy_from_slice(pixels);
        Ok(())
    }
}
