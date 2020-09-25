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
            p(true, "BACK0.DN1", 48),
            p(false, "BACK1.DN1", 48),
            p(false, "BACK2.DN1", 48),
            p(false, "BACK3.DN1", 48),
            p(true, "SOLID0.DN1", 48),
            p(false, "SOLID1.DN1", 48),
            p(false, "SOLID2.DN1", 48),
            p(false, "SOLID3.DN1", 48),
            p(true, "ANIM0.DN1", 48),
            p(true, "ANIM1.DN1", 48),
            p(true, "ANIM2.DN1", 48),
            p(true, "ANIM3.DN1", 48),
            p(true, "ANIM4.DN1", 48),
            p(true, "ANIM5.DN1", 48),
            p(true, "OBJECT0.DN1", 50),
            p(true, "OBJECT1.DN1", 50),
            p(true, "OBJECT2.DN1", 50),
            p(true, "MAN0.DN1", 48),
            p(true, "MAN1.DN1", 48),
            p(true, "MAN2.DN1", 48),
            p(true, "MAN3.DN1", 48),
            p(true, "MAN4.DN1", 48),
            p(true, "FONT1.DN1", 50),
            p(true, "FONT2.DN1", 50),
            p(true, "BORDER.DN1", 48),
            p(true, "NUMBERS.DN1", 44),
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
    use libc::c_char;
    use std::ffi::CStr;
    use std::path::Path;

    #[no_mangle]
    pub extern "C" fn fn_tilecache_load(
        path: *const c_char,
        params: FnTextureCreationParams,
    ) -> *mut FnTileCache {
        assert!(!path.is_null());
        let path = {
            match unsafe { CStr::from_ptr(path) }.to_str() {
                Ok(filename) => filename,
                Err(e) => {
                    eprintln!("Couldn't read file name: {:?}.", e);
                    return std::ptr::null_mut();
                }
            }
        };

        match FnTileCache::load_from_path(Path::new(path), params) {
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
