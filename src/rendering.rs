use crate::geometry::Geometry;
use crate::tilecache::TileCache;
use transdl::video::Surface;

pub type TileIndex = usize;

pub trait Renderer {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry);
    fn fill_rect(&mut self, rect: Geometry, r: u8, g: u8, b: u8);
    fn fill(&mut self, r: u8, g: u8, b: u8);
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

    fn fill_rect(&mut self, mut rect: Geometry, r: u8, g: u8, b: u8) {
        rect.x += self.offset_x as i16;
        rect.y += self.offset_y as i16;
        self.upstream.fill_rect(rect, r, g, b);
    }

    fn fill(&mut self, r: u8, g: u8, b: u8) {
        self.upstream.fill(r, g, b);
    }
}

pub struct SurfaceRenderer<'a> {
    pub target: &'a mut Surface,
    pub tilecache: &'a TileCache,
}

impl<'a> Renderer for SurfaceRenderer<'a> {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry) {
        self.tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            self.target,
            Some(destination),
        );
    }

    fn fill_rect(&mut self, rect: Geometry, r: u8, g: u8, b: u8) {
        self.target.fill_rect(
            rect.as_sdl_rect(),
            transdl::video::map_rgb(&self.target.format(), r, g, b),
        );
    }

    fn fill(&mut self, r: u8, g: u8, b: u8) {
        self.target.fill(transdl::video::map_rgb(
            &self.target.format(),
            r,
            g,
            b,
        ));
    }
}
