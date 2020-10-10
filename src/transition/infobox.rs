use super::geometry::Geometry;
use super::messagebox::messagebox;
use super::texture::TextureCreationParams;
use super::tilecache::TileCache;
use transdl::event::Event;
use transdl::video::Surface;

fn show(
    screen: &mut Surface,
    tilecache: &TileCache,
    texture_creation_params: TextureCreationParams,
    text: &str,
) {
    let messagebox = messagebox(text, tilecache, texture_creation_params);
    let destrect = Geometry {
        x: (screen.width() as usize - messagebox.width() as usize) as i16
            / 2,
        y: (screen.height() as usize - messagebox.height() as usize)
            as i16
            / 2,
        w: messagebox.width(),
        h: messagebox.height(),
    };

    // backup the background
    let mut background_backup = Surface::create_rgb(
        texture_creation_params.flags,
        destrect.w,
        destrect.h,
        texture_creation_params.bits_per_pixel,
        0,
        0,
        0,
        0,
    );
    screen.blit(
        Some(destrect.as_sdl_rect()),
        &mut background_backup,
        None,
    );
    messagebox.blit_to_sdl_surface(None, screen, Some(destrect.clone()));
    screen.update_rect(0, 0, 0, 0);

    loop {
        match Event::wait() {
            Ok(Event::KeyDown { .. })
            | Ok(Event::MouseButtonDown { .. }) => {
                background_backup.blit(
                    None,
                    screen,
                    Some(destrect.as_sdl_rect()),
                );
                screen.update_rect(0, 0, 0, 0);
                return;
            }
            Ok(Event::VideoExpose) => {
                screen.update_rect(0, 0, 0, 0);
            }
            Ok(_) => {
                // Ignore other events
            }
            Err(e) => {
                panic!("Error getting SDL event: {:?}", e);
            }
        }
    }
}

#[derive(Default)]
pub struct InfoMessageQueue {
    pub messages: Vec<String>,
}

impl InfoMessageQueue {
    pub fn process(
        &mut self,
        screen: &mut Surface,
        tilecache: &TileCache,
        params: TextureCreationParams,
    ) {
        for message in self.messages.drain(..) {
            show(screen, tilecache, params, &message);
        }
    }
}

pub mod ffi {
    use super::super::texture::ffi::FnTextureCreationParams;
    use super::super::tilecache::ffi::FnTileCache;
    use libc::c_char;
    use std::ffi::CStr;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    pub type FnInfoMessageQueue = super::InfoMessageQueue;

    #[no_mangle]
    pub extern "C" fn fn_infobox_show(
        screen: *mut SDL_Surface,
        tilecache: *const FnTileCache,
        params: FnTextureCreationParams,
        message: *const c_char,
    ) {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());
        assert!(!message.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };

        let message = match unsafe { CStr::from_ptr(message) }.to_str() {
            Ok(message) => message,
            Err(e) => panic!("Error reading infobox message: {:?}", e),
        };

        super::show(&mut screen, tilecache, params, message);
    }

    #[no_mangle]
    pub extern "C" fn fn_info_message_queue_create(
    ) -> *mut FnInfoMessageQueue {
        Box::into_raw(Box::new(FnInfoMessageQueue::default()))
    }

    #[no_mangle]
    pub extern "C" fn fn_info_message_queue_free(
        ptr: *mut FnInfoMessageQueue,
    ) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_info_message_queue_push(
        ptr: *mut FnInfoMessageQueue,
        message: *const c_char,
    ) {
        assert!(!ptr.is_null());
        let queue = unsafe { &mut (*ptr) };

        let message = {
            match unsafe { CStr::from_ptr(message) }.to_str() {
                Ok(message) => message,
                Err(e) => {
                    eprintln!("Couldn't read info message: {:?}.", e);
                    return;
                }
            }
        };

        queue.messages.push(message.to_string())
    }

    #[no_mangle]
    pub extern "C" fn fn_info_message_queue_process(
        ptr: *mut FnInfoMessageQueue,
        screen: *mut SDL_Surface,
        tilecache: *const FnTileCache,
        params: FnTextureCreationParams,
    ) {
        assert!(!ptr.is_null());
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());

        let queue = unsafe { &mut (*ptr) };
        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };

        queue.process(&mut screen, tilecache, params);
    }
}
