use crate::Result;
use anyhow::anyhow;
use rgb::RGBA8;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    surface::Surface,
    ttf::{Font, FontStyle, Sdl2TtfContext},
};

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
    fn from_data(width: u32, height: u32, color: &[RGBA8])
        -> Result<Self>;
}

impl<'t> Picture for Surface<'t> {
    fn from_data(
        width: u32,
        height: u32,
        pixels: &[RGBA8],
    ) -> Result<Self> {
        let mut surface =
            Surface::new(width, height, PixelFormatEnum::RGBA8888)
                .map_err(|s| anyhow!(s))?;
        for i in 0..height {
            for j in 0..width {
                let pixel = pixels[(i * width + j) as usize];
                let color =
                    Color::RGBA(pixel.r, pixel.g, pixel.b, pixel.a);
                let rect = Rect::new(j as i32, i as i32, 1, 1);
                surface.fill_rect(rect, color).map_err(|s| anyhow!(s))?;
            }
        }
        Ok(surface)
    }
}
