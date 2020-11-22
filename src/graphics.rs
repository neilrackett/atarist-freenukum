use transdl::video::Surface;

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
