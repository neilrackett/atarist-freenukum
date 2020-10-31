use transdl::video::Surface;

#[derive(Copy, Clone, Debug, Default)]
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

    pub fn overlaps(&self, other: Geometry) -> bool {
        if self.x as i32 + self.w as i32 <= other.x as i32 {
            return false;
        }
        if other.x as i32 + other.w as i32 <= self.x as i32 {
            return false;
        }
        if self.y as i32 + self.h as i32 <= other.y as i32 {
            return false;
        }
        if other.y as i32 + other.h as i32 <= self.y as i32 {
            return false;
        }
        true
    }

    pub fn horizontal_distance(&self, other: Geometry) -> i32 {
        if (self.x as i32 + self.w as i32) < other.x as i32 {
            return other.x as i32 - self.w as i32 - self.x as i32;
        }
        if (other.x as i32 + other.w as i32) < self.x as i32 {
            return -(self.x as i32 - other.w as i32 - other.x as i32);
        }
        return 0;
    }

    pub fn touches(&self, other: Geometry) -> bool {
        let mut r1 = self.clone();
        r1.w += 1;
        r1.h += 1;
        let mut r2 = other.clone();
        r2.w += 1;
        r2.h += 1;
        r1.overlaps(r2)
    }

    pub fn overlaps_vertically(&self, other: Geometry) -> bool {
        if self.y as i32 + self.h as i32 <= other.y as i32 {
            return false;
        }
        if other.y as i32 + other.h as i32 <= self.y as i32 {
            return false;
        }
        true
    }

    pub fn draw_outline(&self, surface: &mut Surface, color: u32) {
        {
            let mut r = self.clone();
            r.w = 1;
            surface.fill_rect(r.as_sdl_rect(), color);
        }
        {
            let mut r = self.clone();
            r.x += r.w as i16 - 1;
            r.w = 1;
            surface.fill_rect(r.as_sdl_rect(), color);
        }
        {
            let mut r = self.clone();
            r.h = 1;
            surface.fill_rect(r.as_sdl_rect(), color);
        }
        {
            let mut r = self.clone();
            r.y += r.h as i16 - 1;
            r.h = 1;
            surface.fill_rect(r.as_sdl_rect(), color);
        }
    }
}
