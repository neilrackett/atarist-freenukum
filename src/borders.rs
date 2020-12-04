use super::hero::{Firepower, Inventory, InventoryItem};
use super::text;
use crate::rendering::{MovePositionRenderer, Renderer, TileIndex};
use crate::{
    Result, BORDER_GREY_START, FONT_HEIGHT, FONT_WIDTH, HALFTILE_HEIGHT,
    HALFTILE_WIDTH, MAX_LIFE, OBJECT_ACCESS_CARD, OBJECT_BOOT,
    OBJECT_CLAMP, OBJECT_GLOVE, OBJECT_GUN, OBJECT_HEALTH,
    OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK, OBJECT_KEY_RED,
    OBJECT_NONHEALTH, OBJECT_SHOT, SCORE_DIGITS, TILE_HEIGHT, TILE_WIDTH,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};
use sdl2::rect::Point;

pub struct Borders {}

#[rustfmt::skip]
const BORDERS:
[i32; (2 * WINDOW_HEIGHT / TILE_HEIGHT) as usize * (2 * WINDOW_WIDTH / TILE_WIDTH) as usize] =
[
     4,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1,
     2,-1, 2,-1, 2,-1, 2,-1, 5,-1, 8,-1,38,-1,39,-1, 8,-1, 9,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,14,-1, 8,-1,36,-1,37,-1, 8,-1,15,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,14,-1, 8,33,-1,34,-1,35,-1, 8,15,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,14,-1, 8,30,-1,31,-1,32,-1, 8,15,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     7,-1, 3,-1, 3,-1, 3, -1,3,-1, 3,26,-1,27,-1,28,-1,29,-1, 3,
     3,-1, 3,-1, 3,-1, 3,-1, 6,-1, 8,-1, 8,-1, 8,-1, 8,-1,11,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
];

impl Borders {
    fn render_tile(
        &self,
        x: i32,
        y: i32,
        renderer: &mut dyn Renderer,
        tile: TileIndex,
    ) -> Result<()> {
        renderer.place_tile(tile, Point::new(x, y))?;
        Ok(())
    }

    fn render_iter<I>(
        &self,
        columns: u8,
        grid_x: u16,
        grid_y: u16,
        renderer: &mut dyn Renderer,
        borders: I,
    ) -> Result<()>
    where
        I: Iterator<Item = Option<usize>>,
    {
        for (i, border) in borders.enumerate() {
            if let Some(border) = border {
                self.render_tile(
                    (i as i32 % columns as i32) * (grid_x as i32),
                    (i as i32 / columns as i32) * (grid_y as i32),
                    renderer,
                    border,
                )?;
            }
        }
        Ok(())
    }

    pub fn render(&self, renderer: &mut dyn Renderer) -> Result<()> {
        self.render_iter(
            (2 * WINDOW_WIDTH / TILE_WIDTH) as u8,
            HALFTILE_WIDTH as u16,
            HALFTILE_HEIGHT as u16,
            renderer,
            BORDERS.iter().map(|i| {
                if *i < 0 {
                    None
                } else {
                    Some(*i as usize + BORDER_GREY_START)
                }
            }),
        )
    }

    pub fn render_life(
        &self,
        health: u8,
        renderer: &mut dyn Renderer,
    ) -> Result<()> {
        let health = std::cmp::min(health as usize, MAX_LIFE);
        let iter = (0..MAX_LIFE).map(|i| {
            if i < health {
                Some(OBJECT_HEALTH)
            } else {
                Some(OBJECT_NONHEALTH)
            }
        });
        let mut move_renderer = MovePositionRenderer {
            offset_x: 30 * HALFTILE_WIDTH as i32,
            offset_y: (15 * HALFTILE_HEIGHT as i32) / 2,
            upstream: renderer,
        };
        self.render_iter(
            MAX_LIFE as u8,
            HALFTILE_WIDTH as u16,
            HALFTILE_HEIGHT as u16,
            &mut move_renderer,
            iter,
        )
    }

    pub fn render_score(
        &self,
        score: u128,
        renderer: &mut dyn Renderer,
    ) -> Result<()> {
        let score = std::cmp::min(99999999, score);
        let score_string =
            format!("{0:0width$}", score, width = SCORE_DIGITS);

        let mut position_renderer = MovePositionRenderer {
            offset_x: 30 * FONT_WIDTH as i32,
            offset_y: 3 * FONT_HEIGHT as i32,
            upstream: renderer,
        };

        text::render(&mut position_renderer, &score_string)
    }

    pub fn render_firepower(
        &self,
        firepower: &Firepower,
        renderer: &mut dyn Renderer,
    ) -> Result<()> {
        const GUN: Option<usize> = Some(OBJECT_GUN);
        const SHOT: Option<usize> = Some(OBJECT_SHOT);

        let shots = firepower.num_shots();

        let shot0 = if shots > 0 { SHOT } else { None };
        let shot1 = if shots > 1 { SHOT } else { None };
        let shot2 = if shots > 2 { SHOT } else { None };
        let shot3 = if shots > 3 { SHOT } else { None };

        #[rustfmt::skip]
        let tiles = vec![
            None,  None, None,  GUN,  None,  None, None,  None,
            None,  None, None,  None, None,  None, None,  None,
            shot0, None, shot1, None, shot2, None, shot3, None,
            None,  None, None,  None, None,  None, None,  None,
        ];

        let mut move_renderer = MovePositionRenderer {
            offset_x: 15 * TILE_WIDTH as i32,
            offset_y: 6 * TILE_HEIGHT as i32,
            upstream: renderer,
        };
        self.render_iter(
            MAX_LIFE as u8,
            HALFTILE_WIDTH as u16,
            HALFTILE_HEIGHT as u16,
            &mut move_renderer,
            tiles.into_iter(),
        )
    }

    pub fn render_inventory(
        &self,
        inventory: &Inventory,
        renderer: &mut dyn Renderer,
    ) -> Result<()> {
        let red_key = if inventory.is_set(InventoryItem::KeyRed) {
            Some(OBJECT_KEY_RED)
        } else {
            None
        };
        let green_key = if inventory.is_set(InventoryItem::KeyGreen) {
            Some(OBJECT_KEY_GREEN)
        } else {
            None
        };
        let blue_key = if inventory.is_set(InventoryItem::KeyBlue) {
            Some(OBJECT_KEY_BLUE)
        } else {
            None
        };
        let pink_key = if inventory.is_set(InventoryItem::KeyPink) {
            Some(OBJECT_KEY_PINK)
        } else {
            None
        };
        let boot = if inventory.is_set(InventoryItem::Boot) {
            Some(OBJECT_BOOT)
        } else {
            None
        };
        let glove = if inventory.is_set(InventoryItem::Glove) {
            Some(OBJECT_GLOVE)
        } else {
            None
        };
        let clamp = if inventory.is_set(InventoryItem::Clamp) {
            Some(OBJECT_CLAMP)
        } else {
            None
        };
        let access_card = if inventory.is_set(InventoryItem::AccessCard) {
            Some(OBJECT_ACCESS_CARD)
        } else {
            None
        };

        #[rustfmt::skip]
        let tiles = vec![
            red_key, green_key, blue_key, pink_key,
            boot, glove, clamp, access_card,
        ];

        let mut move_renderer = MovePositionRenderer {
            offset_x: 15 * TILE_WIDTH as i32,
            offset_y: 9 * TILE_HEIGHT as i32,
            upstream: renderer,
        };
        self.render_iter(
            4,
            TILE_WIDTH as u16,
            TILE_HEIGHT as u16,
            &mut move_renderer,
            tiles.into_iter(),
        )
    }
}
