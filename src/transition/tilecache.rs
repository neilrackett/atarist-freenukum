use super::texture::{Texture, TextureCreationParams};
use super::tile::{self, TileHeader};
use crate::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct TileCache {
    tiles: Vec<Texture>,
}

impl TileCache {
    pub fn load_from_path(
        path: &Path,
        params: TextureCreationParams,
    ) -> Result<Self> {
        struct Properties {
            transparent: bool,
            name: &'static str,
            max_tiles: usize,
        };
        fn p(
            transparent: bool,
            name: &'static str,
            max_tiles: usize,
        ) -> Properties {
            Properties {
                transparent,
                name,
                max_tiles,
            }
        }
        let files = vec![
            p(true, "back0.dn1", 48),
            p(false, "back1.dn1", 48),
            p(false, "back2.dn1", 48),
            p(false, "back3.dn1", 48),
            p(true, "solid0.dn1", 48),
            p(false, "solid1.dn1", 48),
            p(false, "solid2.dn1", 48),
            p(false, "solid3.dn1", 48),
            p(true, "anim0.dn1", 48),
            p(true, "anim1.dn1", 48),
            p(true, "anim2.dn1", 48),
            p(true, "anim3.dn1", 48),
            p(true, "anim4.dn1", 48),
            p(true, "anim5.dn1", 48),
            p(true, "object0.dn1", 50),
            p(true, "object1.dn1", 50),
            p(true, "object2.dn1", 50),
            p(true, "man0.dn1", 48),
            p(true, "man1.dn1", 48),
            p(true, "man2.dn1", 48),
            p(true, "man3.dn1", 48),
            p(true, "man4.dn1", 48),
            p(true, "font1.dn1", 50),
            p(true, "font2.dn1", 50),
            p(true, "border.dn1", 48),
            p(true, "numbers.dn1", 44),
        ];

        let mut tiles = Vec::new();

        for Properties {
            transparent,
            name,
            max_tiles,
        } in files.into_iter()
        {
            let path = path.join(name);
            let mut file = File::open(path)?;
            let header = TileHeader::load_from(&mut file)?;
            let max_tiles =
                std::cmp::min(max_tiles, header.tiles as usize);
            tiles.append(&mut Self::load_file(
                &mut file,
                params,
                header,
                max_tiles,
                transparent,
            )?);
        }

        Ok(TileCache { tiles })
    }

    fn load_file<R: Read>(
        r: &mut R,
        params: TextureCreationParams,
        header: TileHeader,
        max_tiles: usize,
        has_transparency: bool,
    ) -> Result<Vec<Texture>> {
        let mut tiles = Vec::new();
        for _ in 0..max_tiles {
            tiles.push(tile::load(
                r,
                params,
                header.clone(),
                has_transparency,
            )?);
        }
        Ok(tiles)
    }

    pub fn get_tile(&self, index: usize) -> Option<&Texture> {
        self.tiles.get(index)
    }
}

pub mod ffi {
    pub type FnTileCache = super::TileCache;
    use super::super::texture::ffi::{FnTexture, FnTextureCreationParams};

    #[no_mangle]
    pub extern "C" fn fn_tilecache_load(
        params: FnTextureCreationParams,
    ) -> *mut FnTileCache {
        let path = super::super::data::path();
        match FnTileCache::load_from_path(&path, params) {
            Ok(tc) => Box::into_raw(Box::new(tc)),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_tilecache_free(ptr: *mut FnTileCache) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_tilecache_get_tile(
        ptr: *const FnTileCache,
        index: usize,
    ) -> *const FnTexture {
        assert!(!ptr.is_null());

        let tilecache: &FnTileCache = unsafe { &(*ptr) };

        match tilecache.get_tile(index) {
            Some(tile) => tile as *const FnTexture,
            None => {
                eprintln!("Tile with index {} doesn't exist", index);
                return std::ptr::null_mut();
            }
        }
    }
}
