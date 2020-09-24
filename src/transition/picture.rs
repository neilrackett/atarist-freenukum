use super::texture::{Texture, TextureCreationParams};
use crate::{
    Result, PICTURE_HEIGHT, PICTURE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use std::fs::File;
use std::io::Read;

pub fn picture_load(
    input: &mut File,
    params: TextureCreationParams,
) -> Result<Texture> {
    let mut picture = Texture::create_with_params(
        WINDOW_WIDTH as u16,
        WINDOW_HEIGHT as u16,
        params,
    );
    const NUM_LOADS: usize = PICTURE_WIDTH * PICTURE_HEIGHT;
    let mut buffer = [0u8; NUM_LOADS];

    let mut data = [0u8; PICTURE_WIDTH * PICTURE_HEIGHT * 4 * 8];
    let mut index;

    // read blue
    index = 2;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let blue_pixel: u8 = (buffer[i] >> (7 - j)) & 1;
            data[index] += blue_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read green
    index = 1;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let green_pixel: u8 = (buffer[i] >> (7 - j)) & 1;
            data[index] += green_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read red
    index = 0;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let red_pixel: u8 = (buffer[i] >> (7 - j)) & 1;
            data[index] += red_pixel * 0x54 * 2;
            index += 4;
        }
    }

    // read brighten, and set pixels opaque
    index = 0;
    input.read_exact(&mut buffer)?;
    for i in 0..NUM_LOADS {
        for j in 0..8 {
            let bright_pixel: u8 = (buffer[i] >> (7 - j)) & 1;

            // brighten red
            data[index] += bright_pixel * 0x54;
            index += 1;

            // brighten blue
            data[index] += bright_pixel * 0x54;
            index += 1;

            // brighten green
            data[index] += bright_pixel * 0x54;
            index += 1;

            // set opaque
            data[index] = 0xff;
            index += 1;
        }
    }

    picture.set_data(&data, params.transparent);

    Ok(picture)
}

mod ffi {
    use super::super::file::ffi::FnFile;
    use super::super::texture::ffi::FnTexture;
    use super::super::texture::ffi::FnTextureCreationParams;

    #[no_mangle]
    pub extern "C" fn fn_picture_load(
        file: *mut FnFile,
        params: FnTextureCreationParams,
    ) -> *mut FnTexture {
        let file = unsafe { &mut (*file) };
        match super::picture_load(file.as_ref_mut(), params) {
            Ok(t) => Box::into_raw(Box::new(t)),
            Err(e) => {
                eprintln!("Error loading picture: {:?}", e);
                std::ptr::null_mut()
            }
        }
    }
}
