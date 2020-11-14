use crate::geometry::Geometry;
use crate::tilecache::TileCache;
use transdl::video::Surface;

pub type TileIndex = usize;

pub trait Renderer {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry);
}

pub struct RenderInstruction {
    pub tile: TileIndex,
    pub destination: Geometry,
}

impl Renderer for Vec<RenderInstruction> {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry) {
        self.push(RenderInstruction { tile, destination });
    }
}

pub struct MovePositionRenderer<'a> {
    pub start_x: i32,
    pub start_y: i32,
    pub upstream: &'a mut dyn Renderer,
}

impl<'a> Renderer for MovePositionRenderer<'a> {
    fn place_tile(&mut self, tile: TileIndex, destination: Geometry) {
        let new_destination = Geometry {
            x: destination.x + self.start_x as i16,
            y: destination.y + self.start_y as i16,
            w: destination.w,
            h: destination.h,
        };
        self.upstream.place_tile(tile, new_destination);
    }
}

pub trait RenderToSdlSurface {
    fn render_to_sdl_surface(
        &mut self,
        surface: &mut Surface,
        tilecache: &TileCache,
    );
}

impl RenderToSdlSurface for Vec<RenderInstruction> {
    fn render_to_sdl_surface(
        &mut self,
        target: &mut Surface,
        tilecache: &TileCache,
    ) {
        for instruction in self.drain(..) {
            tilecache
                .get_tile(instruction.tile)
                .unwrap()
                .blit_to_sdl_surface(
                    None,
                    target,
                    Some(instruction.destination),
                );
        }
    }
}
