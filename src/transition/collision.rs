use transdl::video::{Rect, Surface};

fn overlap_rect_rect(r1: Rect, r2: Rect) -> bool {
    overlap_area_area(r1.x, r1.y, r1.w, r1.h, r2.x, r2.y, r2.w, r2.h)
}

fn overlap_area_area<I: Into<i64> + Copy, U: Into<i64> + Copy>(
    x1: I,
    y1: I,
    w1: U,
    h1: U,
    x2: I,
    y2: I,
    w2: U,
    h2: U,
) -> bool {
    if x1.into() + w1.into() <= x2.into() {
        return false;
    }
    if x2.into() + w2.into() <= x1.into() {
        return false;
    }
    if y1.into() + h1.into() <= y2.into() {
        return false;
    }
    if y2.into() + h2.into() <= y1.into() {
        return false;
    }
    true
}

fn area_draw<I: Into<i64> + Copy, U: Into<i64> + Copy>(
    destination: &mut Surface,
    x: I,
    y: I,
    w: U,
    h: U,
    color: u32,
) {
    let rect = Rect {
        x: x.into() as i16,
        y: y.into() as i16,
        w: w.into() as u16,
        h: h.into() as u16,
    };
    rect_draw(destination, rect, color);
}

fn rect_draw(destination: &mut Surface, rect: Rect, color: u32) {
    {
        // left border
        let mut rect = rect.clone();
        rect.w = 1;
        destination.fill_rect(&rect, color);
    }
    {
        // right border
        let mut rect = rect.clone();
        rect.x = rect.x + rect.w as i16 - 1;
        rect.w = 1;
        destination.fill_rect(&rect, color);
    }
    {
        // top border
        let mut rect = rect.clone();
        rect.h = 1;
        destination.fill_rect(&rect, color);
    }
    {
        // bottom border
        let mut rect = rect.clone();
        rect.y = rect.y + rect.h as i16 - 1;
        rect.h = 1;
        destination.fill_rect(&rect, color);
    }
}

/*
fn distance_horizontal_area_area(
    x1: dyn Into<i64> + Copy,
    w1: dyn Into<i64> + Copy,
    x2: dyn Into<i64> + Copy,
    w2: dyn Into<i64> + Copy,
) -> i64 {
    if x1.into() + w1.into() < x2.into() {
        return x2.into() - w1.into() - x1.into();
    }
    if x2.into() + w2.into() < x1.into() {
        return -(x1.into() - w2.into() - x2.into());
    }
    0
}

fn distance_horizontal_rect_area<
    I: Into<i64> + Copy,
    U: Into<i64> + Copy,
>(
    rect: Rect,
    x: I,
    w: U,
) -> i64 {
    distance_horizontal_area_area(rect.x as i64, rect.w as i64, x, w)
}
*/
