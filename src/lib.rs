#[macro_use]
extern crate serde_derive;

use anyhow::Result;

mod borders;
mod game;
mod settings;
mod sprite;
mod text;
pub mod tile;

pub const HALFTILE_WIDTH: usize = 8;
pub const HALFTILE_HEIGHT: usize = 8;
pub const TILE_WIDTH: usize = HALFTILE_WIDTH * 2;
pub const TILE_HEIGHT: usize = HALFTILE_HEIGHT * 2;
pub const MAX_TILES_PER_FILE: usize = 50;
pub const HEALTH_COUNT: usize = 8;
pub const INVENTORY_WIDTH: usize = HEALTH_COUNT / 2;
pub const FONT_WIDTH: usize = 8;
pub const FONT_HEIGHT: usize = 8;
pub const WINDOW_WIDTH: usize = 320;
pub const WINDOW_HEIGHT: usize = 200;
pub const PICTURE_WIDTH: usize = 40;
pub const PICTURE_HEIGHT: usize = 200;
pub const BACKDROP_WIDTH: usize = 13;
pub const BACKDROP_HEIGHT: usize = 13;

const FONT_START: usize = 19 * 48 + 3 * 50;
const FONT_ASCII_UPPERCASE: usize = FONT_START + 10;
const FONT_ASCII_LOWERCASE: usize = FONT_START + 69;
const FONT_QUESTIONMARK: usize = FONT_START + 67;
const BORDER_START: usize = 19 * 48 + 5 * 50;
const BORDER_BLUE_MIDDLE: usize = BORDER_START + 17;
const BORDER_BLUE_TOPLEFT: usize = BORDER_START + 18;
const BORDER_BLUE_TOPRIGHT: usize = BORDER_START + 19;
const BORDER_BLUE_BOTTOMLEFT: usize = BORDER_START + 20;
const BORDER_BLUE_BOTTOMRIGHT: usize = BORDER_START + 21;
const BORDER_BLUE_LEFT: usize = BORDER_START + 22;
const BORDER_BLUE_RIGHT: usize = BORDER_START + 23;
const BORDER_BLUE_TOP: usize = BORDER_START + 24;
const BORDER_BLUE_BOTTOM: usize = BORDER_START + 25;

pub use game::Game;
pub use settings::Settings;

pub mod transition;

pub use transition::geometry::Geometry;
pub use transition::texture::Texture;
