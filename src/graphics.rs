use transdl::video::{map_rgb, Rect, Surface};

pub trait SurfaceCreatorProvider {
    type Creator: SurfaceCreator;
    fn surface_creator(&self) -> Self::Creator;
}

impl SurfaceCreatorProvider for Surface {
    type Creator = SurfaceParams;

    fn surface_creator(&self) -> Self::Creator {
        SurfaceParams {
            flags: self.flags(),
            bits_per_pixel: self.format().bits_per_pixel(),
            transparent: crate::sdl_surface_transparent(&self.format()),
        }
    }
}

pub trait SurfaceCreator {
    fn create(&self, w: u32, h: u32) -> Surface;
}

pub struct SurfaceParams {
    pub flags: u32,
    pub bits_per_pixel: u8,
    pub transparent: u32,
}

impl SurfaceCreator for SurfaceParams {
    fn create(&self, w: u32, h: u32) -> Surface {
        let mut surface = Surface::create_rgb(
            self.flags,
            w as u16,
            h as u16,
            self.bits_per_pixel as i32,
            0,
            0,
            0,
            0,
        );
        surface.set_color_key(
            transdl::ll::SDL_SRCCOLORKEY,
            crate::sdl_surface_transparent(&surface.format()),
        );
        surface
    }
}

pub trait SurfaceExt {
    fn set_data(&mut self, data: &[u8]);
}

impl SurfaceExt for Surface {
    fn set_data(&mut self, data: &[u8]) {
        let mut iter = data.into_iter();
        let mut r = Rect {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
        };
        let format = self.format();

        for i in 0..self.height() {
            for j in 0..self.width() {
                let red = *iter.next().unwrap();
                let green = *iter.next().unwrap();
                let blue = *iter.next().unwrap();
                let opaque = *iter.next().unwrap();

                let color = if opaque == 0 {
                    crate::sdl_surface_transparent(&self.format())
                } else {
                    map_rgb(&format, red, green, blue)
                };

                r.x = j as i16;
                r.y = i as i16;

                self.fill_rect(r, color);
            }
        }
    }
}
