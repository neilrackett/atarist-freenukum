#[derive(Clone)]
#[repr(C)]
pub struct Geometry {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl Geometry {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Geometry {
        Geometry { x, y, w, h }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    pub fn set_w(&mut self, w: u32) {
        self.w = w;
    }
    pub fn set_h(&mut self, h: u32) {
        self.h = h;
    }
    pub fn set_data(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn w(&self) -> u32 {
        self.w
    }
    pub fn h(&self) -> u32 {
        self.h
    }
    pub fn add_x(&mut self, x: i32) {
        self.x += x;
    }
    pub fn add_y(&mut self, y: i32) {
        self.y += y;
    }
}

mod ffi {
    type FnGeometry = super::Geometry;

    #[no_mangle]
    pub unsafe extern "C" fn fn_geometry_create(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> FnGeometry {
        FnGeometry::new(x, y, w, h)
    }
}
