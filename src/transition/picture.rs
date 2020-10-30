use super::geometry::Geometry;
use super::messagebox::messagebox;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::{
    Result, PICTURE_HEIGHT, PICTURE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use anyhow::anyhow;
use std::fs::File;
use std::io::Read;
use transdl::event::{Event, KeyCode, MouseButton};
use transdl::video::Surface;

pub fn load(
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

pub fn show_splash_with_message(
    tilecache: &TileCache,
    texture_creation_params: TextureCreationParams,
    target: &mut Surface,
    file: &mut File,
    message: Option<&str>,
    x: i16,
    y: i16,
) -> Result<()> {
    let picture = load(file, texture_creation_params)?;
    picture.blit_to_sdl_surface(None, target, None);

    if let Some(message) = message {
        let messagebox =
            messagebox(message, tilecache, texture_creation_params);
        let destrect = Geometry {
            x,
            y,
            w: messagebox.width(),
            h: messagebox.height(),
        };
        messagebox.blit_to_sdl_surface(None, target, Some(destrect));
    }
    target.update_rect(0, 0, 0, 0);

    loop {
        match Event::wait().map_err(|e| anyhow!("{}", e))? {
            Event::Quit
            | Event::KeyDown {
                key: Some(KeyCode::Escape),
                ..
            }
            | Event::KeyDown {
                key: Some(KeyCode::Return),
                ..
            }
            | Event::MouseButtonDown {
                button: Some(MouseButton::Left),
                ..
            } => return Ok(()),
            Event::VideoExpose => {
                target.update_rect(0, 0, 0, 0);
            }
            _ => {}
        }
    }
}

mod ffi {
    use super::super::file::ffi::FnFile;
    use super::super::texture::ffi::FnTexture;
    use super::super::texture::ffi::FnTextureCreationParams;
    use super::super::tilecache::ffi::FnTileCache;
    use libc::c_char;
    use std::ffi::CStr;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_picture_load(
        file: *mut FnFile,
        params: FnTextureCreationParams,
    ) -> *mut FnTexture {
        let file = unsafe { &mut (*file) };
        match super::load(file.as_ref_mut(), params) {
            Ok(t) => Box::into_raw(Box::new(t)),
            Err(e) => {
                eprintln!("Error loading picture: {:?}", e);
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_picture_splash_show_with_message(
        tilecache: *const FnTileCache,
        texture_creation_params: FnTextureCreationParams,
        target: *mut SDL_Surface,
        file: *mut FnFile,
        message: *const c_char,
        x: i16,
        y: i16,
    ) -> bool {
        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        assert!(!file.is_null());
        let file = unsafe { &mut (*file) };

        assert!(!target.is_null());
        let mut target = Surface { raw: target };

        let message = {
            if message.is_null() {
                None
            } else {
                match unsafe { CStr::from_ptr(message) }.to_str() {
                    Ok(message) => Some(message),
                    Err(e) => {
                        eprintln!("Couldn't read message: {:?}.", e);
                        return false;
                    }
                }
            }
        };

        match super::show_splash_with_message(
            tilecache,
            texture_creation_params,
            &mut target,
            file.as_ref_mut(),
            message,
            x,
            y,
        ) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("Error showing splash picture: {:?}", e);
                false
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_picture_splash_show(
        tilecache: *const FnTileCache,
        texture_creation_params: FnTextureCreationParams,
        target: *mut SDL_Surface,
        file: *mut FnFile,
    ) -> bool {
        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        assert!(!file.is_null());
        let file = unsafe { &mut (*file) };

        assert!(!target.is_null());
        let mut target = Surface { raw: target };

        match super::show_splash_with_message(
            tilecache,
            texture_creation_params,
            &mut target,
            file.as_ref_mut(),
            None,
            0,
            0,
        ) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("Error showing splash picture: {:?}", e);
                false
            }
        }
    }
}
