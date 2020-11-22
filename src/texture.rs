use crate::rendering::{Color, Renderer, TileIndex};
use crate::tilecache::TileCache;
use crate::Geometry;
use transdl::video::{map_rgb, Rect, Surface};

pub struct TextureRenderer<'a> {
    pub target: &'a mut Texture,
    pub tilecache: &'a TileCache,
}

impl<'a> Renderer for TextureRenderer<'a> {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry) {
        self.tilecache.get_tile(tile).unwrap().clone_to_texture(
            None,
            self.target,
            Some(destination),
        );
    }

    fn fill_rect(&mut self, rect: Geometry, color: &dyn Color) {
        self.target.fill_area(Some(rect), color);
    }

    fn fill(&mut self, color: &dyn Color) {
        self.target.fill_area(None, color);
    }
}

pub trait CloneToTexture {
    fn clone_to_texture(
        &self,
        srcrect: Option<Geometry>,
        dst: &mut Texture,
        dstrect: Option<Geometry>,
    );
}

impl CloneToTexture for Surface {
    fn clone_to_texture(
        &self,
        srcrect: Option<Geometry>,
        dst: &mut Texture,
        dstrect: Option<Geometry>,
    ) {
        self.blit(
            srcrect.map(|r| r.as_sdl_rect()),
            &mut dst.surface,
            dstrect.map(|r| r.as_sdl_rect()),
        );
    }
}

#[derive(Debug)]
pub struct Texture {
    w: u16,
    h: u16,
    surface: Surface,
}

#[derive(Clone, Copy, Debug)]
pub struct TextureCreationParams {
    pub flags: u32,
    pub bits_per_pixel: u8,
    pub transparent: u32,
}

impl TextureCreationParams {
    pub fn create_surface(&self, w: u16, h: u16) -> Surface {
        let mut surface = Surface::create_rgb(
            self.flags,
            w,
            h,
            self.bits_per_pixel as i32,
            0,
            0,
            0,
            0,
        );
        surface.set_color_key(
            transdl::ll::SDL_SRCCOLORKEY,
            super::sdl_surface_transparent(&surface),
        );
        surface
    }
}

impl Texture {
    pub fn create_with_params(
        w: u16,
        h: u16,
        params: TextureCreationParams,
    ) -> Texture {
        Texture {
            w,
            h,
            surface: params.create_surface(w, h),
        }
    }

    pub fn set_data(&mut self, data: &[u8], transparent: u32) {
        let mut iter = data.into_iter();
        let mut r = Rect {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
        };
        let format = self.surface.format();

        for i in 0..self.h {
            for j in 0..self.w {
                let red = *iter.next().unwrap();
                let green = *iter.next().unwrap();
                let blue = *iter.next().unwrap();
                let opaque = *iter.next().unwrap();

                let color = if opaque == 0 {
                    transparent
                } else {
                    map_rgb(&format, red, green, blue)
                };

                r.x = j as i16;
                r.y = i as i16;

                self.surface.fill_rect(r, color);
            }
        }
    }

    pub fn blit_to_sdl_surface(
        &self,
        srcrect: Option<Geometry>,
        destination: &mut Surface,
        dstrect: Option<Geometry>,
    ) {
        let srcrect_sdl = srcrect
            .unwrap_or_else(|| Geometry {
                x: 0,
                y: 0,
                w: self.w,
                h: self.h,
            })
            .as_sdl_rect();
        let dstrect_sdl = dstrect.map(|g| g.as_sdl_rect());
        self.surface
            .blit(Some(srcrect_sdl), destination, dstrect_sdl);
    }

    pub fn clone_to_texture(
        &self,
        srcrect: Option<Geometry>,
        dst: &mut Texture,
        dstrect: Option<Geometry>,
    ) {
        self.blit_to_sdl_surface(srcrect, &mut dst.surface, dstrect);
    }

    pub fn width(&self) -> u16 {
        self.w
    }

    pub fn height(&self) -> u16 {
        self.h
    }

    pub fn fill_area(
        &mut self,
        area: Option<Geometry>,
        color: &dyn Color,
    ) {
        let r = match area {
            Some(g) => g.as_sdl_rect(),
            None => Rect {
                x: 0,
                y: 0,
                w: self.w,
                h: self.h,
            },
        };

        let format = self.surface.format();
        let color = color.transdl_map_rgb(&format);

        self.surface.fill_rect(r, color);
    }
}
