#[derive(Clone)]
struct Geometry {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl Geometry {
    fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    fn set_w(&mut self, w: u32) {
        self.w = w;
    }
    fn set_h(&mut self, h: u32) {
        self.h = h;
    }
    fn set_data(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn w(&self) -> u32 {
        self.w
    }
    fn h(&self) -> u32 {
        self.h
    }
    fn add_x(&mut self, x: i32) {
        self.x += x;
    }
    fn add_y(&mut self, y: i32) {
        self.y += y;
    }
}
