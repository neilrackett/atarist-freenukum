use crate::Result;
use std::io::Read;

#[derive(Clone)]
#[repr(C)]
pub struct TileHeader {
    pub tiles: u8,
    pub width: u8,
    pub height: u8,
}

impl TileHeader {
    pub fn load_from<R: Read>(r: &mut R) -> Result<Self> {
        let mut buf = [0u8; 3];
        r.read_exact(&mut buf)?;
        Ok(TileHeader {
            tiles: buf[0],
            width: buf[1],
            height: buf[2],
        })
    }
}

mod ffi {
    type FnTileHeader = super::TileHeader;
    use super::super::file::ffi::FnFile;

    #[no_mangle]
    pub extern "C" fn fn_tileheader_load(
        file: *mut FnFile,
    ) -> FnTileHeader {
        let file = unsafe { &mut (*file) };
        FnTileHeader::load_from(file.as_ref_mut()).unwrap()
    }
}
