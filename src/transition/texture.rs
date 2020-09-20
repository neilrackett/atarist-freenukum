/*
use sdl::video::{Surface, SurfaceFlag};

#[repr(C)]
pub struct Texture {
    w: isize,
    h: isize,
    surface: Surface,
}

impl Texture {
    pub fn create(w: isize, h: isize, bits_per_pixel: isize) -> Texture {
        let flags = vec![SurfaceFlag::HWSurface];
        let surface =
            Surface::new(&flags, w, h, bits_per_pixel, 0, 0, 0, 0)
                .unwrap();

        Texture { w, h, surface }
    }
}

mod ffi {
    use super::Texture;

    #[repr(transparent)]
    pub struct FnTexture(Box<Texture>);

    #[no_mangle]
    pub unsafe extern "C" fn fn_texture_create(
        w: isize,
        h: isize,
        bits_per_pixel: isize,
    ) -> FnTexture {
        FnTexture(Box::new(Texture::create(w, h, bits_per_pixel)))
    }
}
*/
