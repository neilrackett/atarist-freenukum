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
pub const BACKDROP_HEIGHT: usize = 10;
pub const MAX_LIFE: usize = 8;
pub const MAX_FIREPOWER: usize = 4;
pub const SCORE_DIGITS: usize = 8;

/// The height of the level in full tiles
pub const LEVEL_HEIGHT: usize = 90;
/// The width of the level in full tiles
pub const LEVEL_WIDTH : usize =    128;

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
const BORDER_GREY_START: usize = BORDER_START;

const SOLID_START: usize = 4 * 48;
const ANIMATION_START: usize = SOLID_START + 4 * 48;

const ANIMATION_FOOTBOT: usize = ANIMATION_START + 10;
const ANIMATION_CARBOT: usize = ANIMATION_START + 34;
const ANIMATION_WALLCRAWLERBOT_LEFT: usize = ANIMATION_START + 136;
const ANIMATION_WALLCRAWLERBOT_RIGHT: usize = ANIMATION_START + 140;

const OBJECT_START: usize = ANIMATION_START + 6 * 48;
const OBJECT_SHOT: usize = OBJECT_START + 6;
const OBJECT_GUN: usize = OBJECT_START + 43;
const OBJECT_HEALTH: usize = OBJECT_START + 61;
const OBJECT_NONHEALTH: usize = OBJECT_START + 62;
const OBJECT_BOOT: usize = OBJECT_START + 10;
const OBJECT_CLAMP: usize = OBJECT_START + 18;
const OBJECT_ROBOHAND: usize = OBJECT_START + 63;
const OBJECT_ACCESS_CARD: usize = OBJECT_START + 64;
const OBJECT_POINT: usize = OBJECT_START + 85;
const OBJECT_KEY_RED: usize = OBJECT_START + 124;
const OBJECT_KEY_GREEN: usize = OBJECT_START + 125;
const OBJECT_KEY_BLUE: usize = OBJECT_START + 126;
const OBJECT_KEY_PINK: usize = OBJECT_START + 127;

const INVENTORY_KEY_RED: u8 = 0x01 << 7;
const INVENTORY_KEY_GREEN: u8 = 0x01 << 6;
const INVENTORY_KEY_BLUE: u8 = 0x01 << 5;
const INVENTORY_KEY_PINK: u8 = 0x01 << 4;
const INVENTORY_BOOT: u8 = 0x01 << 3;
const INVENTORY_GLOVE: u8 = 0x01 << 2;
const INVENTORY_CLAMP: u8 = 0x01 << 1;
const INVENTORY_ACCESS_CARD: u8 = 0x01 << 0;

pub use game::Game;
pub use settings::Settings;

pub mod transition;

pub use transition::geometry::Geometry;
pub use transition::texture::Texture;

fn config_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("", "", "freenukum")
        .unwrap()
        .config_dir()
        .to_path_buf()
}
