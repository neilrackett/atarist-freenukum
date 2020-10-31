use super::texture::{Texture, TextureCreationParams};
use crate::Result;
use std::io::Read;

const SOLID_START: u16 = 4 * 48;
const SOLID_END: u16 = 8 * 48;

#[derive(Clone, Copy)]
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

pub fn load<R: Read>(
    r: &mut R,
    params: TextureCreationParams,
    header: TileHeader,
    has_transparency: bool,
) -> Result<Texture> {
    let width: u16 = header.width as u16 * 8;
    let height: u16 = header.height as u16;

    let mut tile = Texture::create_with_params(width, height, params);
    let mut data: Vec<u8> =
        Vec::with_capacity(width as usize * height as usize * 4);

    let mut readbuf = [0u8; 5];

    let num_loads = width as usize * height as usize / 8;

    for _ in 0..num_loads {
        r.read_exact(&mut readbuf)?;

        let opaque_row = readbuf[0];
        let blue_row = readbuf[1];
        let green_row = readbuf[2];
        let red_row = readbuf[3];
        let bright_row = readbuf[4];

        for i in 0..8 {
            let bright_pixel = (bright_row >> (7 - i)) & 1;
            let red_pixel = (red_row >> (7 - i)) & 1;
            let green_pixel = (green_row >> (7 - i)) & 1;
            let blue_pixel = (blue_row >> (7 - i)) & 1;
            let opaque_pixel = if has_transparency {
                (opaque_row >> (7 - i)) & 1
            } else {
                1
            };
            let ugly_yellow = if red_pixel == 1
                && green_pixel == 1
                && blue_pixel == 0
                && bright_pixel == 0
            {
                1
            } else {
                0
            };

            data.push(0x54 * (red_pixel * 2 + bright_pixel));
            data.push(
                0x54 * (green_pixel * 2 + bright_pixel - ugly_yellow),
            );
            data.push(0x54 * (blue_pixel * 2 + bright_pixel));
            data.push(opaque_pixel * 0xff);
        }
    }

    tile.set_data(&data, params.transparent);

    Ok(tile)
}

fn is_solid(index: u16) -> bool {
    index >= SOLID_START && index < SOLID_END
}

mod ffi {
    type FnTileHeader = super::TileHeader;
    use super::super::file::ffi::FnFile;
    use super::super::texture::ffi::{FnTexture, FnTextureCreationParams};

    #[no_mangle]
    pub extern "C" fn fn_tileheader_load(
        file: *mut FnFile,
    ) -> FnTileHeader {
        let file = unsafe { &mut (*file) };
        FnTileHeader::load_from(file.as_ref_mut()).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn fn_tile_load(
        file: *mut FnFile,
        params: FnTextureCreationParams,
        header: FnTileHeader,
        has_transparency: bool,
    ) -> *mut FnTexture {
        let file = unsafe { &mut (*file) };
        match super::load(
            file.as_ref_mut(),
            params,
            header,
            has_transparency,
        ) {
            Ok(t) => Box::into_raw(Box::new(t)),
            Err(e) => {
                eprintln!("Error loading tile: {:?}", e);
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_tile_is_solid(index: u16) -> bool {
        super::is_solid(index)
    }
}
