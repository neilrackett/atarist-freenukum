use crate::Result;
use anyhow::anyhow;
use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
    ttf::{Font, FontStyle, Sdl2TtfContext},
};

pub fn load_default_font(ttf_context: &Sdl2TtfContext) -> Result<Font> {
    // TODO: replace this by some determination method
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
    let mut font = ttf_context
        .load_font(font_path, 10)
        .map_err(|s| anyhow!(s))?;
    font.set_style(FontStyle::BOLD);
    Ok(font)
}

pub trait SurfaceExt {
    fn set_data(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<()>;
}

impl<RT: RenderTarget> SurfaceExt for Canvas<RT> {
    fn set_data(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<()> {
        let mut iter = data.iter();

        for i in 0..height {
            for j in 0..width {
                let red = *iter.next().unwrap();
                let green = *iter.next().unwrap();
                let blue = *iter.next().unwrap();
                let opaque = *iter.next().unwrap();

                let color = if opaque == 0 {
                    Color::RGBA(0, 0, 0, 0)
                } else {
                    Color::RGB(red, green, blue)
                };

                self.set_draw_color(color);
                self.draw_point(Point::new(j as i32, i as i32))
                    .map_err(|s| anyhow!(s))?;
            }
        }
        Ok(())
    }
}
