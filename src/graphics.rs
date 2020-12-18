use crate::Result;
use anyhow::anyhow;
use rgb::RGBA8;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    surface::Surface,
    ttf::{Font, FontStyle, Sdl2TtfContext},
};

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
    let font_path = std::path::Path::new(&std::env::var("WINDIR")?)
        .join("Fonts")
        .join("Arial.ttf");

    #[cfg(target_os = "linux")]
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";

    let mut font = ttf_context
        .load_font(font_path, 10)
        .map_err(|s| anyhow!(s))?;
    font.set_style(FontStyle::BOLD);
    Ok(font)
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
                .map_err(|s| anyhow!(s))?;
        surface
            .fill_rect(None, RGBA8::default().as_sdl_color())
            .map_err(|s| anyhow!(s))?;
        Ok(surface)
    }

    fn load_data(&mut self, pixels: &[RGBA8]) -> Result<()> {
        for i in 0..self.height() {
            for j in 0..self.width() {
                let color =
                    pixels[(i * self.width() + j) as usize].as_sdl_color();
                let rect = Rect::new(j as i32, i as i32, 1, 1);
                self.fill_rect(rect, color).map_err(|s| anyhow!(s))?;
            }
        }
        Ok(())
    }
}
