use crate::tile::{Category, Tiles};
use crate::{
    HALFTILE_HEIGHT, HALFTILE_WIDTH, INVENTORY_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

pub fn draw<'t, C: RenderTarget>(
    target: &mut Canvas<C>,
    tiles: &Tiles<'t>,
    dst: &Rect,
) -> Result<(), String> {
    let borders = tiles.get(&Category::Border).unwrap();
    {
        // Horizontal borders
        let mut r = Rect::new(0, 0, dst.width(), TILE_HEIGHT as u32);
        target.copy(&borders[BorderType::Top.index()], None, r)?;

        r.offset(0, dst.height() as i32 - TILE_HEIGHT as i32);
        target.copy(&borders[BorderType::Bottom.index()], None, r)?;
    }
    {
        // Vertical borders
        let mut r = Rect::new(0, 0, TILE_WIDTH as u32, dst.height());
        target.copy(&borders[BorderType::Left.index()], None, r)?;

        r.offset(
            dst.width() as i32
                - (INVENTORY_WIDTH as i32 + 2) * TILE_WIDTH as i32,
            0,
        );
        target.copy(
            &borders[BorderType::VerticalMiddle.index()],
            None,
            r,
        )?;
        r.offset((INVENTORY_WIDTH + 1) as i32 * TILE_WIDTH as i32, 0);

        target.copy(&borders[BorderType::Right.index()], None, r)?;
    }
    {
        // Left corners
        let mut r = Rect::new(0, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
        target.copy(&borders[BorderType::TopLeft.index()], None, r)?;

        r.offset(0, dst.height() as i32 - TILE_HEIGHT as i32);
        target.copy(&borders[BorderType::BottomLeft.index()], None, r)?;
    }
    {
        // Inventory and right corners
        let mut x = dst.width() as i32
            - (INVENTORY_WIDTH as i32 + 2) * TILE_WIDTH as i32;
        let mut r_left =
            Rect::new(x, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);

        x += TILE_WIDTH as i32;
        let mut r_border = Rect::new(
            x,
            0,
            TILE_WIDTH as u32 * INVENTORY_WIDTH as u32,
            TILE_HEIGHT as u32,
        );
        x += TILE_WIDTH as i32;

        let mut r_title0 =
            Rect::new(x, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
        x += TILE_WIDTH as i32;
        let mut r_title1 =
            Rect::new(x, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
        x += TILE_WIDTH as i32;
        let mut r_title2 =
            Rect::new(x, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
        x += TILE_WIDTH as i32;

        let mut r_right =
            Rect::new(x, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);

        // Top (score)
        let left = &borders[BorderType::TopT.index()];
        let middle = &borders[BorderType::HorizontalMiddle.index()];
        let title0 = &borders[BorderType::Score0.index()];
        let title1 = &borders[BorderType::Score1.index()];
        let right = &borders[BorderType::TopRight.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(title0, None, r_title0)?;
        target.copy(title1, None, r_title1)?;
        target.copy(right, None, r_right)?;

        // Health
        let down = HALFTILE_HEIGHT as i32 * 5;
        r_left.offset(0, down);
        r_border.offset(0, down);
        r_title0.offset(0, down);
        r_title1.offset(0, down);
        r_title2.offset(0, down);
        r_right.offset(0, down);

        let left = &borders[BorderType::LeftT.index()];
        let title0 = &borders[BorderType::Health0.index()];
        let title1 = &borders[BorderType::Health1.index()];
        let right = &borders[BorderType::RightT.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(title0, None, r_title0)?;
        target.copy(title1, None, r_title1)?;
        target.copy(right, None, r_right)?;

        // Firepower
        let down = HALFTILE_HEIGHT as i32 * 5;
        r_left.offset(0, down);
        r_border.offset(0, down);
        r_title0.offset(-(HALFTILE_WIDTH as i32), down);
        r_title1.offset(-(HALFTILE_WIDTH as i32), down);
        r_title2.offset(-(HALFTILE_WIDTH as i32), down);
        r_right.offset(0, down);

        let left = &borders[BorderType::LeftT.index()];
        let title0 = &borders[BorderType::FirePower0.index()];
        let title1 = &borders[BorderType::FirePower1.index()];
        let title2 = &borders[BorderType::FirePower2.index()];
        let right = &borders[BorderType::RightT.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(title0, None, r_title0)?;
        target.copy(title1, None, r_title1)?;
        target.copy(title2, None, r_title2)?;
        target.copy(right, None, r_right)?;

        // Firepower
        let down = HALFTILE_HEIGHT as i32 * 6;
        r_left.offset(0, down);
        r_border.offset(0, down);
        r_title0.offset(0, down);
        r_title1.offset(0, down);
        r_title2.offset(0, down);
        r_right.offset(0, down);

        let left = &borders[BorderType::LeftT.index()];
        let title0 = &borders[BorderType::Inventory0.index()];
        let title1 = &borders[BorderType::Inventory1.index()];
        let title2 = &borders[BorderType::Inventory2.index()];
        let right = &borders[BorderType::RightT.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(title0, None, r_title0)?;
        target.copy(title1, None, r_title1)?;
        target.copy(title2, None, r_title2)?;
        target.copy(right, None, r_right)?;

        // Bottom
        let y = dst.height() as i32 - TILE_HEIGHT as i32;
        r_left.set_y(y);
        r_border.set_y(y);
        r_right.set_y(y);

        let left = &borders[BorderType::BottomT.index()];
        let right = &borders[BorderType::BottomRight.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(right, None, r_right)?;
    }
    {
        // Bottom help info
        let bottom_w = dst.width() as i32
            - (INVENTORY_WIDTH + 3) as i32 * TILE_WIDTH as i32;
        let x = TILE_WIDTH as i32 + bottom_w / 2 - 2 * TILE_WIDTH as i32;
        let y = dst.height() as i32 - TILE_HEIGHT as i32;
        let mut r = Rect::new(x, y, TILE_WIDTH as u32, TILE_HEIGHT as u32);

        let tile = &borders[BorderType::BottomHelp0.index()];
        target.copy(tile, None, r)?;

        r.offset(TILE_WIDTH as i32, 0);
        let tile = &borders[BorderType::BottomHelp1.index()];
        target.copy(tile, None, r)?;

        r.offset(TILE_WIDTH as i32, 0);
        let tile = &borders[BorderType::BottomHelp2.index()];
        target.copy(tile, None, r)?;

        r.offset(TILE_WIDTH as i32, 0);
        let tile = &borders[BorderType::BottomHelp3.index()];
        target.copy(tile, None, r)?;
    }

    Ok(())
}

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
enum BorderType {
    Left,
    VerticalMiddle,
    Top,
    Bottom,
    TopLeft,
    TopT,
    BottomT,
    BottomLeft,
    HorizontalMiddle,
    TopRight,
    Right,
    BottomRight,
    TopWithP,
    Middle,
    LeftT,
    RightT,
    TopWithR,
    MessageMiddle,
    MessageTopLeft,
    MessageTopRight,
    MessageBottomLeft,
    MessageBottomRight,
    MessageLeft,
    MessageRight,
    MessageTop,
    MessageBottom,
    BottomHelp0,
    BottomHelp1,
    BottomHelp2,
    BottomHelp3,
    Inventory0,
    Inventory1,
    Inventory2,
    FirePower0,
    FirePower1,
    FirePower2,
    Health0,
    Health1,
    Score0,
    Score1,
    TopWithF,
    TopWithS,
    HealthBlinking0,
    HealthBlinking1,
}

impl BorderType {
    fn index(&self) -> usize {
        *self as usize
    }
}
