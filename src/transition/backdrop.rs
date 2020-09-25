use super::geometry::Geometry;
use super::texture::{Texture, TextureCreationParams};
use super::tile::{self, TileHeader};
use crate::{
    Result, BACKDROP_HEIGHT, BACKDROP_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use std::io::Read;

pub fn load<R: Read>(
    r: &mut R,
    params: TextureCreationParams,
) -> Result<Texture> {
    let mut backdrop = Texture::create_with_params(
        BACKDROP_WIDTH as u16 * TILE_WIDTH as u16,
        BACKDROP_HEIGHT as u16 * TILE_HEIGHT as u16,
        params,
    );

    let mut geometry = Geometry {
        x: 0,
        y: 0,
        w: TILE_WIDTH as u16,
        h: TILE_HEIGHT as u16,
    };

    let header = TileHeader {
        width: 2,
        height: 16,
        tiles: 0,
    };

    for _ in 0..BACKDROP_WIDTH * BACKDROP_HEIGHT {
        let tile = tile::load(r, params, header.clone(), false)?;
        tile.clone_to_texture(None, &mut backdrop, Some(geometry.clone()));

        geometry.x += 16;
        if geometry.x == 16 * BACKDROP_WIDTH as i16 {
            geometry.x = 0;
            geometry.y += 16;
        }
    }

    Ok(backdrop)
}

pub mod ffi {
    pub type FnTexture = super::Texture;
    pub type FnTextureCreationParams = super::TextureCreationParams;
    pub use super::super::file::ffi::FnFile;

    #[no_mangle]
    pub extern "C" fn fn_backdrop_load(
        ptr: *mut FnFile,
        params: FnTextureCreationParams,
    ) -> *mut FnTexture {
        assert!(!ptr.is_null());
        let file = unsafe { &mut (*ptr) };
        match super::load(file.as_ref_mut(), params) {
            Ok(t) => Box::into_raw(Box::new(t)),
            Err(e) => {
                eprintln!("Error loading picture: {:?}", e);
                std::ptr::null_mut()
            }
        }
    }
}
