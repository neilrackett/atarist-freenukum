use crate::Result;
use anyhow::anyhow;
use rgb::RGBA8;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Point,
    render::{Canvas, RenderTarget},
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

pub trait Picture: Sized {
    fn from_data(width: u32, height: u32, color: &[RGBA8])
        -> Result<Self>;
}

impl<'t> Picture for Surface<'t> {
    fn from_data(
        width: u32,
        height: u32,
        color: &[RGBA8],
    ) -> Result<Self> {
        let surface =
            Surface::new(width, height, PixelFormatEnum::RGBA8888)
                .map_err(|s| anyhow!(s))?;
        let mut canvas =
            Canvas::from_surface(surface).map_err(|s| anyhow!(s))?;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        use rgb::ComponentBytes;
        canvas.set_data(color.as_bytes(), width, height)?;
        Ok(canvas.into_surface())
    }
}
