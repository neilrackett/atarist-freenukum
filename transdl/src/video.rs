use crate::ll;

pub use ll::SDL_Color as Color;
pub use ll::SDL_Rect as Rect;

#[repr(transparent)]
#[derive(Debug)]
pub struct Surface {
    pub raw: *mut ll::SDL_Surface,
}

#[repr(transparent)]
pub struct PixelFormat {
    pub raw: *mut ll::SDL_PixelFormat,
}

impl PixelFormat {
    pub fn bits_per_pixel(&self) -> u8 {
        unsafe { (*self.raw).BitsPerPixel }
    }
}

impl Surface {
    pub fn set_video_mode(w: i32, h: i32, depth: i32, flags: u32) -> Self {
        Surface {
            raw: unsafe { ll::SDL_SetVideoMode(w, h, depth, flags) },
        }
    }

    pub fn create_rgb(
        flags: u32,
        width: u16,
        height: u16,
        depth: i32,
        rmask: u32,
        gmask: u32,
        bmask: u32,
        amask: u32,
    ) -> Self {
        Surface {
            raw: unsafe {
                ll::SDL_CreateRGBSurface(
                    flags,
                    width as i32,
                    height as i32,
                    depth,
                    rmask,
                    gmask,
                    bmask,
                    amask,
                )
            },
        }
    }

    pub fn set_color_key(&mut self, flag: u32, key: u32) {
        unsafe {
            ll::SDL_SetColorKey(self.raw, flag, key);
        }
    }

    pub fn format(&self) -> PixelFormat {
        let raw = unsafe { *self.raw }.format;
        PixelFormat { raw }
    }

    pub fn flags(&self) -> u32 {
        unsafe { *self.raw }.flags
    }

    pub fn fill_rect(&mut self, dstrect: Rect, color: u32) {
        unsafe {
            let dstrect_ptr: *const Rect = &dstrect;
            let dstrect_mut_ptr = dstrect_ptr as *mut Rect;
            ll::SDL_FillRect(self.raw, dstrect_mut_ptr, color);
        }
    }

    pub fn fill(&mut self, color: u32) {
        unsafe {
            ll::SDL_FillRect(self.raw, std::ptr::null_mut(), color);
        }
    }

    pub fn update_rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
        unsafe {
            ll::SDL_UpdateRect(self.raw, x, y, w, h);
        }
    }

    pub fn update(&mut self) {
        unsafe {
            ll::SDL_UpdateRect(self.raw, 0, 0, 0, 0);
        }
    }

    pub fn blit(
        &self,
        srcrect: Option<Rect>,
        dst: &mut Surface,
        dstrect: Option<Rect>,
    ) {
        let srcrect: *mut Rect = if let Some(ref r) = srcrect {
            r as *const Rect as *mut Rect
        } else {
            std::ptr::null_mut()
        };
        let dstrect: *mut Rect = if let Some(ref r) = dstrect {
            r as *const Rect as *mut Rect
        } else {
            std::ptr::null_mut()
        };
        let dst_raw = dst.raw as *mut ll::SDL_Surface;
        unsafe {
            ll::SDL_UpperBlit(self.raw, srcrect, dst_raw, dstrect);
        }
    }

    pub fn width(&self) -> usize {
        unsafe { *self.raw }.w as usize
    }

    pub fn height(&self) -> usize {
        unsafe { *self.raw }.h as usize
    }

    pub fn toggle_fullscreen(&mut self) -> bool {
        unsafe { ll::SDL_WM_ToggleFullScreen(self.raw) != 0 }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        // TODO: re-enable once we have migrated everything to rust
        // unsafe { ll::SDL_FreeSurface(self.raw) }
    }
}

pub fn map_rgb(format: &PixelFormat, r: u8, g: u8, b: u8) -> u32 {
    unsafe { ll::SDL_MapRGB(format.raw, r, g, b) }
}
