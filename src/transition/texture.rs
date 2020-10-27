use crate::Geometry;
use transdl::video::{map_rgb, Rect, Surface};

#[derive(Debug)]
pub struct Texture {
    w: u16,
    h: u16,
    surface: Surface,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TextureCreationParams {
    pub flags: u32,
    pub bits_per_pixel: u8,
    pub transparent: u32,
}

impl TextureCreationParams {
    pub fn create_surface(&self, w: u16, h: u16) -> Surface {
        let mut surface = Surface::create_rgb(
            self.flags,
            w,
            h,
            self.bits_per_pixel as i32,
            0,
            0,
            0,
            0,
        );
        surface.set_color_key(
            transdl::ll::SDL_SRCCOLORKEY,
            super::sdl_surface_transparent(&surface),
        );
        surface
    }
}

impl Texture {
    pub fn create_with_params(
        w: u16,
        h: u16,
        params: TextureCreationParams,
    ) -> Texture {
        Texture {
            w,
            h,
            surface: params.create_surface(w, h),
        }
    }

    pub fn set_data(&mut self, data: &[u8], transparent: u32) {
        let mut iter = data.into_iter();
        let mut r = Rect {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
        };
        let format = self.surface.format();

        for i in 0..self.h {
            for j in 0..self.w {
                let red = *iter.next().unwrap();
                let green = *iter.next().unwrap();
                let blue = *iter.next().unwrap();
                let opaque = *iter.next().unwrap();

                let color = if opaque == 0 {
                    transparent
                } else {
                    map_rgb(&format, red, green, blue)
                };

                r.x = j as i16;
                r.y = i as i16;

                self.surface.fill_rect(r, color);
            }
        }
    }

    pub fn blit_to_sdl_surface(
        &self,
        srcrect: Option<Geometry>,
        destination: &mut Surface,
        dstrect: Option<Geometry>,
    ) {
        let srcrect_sdl = srcrect
            .unwrap_or_else(|| Geometry {
                x: 0,
                y: 0,
                w: self.w,
                h: self.h,
            })
            .as_sdl_rect();
        let dstrect_sdl = dstrect.map(|g| g.as_sdl_rect());
        self.surface
            .blit(Some(srcrect_sdl), destination, dstrect_sdl);
    }

    pub fn clone_to_texture(
        &self,
        srcrect: Option<Geometry>,
        dst: &mut Texture,
        dstrect: Option<Geometry>,
    ) {
        self.blit_to_sdl_surface(srcrect, &mut dst.surface, dstrect);
    }

    pub fn width(&self) -> u16 {
        self.w
    }

    pub fn height(&self) -> u16 {
        self.h
    }

    pub fn fill_area(
        &mut self,
        area: Option<Geometry>,
        red: u8,
        green: u8,
        blue: u8,
    ) {
        let r = match area {
            Some(g) => g.as_sdl_rect(),
            None => Rect {
                x: 0,
                y: 0,
                w: self.w,
                h: self.h,
            },
        };

        let format = self.surface.format();
        let color = map_rgb(&format, red, green, blue);

        self.surface.fill_rect(r, color);
    }
}

pub mod ffi {
    pub type FnTexture = super::Texture;
    pub type FnTextureCreationParams = super::TextureCreationParams;
    use crate::Geometry;
    use transdl::video::Surface;

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_new_with_params(
        w: u16,
        h: u16,
        params: FnTextureCreationParams,
    ) -> *mut FnTexture {
        Box::into_raw(Box::new(FnTexture::create_with_params(
            w, h, params,
        )))
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_free(ptr: *mut FnTexture) {
        if !ptr.is_null() {
            Box::from_raw(ptr);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_set_data(
        ptr: *mut FnTexture,
        data: *const u8,
        transparent: u32,
    ) {
        let t: &mut FnTexture = &mut (*ptr);
        assert!(!data.is_null());
        let data = std::slice::from_raw_parts(
            data,
            t.w as usize * t.h as usize * 4,
        );
        t.set_data(data, transparent);
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_blit_to_sdl_surface(
        ptr: *const FnTexture,
        srcrect: *const Geometry,
        destination: *mut transdl::ll::SDL_Surface,
        dstrect: *const Geometry,
    ) {
        let t: &FnTexture = &(*ptr);
        let srcrect = srcrect.as_ref().cloned();
        let mut destination = Surface { raw: destination };
        let dstrect = dstrect.as_ref().cloned();
        t.blit_to_sdl_surface(srcrect, &mut destination, dstrect);
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_clone_to_texture(
        ptr: *const FnTexture,
        srcrect: *const Geometry,
        destination: *mut FnTexture,
        dstrect: *const Geometry,
    ) {
        let t: &FnTexture = &(*ptr);
        let srcrect = srcrect.as_ref().cloned();
        let destination: &mut FnTexture = &mut (*destination);
        let dstrect = dstrect.as_ref().cloned();
        t.clone_to_texture(srcrect, destination, dstrect);
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_fill_area(
        ptr: *mut FnTexture,
        area: *const Geometry,
        red: u8,
        green: u8,
        blue: u8,
    ) {
        let t: &mut FnTexture = &mut (*ptr);
        let area = area.as_ref().cloned();
        t.fill_area(area, red, green, blue);
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_get_width(
        ptr: *const FnTexture,
    ) -> u16 {
        let t: &FnTexture = &(*ptr);
        t.width()
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_get_height(
        ptr: *const FnTexture,
    ) -> u16 {
        let t: &FnTexture = &(*ptr);
        t.height()
    }
}
