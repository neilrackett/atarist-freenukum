use super::geometry::Geometry;
use crate::{LEVEL_HEIGHT, LEVEL_WIDTH, TILE_HEIGHT, TILE_WIDTH};

pub struct LevelSolids {
    solids: [[bool; LEVEL_WIDTH]; LEVEL_HEIGHT],
}

impl LevelSolids {
    pub fn new() -> Self {
        LevelSolids {
            solids: [[false; LEVEL_WIDTH]; LEVEL_HEIGHT],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.solids[y][x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < LEVEL_WIDTH);
        assert!(y < LEVEL_HEIGHT);

        self.solids[y][x]
    }

    pub fn collides(&self, rect: Geometry) -> bool {
        let mut solidrect = Geometry {
            x: 0,
            y: 0,
            w: TILE_WIDTH as u16,
            h: TILE_HEIGHT as u16,
        };
        let left_edge = rect.x as usize / TILE_WIDTH;
        let right_edge =
            (rect.x as usize + rect.w as usize) / TILE_WIDTH + 1;
        let top_edge = rect.y as usize / TILE_HEIGHT;
        let bottom_edge =
            (rect.y as usize + rect.h as usize) / TILE_HEIGHT + 1;

        for i in left_edge..right_edge {
            for j in top_edge..bottom_edge {
                if self.get(i, j) {
                    solidrect.x = (i * TILE_WIDTH) as i16;
                    solidrect.y = (j * TILE_HEIGHT) as i16;
                    if rect.overlaps(solidrect) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

pub mod ffi {
    use super::super::geometry::ffi::FnGeometry;

    pub type FnLevelSolids = super::LevelSolids;

    #[no_mangle]
    pub extern "C" fn fn_level_solids_create() -> *mut FnLevelSolids {
        Box::into_raw(Box::new(FnLevelSolids::new()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_solids_free(ptr: *mut FnLevelSolids) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_solids_get(
        ptr: *const FnLevelSolids,
        x: usize,
        y: usize,
    ) -> bool {
        assert!(!ptr.is_null());
        let solids: &FnLevelSolids = unsafe { &(*ptr) };
        solids.get(x, y)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_solids_set(
        ptr: *mut FnLevelSolids,
        x: usize,
        y: usize,
        value: bool,
    ) {
        assert!(!ptr.is_null());
        let solids = unsafe { &mut (*ptr) };
        solids.set(x, y, value);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_solids_collides(
        ptr: *const FnLevelSolids,
        rect: FnGeometry,
    ) -> bool {
        assert!(!ptr.is_null());
        let solids: &FnLevelSolids = unsafe { &(*ptr) };
        solids.collides(rect)
    }
}
