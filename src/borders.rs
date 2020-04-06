use crate::{
    HALFTILE_HEIGHT, HALFTILE_WIDTH, INVENTORY_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, Texture, TextureQuery};
use sdl2::surface::Surface;

pub struct Borders {}

pub fn draw_borders<'t, C: RenderTarget>(
    target: &mut Canvas<C>,
    tiles: &[Texture<'t>],
    dst: &Rect,
) -> Result<(), String> {
    {
        // Horizontal borders
        let mut r = Rect::new(0, 0, dst.width(), TILE_HEIGHT as u32);
        target.copy(&tiles[BorderType::Top.index()], None, r)?;

        r.offset(0, dst.height() as i32 - TILE_HEIGHT as i32);
        target.copy(&tiles[BorderType::Bottom.index()], None, r)?;
    }
    {
        // Vertical borders
        let mut r = Rect::new(0, 0, TILE_WIDTH as u32, dst.height());
        target.copy(&tiles[BorderType::Left.index()], None, r)?;

        r.offset(
            dst.width() as i32
                - (INVENTORY_WIDTH as i32 + 2) * TILE_WIDTH as i32,
            0,
        );
        target.copy(
            &tiles[BorderType::VerticalMiddle.index()],
            None,
            r,
        )?;
        r.offset((INVENTORY_WIDTH + 1) as i32 * TILE_WIDTH as i32, 0);

        target.copy(&tiles[BorderType::Right.index()], None, r)?;
    }
    {
        // Inventory
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
        let left = &tiles[BorderType::TopT.index()];
        let middle = &tiles[BorderType::HorizontalMiddle.index()];
        let title0 = &tiles[BorderType::Score0.index()];
        let title1 = &tiles[BorderType::Score1.index()];
        let right = &tiles[BorderType::TopRight.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(title0, None, r_title0)?;
        target.copy(title1, None, r_title1)?;
        target.copy(right, None, r_right)?;

        let down = HALFTILE_HEIGHT as i32 * 5;
        r_left.offset(0, down);
        r_border.offset(0, down);
        r_title0.offset(0, down);
        r_title1.offset(0, down);
        r_title2.offset(0, down);
        r_right.offset(0, down);

        // Health
        let left = &tiles[BorderType::LeftT.index()];
        let title0 = &tiles[BorderType::Health0.index()];
        let title1 = &tiles[BorderType::Health1.index()];
        let right = &tiles[BorderType::RightT.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(title0, None, r_title0)?;
        target.copy(title1, None, r_title1)?;
        target.copy(right, None, r_right)?;

        // Firepower
        let down = HALFTILE_HEIGHT as i32 * 6;
        r_left.offset(0, down);
        r_border.offset(0, down);
        r_title0.offset(-(HALFTILE_WIDTH as i32), down);
        r_title1.offset(-(HALFTILE_WIDTH as i32), down);
        r_title2.offset(-(HALFTILE_WIDTH as i32), down);
        r_right.offset(0, down);

        let left = &tiles[BorderType::LeftT.index()];
        let title0 = &tiles[BorderType::FirePower0.index()];
        let title1 = &tiles[BorderType::FirePower1.index()];
        let title2 = &tiles[BorderType::FirePower2.index()];
        let right = &tiles[BorderType::RightT.index()];
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

        let left = &tiles[BorderType::LeftT.index()];
        let title0 = &tiles[BorderType::Inventory0.index()];
        let title1 = &tiles[BorderType::Inventory1.index()];
        let title2 = &tiles[BorderType::Inventory2.index()];
        let right = &tiles[BorderType::RightT.index()];
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

        let left = &tiles[BorderType::BottomT.index()];
        let right = &tiles[BorderType::BottomRight.index()];
        target.copy(left, None, r_left)?;
        target.copy(middle, None, r_border)?;
        target.copy(right, None, r_right)?;
    }
    /*
    // Corners
    {
        let dst = Rect::new(0, 0, TILE_WIDTH as u32, TILE_HEIGHT as u32);
        target.copy(&tiles[BorderType::TopLeft.index()], None, dst)?;
    }
    {
        let dst = Rect::new(
            dst.width() as i32 - TILE_WIDTH as i32,
            0,
            TILE_WIDTH as u32,
            TILE_HEIGHT as u32,
        );
        target.copy(&tiles[BorderType::TopRight.index()], None, dst)?;
    }
    {
        let dst = Rect::new(
            0,
            dst.height() as i32 - TILE_HEIGHT as i32,
            TILE_WIDTH as u32,
            TILE_HEIGHT as u32,
        );
        target.copy(&tiles[BorderType::BottomLeft.index()], None, dst)?;
    }
    {
        let dst = Rect::new(
            dst.width() as i32 - TILE_WIDTH as i32,
            dst.height() as i32 - TILE_HEIGHT as i32,
            TILE_WIDTH as u32,
            TILE_HEIGHT as u32,
        );
        target.copy(&tiles[BorderType::BottomRight.index()], None, dst)?;
    }
    {
        let dst = Rect::new(
            dst.width() as i32
                - TILE_WIDTH as i32 * (2 + INVENTORY_WIDTH as i32),
            0,
            TILE_WIDTH as u32,
            TILE_HEIGHT as u32,
        );
        target.copy(&tiles[BorderType::TopT.index()], None, dst)?;
    }
    */

    Ok(())
}

fn borders_blit_tile(
    pixelsize: u8,
    target: &Surface,
    tile: &Texture,
    x: i32,
    y: i32,
) {
    let TextureQuery { width, height, .. } = tile.query();

    let dst = Rect::new(
        HALFTILE_WIDTH as i32 * x,
        HALFTILE_HEIGHT as i32 * y,
        width,
        height,
    );

    //tile.blit(None, target, dst);

    unimplemented!();
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
