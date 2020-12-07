use sdl2::surface::Surface;

pub type TileIndex = usize;

pub trait TileProvider {
    fn get_tile(&self, index: TileIndex) -> Option<&Surface>;
}
