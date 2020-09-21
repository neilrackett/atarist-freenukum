#[derive(Clone)]
#[repr(C)]
pub struct Geometry {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

impl Geometry {
    pub fn new(x: i16, y: i16, w: u16, h: u16) -> Geometry {
        Geometry { x, y, w, h }
    }

    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }
    pub fn set_w(&mut self, w: u16) {
        self.w = w;
    }
    pub fn set_h(&mut self, h: u16) {
        self.h = h;
    }
    pub fn set_data(&mut self, x: i16, y: i16, w: u16, h: u16) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    pub fn x(&self) -> i16 {
        self.x
    }
    pub fn y(&self) -> i16 {
        self.y
    }
    pub fn w(&self) -> u16 {
        self.w
    }
    pub fn h(&self) -> u16 {
        self.h
    }
    pub fn add_x(&mut self, x: i16) {
        self.x += x;
    }
    pub fn add_y(&mut self, y: i16) {
        self.y += y;
    }

    pub fn as_sdl_rect(&self) -> transdl::video::Rect {
        transdl::video::Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
}

mod ffi {
    type FnGeometry = super::Geometry;

    #[no_mangle]
    pub unsafe extern "C" fn fn_geometry_create(
        x: i16,
        y: i16,
        w: u16,
        h: u16,
    ) -> FnGeometry {
        FnGeometry::new(x, y, w, h)
    }

    #[no_mangle]
    pub unsafe extern "C" fn fn_geometry_as_sdl_rect(
        ptr: *const FnGeometry,
    ) -> transdl::ll::video::SDL_Rect {
        let g: &FnGeometry = &(*ptr);
        g.as_sdl_rect()
    }
}
