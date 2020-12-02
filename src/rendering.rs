use crate::geometry::Geometry;
use crate::tilecache::TileCache;
use transdl::video::{PixelFormat, Surface};

pub type TileIndex = usize;

pub trait Color {
    fn transdl_map_rgb(&self, format: &PixelFormat) -> u32;
}

impl Color for (u8, u8, u8) {
    fn transdl_map_rgb(&self, format: &PixelFormat) -> u32 {
        let &(r, g, b) = self;
        transdl::video::map_rgb(format, r, g, b)
    }
}

pub struct Transparent;

impl Color for Transparent {
    fn transdl_map_rgb(&self, format: &PixelFormat) -> u32 {
        crate::sdl_surface_transparent(format)
    }
}

pub trait Renderer {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry);
    fn fill_rect(&mut self, rect: Geometry, color: &dyn Color);
    fn fill(&mut self, color: &dyn Color);

    fn draw_rect(&mut self, rect: Geometry, color: &dyn Color) {
        {
            let mut r = rect.clone();
            r.w = 1;
            self.fill_rect(rect, color);
        }
        {
            let mut r = rect.clone();
            r.x += r.w as i16 - 1;
            r.w = 1;
            self.fill_rect(rect, color);
        }
        {
            let mut r = rect.clone();
            r.h = 1;
            self.fill_rect(rect, color);
        }
        {
            let mut r = rect.clone();
            r.y += r.h as i16 - 1;
            r.h = 1;
            self.fill_rect(rect, color);
        }
    }
}

pub struct MovePositionRenderer<'a> {
    pub offset_x: i32,
    pub offset_y: i32,
    pub upstream: &'a mut dyn Renderer,
}

impl<'a> Renderer for MovePositionRenderer<'a> {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry) {
        let new_destination = Geometry {
            x: destination.x + self.offset_x as i16,
            y: destination.y + self.offset_y as i16,
            w: destination.w,
            h: destination.h,
        };
        self.upstream.place_tile(tile, new_destination);
    }

    fn fill_rect(&mut self, mut rect: Geometry, color: &dyn Color) {
        rect.x += self.offset_x as i16;
        rect.y += self.offset_y as i16;
        self.upstream.fill_rect(rect, color);
    }

    fn fill(&mut self, color: &dyn Color) {
        self.upstream.fill(color);
    }
}

pub struct SurfaceRenderer<'a> {
    pub target: &'a mut Surface,
    pub tilecache: &'a TileCache,
}

impl<'a> Renderer for SurfaceRenderer<'a> {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry) {
        self.tilecache.get_tile(tile).unwrap().blit(
            None,
            self.target,
            Some(destination.as_sdl_rect()),
        );
    }

    fn fill_rect(&mut self, rect: Geometry, color: &dyn Color) {
        self.target.fill_rect(
            rect.as_sdl_rect(),
            color.transdl_map_rgb(&self.target.format()),
        );
    }

    fn fill(&mut self, color: &dyn Color) {
        self.target
            .fill(color.transdl_map_rgb(&self.target.format()));
    }
}
