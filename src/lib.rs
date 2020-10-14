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
pub const LEVEL_WIDTH: usize = 128;

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

const NUMBER_START: usize = BORDER_START + 48;

const NUMBER_100: usize = NUMBER_START + 0;
const NUMBER_200: usize = NUMBER_START + 2;
const NUMBER_500: usize = NUMBER_START + 4;
const NUMBER_1000: usize = NUMBER_START + 6;
const NUMBER_2000: usize = NUMBER_START + 8;
const NUMBER_5000: usize = NUMBER_START + 10;
const NUMBER_10000: usize = NUMBER_START + 12;
const NUMBER_BONUS_1_LEFT: usize = NUMBER_START + 14;
const NUMBER_BONUS_1_RIGHT: usize = NUMBER_START + 16;
const NUMBER_BONUS_2_LEFT: usize = NUMBER_START + 18;
const NUMBER_BONUS_2_RIGHT: usize = NUMBER_START + 20;
const NUMBER_BONUS_3_LEFT: usize = NUMBER_START + 22;
const NUMBER_BONUS_3_RIGHT: usize = NUMBER_START + 24;
const NUMBER_BONUS_4_LEFT: usize = NUMBER_START + 26;
const NUMBER_BONUS_4_RIGHT: usize = NUMBER_START + 28;
const NUMBER_BONUS_5_LEFT: usize = NUMBER_START + 30;
const NUMBER_BONUS_5_RIGHT: usize = NUMBER_START + 32;
const NUMBER_BONUS_6_LEFT: usize = NUMBER_START + 34;
const NUMBER_BONUS_6_RIGHT: usize = NUMBER_START + 36;
const NUMBER_BONUS_7_LEFT: usize = NUMBER_START + 38;
const NUMBER_BONUS_7_RIGHT: usize = NUMBER_START + 40;

const SOLID_START: usize = 4 * 48;
const SOLID_ELEVATOR: usize = SOLID_START + 23;

const ANIMATION_START: usize = SOLID_START + 4 * 48;

const ANIMATION_FOOTBOT: usize = ANIMATION_START + 10;
const ANIMATION_CARBOT: usize = ANIMATION_START + 34;
const ANIMATION_EXPLOSION: usize = ANIMATION_START + 42;
const ANIMATION_FIREWHEEL_OFF: usize = ANIMATION_START + 48;
const ANIMATION_FIREWHEEL_ON: usize = ANIMATION_START + 64;
const ANIMATION_ROBOT: usize = ANIMATION_START + 80;
const ANIMATION_BOMBFIRE: usize = ANIMATION_START + 90;
const ANIMATION_BOMB: usize = ANIMATION_START + 112;
const ANIMATION_SODA: usize = ANIMATION_START + 128;
const ANIMATION_SODAFLY: usize = ANIMATION_START + 132;
const ANIMATION_WALLCRAWLERBOT_LEFT: usize = ANIMATION_START + 136;
const ANIMATION_WALLCRAWLERBOT_RIGHT: usize = ANIMATION_START + 140;
const ANIMATION_CAMERA_LEFT: usize = ANIMATION_START + 200;
const ANIMATION_CAMERA_CENTER: usize = ANIMATION_START + 201;
const ANIMATION_CAMERA_RIGHT: usize = ANIMATION_START + 202;
const ANIMATION_BROKENWALLBG: usize = ANIMATION_START + 203;
const ANIMATION_STONEWINDOWBG: usize = ANIMATION_START + 206;
const ANIMATION_TELEPORTER1: usize = ANIMATION_START + 212;
const ANIMATION_MINE: usize = ANIMATION_START + 223;
const ANIMATION_WINDOWBG: usize = ANIMATION_START + 253;

const OBJECT_START: usize = ANIMATION_START + 6 * 48;
const OBJECT_BOX_GREY: usize = OBJECT_START + 0;
const OBJECT_SPARK_PINK: usize = OBJECT_START + 1;
const OBJECT_SPARK_BLUE: usize = OBJECT_START + 2;
const OBJECT_SPARK_WHITE: usize = OBJECT_START + 3;
const OBJECT_SPARK_GREEN: usize = OBJECT_START + 4;
const OBJECT_ELEVATOR_TOP: usize = OBJECT_START + 5;
const OBJECT_SHOT: usize = OBJECT_START + 6;
const OBJECT_BOOT: usize = OBJECT_START + 10;
const OBJECT_ROCKET: usize = OBJECT_START + 11;
const OBJECT_CLAMP: usize = OBJECT_START + 18;
const OBJECT_DUSTCLOUD: usize = OBJECT_START + 19;
const OBJECT_FIRERIGHT: usize = OBJECT_START + 24;
const OBJECT_FIRELEFT: usize = OBJECT_START + 29;
const OBJECT_STEAM: usize = OBJECT_START + 34;
const OBJECT_GUN: usize = OBJECT_START + 43;
const OBJECT_CHICKEN_SINGLE: usize = OBJECT_START + 44;
const OBJECT_CHICKEN_DOUBLE: usize = OBJECT_START + 45;
const OBJECT_FOOTBALL: usize = OBJECT_START + 58;
const OBJECT_JOYSTICK: usize = OBJECT_START + 59;
const OBJECT_DISK: usize = OBJECT_START + 60;
const OBJECT_HEALTH: usize = OBJECT_START + 61;
const OBJECT_NONHEALTH: usize = OBJECT_START + 62;
const OBJECT_GLOVE: usize = OBJECT_START + 63;
const OBJECT_ACCESS_CARD: usize = OBJECT_START + 64;
const OBJECT_BALLOON: usize = OBJECT_START + 69;
const OBJECT_NUCLEARMOLECULE: usize = OBJECT_START + 74;
const OBJECT_FALLINGBLOCK: usize = OBJECT_START + 83;
const OBJECT_POINT: usize = OBJECT_START + 85;
const OBJECT_ROTATINGCYLINDER: usize = OBJECT_START + 90;
const OBJECT_SPIKE: usize = OBJECT_START + 95;
const OBJECT_FLAG: usize = OBJECT_START + 97;
const OBJECT_BOX_BLUE: usize = OBJECT_START + 100;
const OBJECT_BOX_RED: usize = OBJECT_START + 101;
const OBJECT_RADIO: usize = OBJECT_START + 102;
const OBJECT_ACCESS_CARD_SLOT: usize = OBJECT_START + 105;
const OBJECT_GLOVE_SLOT: usize = OBJECT_START + 114;
const OBJECT_LETTER_D: usize = OBJECT_START + 118;
const OBJECT_LETTER_U: usize = OBJECT_START + 119;
const OBJECT_LETTER_K: usize = OBJECT_START + 120;
const OBJECT_LETTER_E: usize = OBJECT_START + 121;
const OBJECT_KEY_RED: usize = OBJECT_START + 124;
const OBJECT_KEY_GREEN: usize = OBJECT_START + 125;
const OBJECT_KEY_BLUE: usize = OBJECT_START + 126;
const OBJECT_KEY_PINK: usize = OBJECT_START + 127;
const OBJECT_SPIKES_UP: usize = OBJECT_START + 148;
const OBJECT_SPIKES_DOWN: usize = OBJECT_START + 149;

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
