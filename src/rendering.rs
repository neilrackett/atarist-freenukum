use crate::geometry::Geometry;

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
