#[macro_use]
extern crate serde_derive;

pub mod actor;
pub mod backdrop;
pub mod borders;
pub mod data;
pub mod episodes;
pub mod event;
pub mod game;
pub mod geometry;
pub mod graphics;
pub mod hero;
pub mod infobox;
pub mod inputbox;
pub mod inputfield;
pub mod level;
pub mod mainmenu;
pub mod menu;
pub mod messagebox;
pub mod picture;
pub mod rendering;
pub mod settings;
pub mod shot;
pub mod text;
pub mod tile;
pub mod tilecache;
pub mod tileprovider;

use anyhow::Result;
use hero::Hero;
use tileprovider::TileProvider;

pub const GAME_INTERVAL: u32 = 80;

pub const HALFTILE_WIDTH: u32 = 8;
pub const HALFTILE_HEIGHT: u32 = 8;
pub const TILE_WIDTH: u32 = HALFTILE_WIDTH * 2;
pub const TILE_HEIGHT: u32 = HALFTILE_HEIGHT * 2;
pub const MATILES_PER_FILE: usize = 50;
pub const HEALTH_COUNT: usize = 8;
pub const INVENTORY_WIDTH: usize = HEALTH_COUNT / 2;
pub const FONT_WIDTH: u32 = 8;
pub const FONT_HEIGHT: u32 = 8;
pub const WINDOW_WIDTH: u32 = 320;
pub const WINDOW_HEIGHT: u32 = 200;
pub const PICTURE_WIDTH: u32 = 40;
pub const PICTURE_HEIGHT: u32 = 200;
pub const BACKDROP_WIDTH: u32 = 13;
pub const BACKDROP_HEIGHT: u32 = 10;
pub const MAX_LIFE: usize = 8;
pub const MAX_FIREPOWER: usize = 4;
pub const SCORE_DIGITS: usize = 8;
pub const LEVELWINDOW_WIDTH: u32 = 13;
pub const LEVELWINDOW_HEIGHT: u32 = 10;

/// The height of the level in full tiles
pub const LEVEL_HEIGHT: u32 = 90;
/// The width of the level in full tiles
pub const LEVEL_WIDTH: u32 = 128;

const BACKGROUND_START: usize = 0;
const BACKGROUND_LIGHT_GREY: usize = BACKGROUND_START + 70;

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

const NUMBER_100: usize = NUMBER_START;
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
const SOLID_SHOOTABLE_WALL_BRICKS: usize = SOLID_START;
const SOLID_ELEVATOR: usize = SOLID_START + 23;
const SOLID_BLACK: usize = SOLID_START + 65;
const SOLID_EXPANDINGFLOOR: usize = SOLID_START + 191;
const SOLID_CONVEYORBELT: usize = SOLID_START + 0x1C;
const SOLID_CONVEYORBELT_LEFTEND: usize = SOLID_CONVEYORBELT;
const SOLID_CONVEYORBELT_CENTER: usize = SOLID_CONVEYORBELT + 4;
const SOLID_CONVEYORBELT_RIGHTEND: usize = SOLID_CONVEYORBELT + 6;

const SOLID_END: usize = SOLID_START + 4 * 48;
const ANIMATION_START: usize = SOLID_END;

const _ANIMATION_FOOTBOT: usize = ANIMATION_START + 10;
const ANIMATION_CARBOT: usize = ANIMATION_START + 34;
const ANIMATION_EXPLOSION: usize = ANIMATION_START + 42;
const ANIMATION_FIREWHEEL_OFF: usize = ANIMATION_START + 48;
const ANIMATION_FIREWHEEL_ON: usize = ANIMATION_START + 64;
const ANIMATION_ROBOT: usize = ANIMATION_START + 80;
const ANIMATION_BOMBFIRE: usize = ANIMATION_START + 90;
const ANIMATION_EXITDOOR: usize = ANIMATION_START + 96;
const ANIMATION_BOMB: usize = ANIMATION_START + 112;
const ANIMATION_SODA: usize = ANIMATION_START + 128;
const ANIMATION_SODAFLY: usize = ANIMATION_START + 132;
const ANIMATION_WALLCRAWLERBOT_LEFT: usize = ANIMATION_START + 136;
const ANIMATION_WALLCRAWLERBOT_RIGHT: usize = ANIMATION_START + 140;
const ANIMATION_FAN: usize = ANIMATION_START + 156;
const ANIMATION_CAMERA_LEFT: usize = ANIMATION_START + 200;
const ANIMATION_CAMERA_CENTER: usize = ANIMATION_START + 201;
const ANIMATION_CAMERA_RIGHT: usize = ANIMATION_START + 202;
const ANIMATION_BROKENWALLBG: usize = ANIMATION_START + 203;
const ANIMATION_STONEWINDOWBG: usize = ANIMATION_START + 206;
const ANIMATION_TELEPORTER1: usize = ANIMATION_START + 212;
const ANIMATION_MINE: usize = ANIMATION_START + 223;
const ANIMATION_WINDOWBG: usize = ANIMATION_START + 253;
const ANIMATION_BADGUYSCREEN: usize = ANIMATION_START + 260;

const OBJECT_START: usize = ANIMATION_START + 6 * 48;
const OBJECT_BOX_GREY: usize = OBJECT_START;
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
const OBJECT_HOSTILESHOT: usize = OBJECT_START + 39;
const OBJECT_GUN: usize = OBJECT_START + 43;
const OBJECT_CHICKEN_SINGLE: usize = OBJECT_START + 44;
const OBJECT_CHICKEN_DOUBLE: usize = OBJECT_START + 45;
const OBJECT_ELECTRIC_ARC: usize = OBJECT_START + 50;
const OBJECT_ELECTRIC_ARC_HURTING: usize = OBJECT_START + 54;
const OBJECT_FOOTBALL: usize = OBJECT_START + 58;
const OBJECT_JOYSTICK: usize = OBJECT_START + 59;
const OBJECT_DISK: usize = OBJECT_START + 60;
const OBJECT_HEALTH: usize = OBJECT_START + 61;
const OBJECT_NONHEALTH: usize = OBJECT_START + 62;
const OBJECT_GLOVE: usize = OBJECT_START + 63;
const OBJECT_LASERBEAM: usize = OBJECT_START + 65;
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
const OBJECT_NOTEBOOK: usize = OBJECT_START + 123;
const OBJECT_KEY_RED: usize = OBJECT_START + 124;
const OBJECT_KEY_GREEN: usize = OBJECT_START + 125;
const OBJECT_KEY_BLUE: usize = OBJECT_START + 126;
const OBJECT_KEY_PINK: usize = OBJECT_START + 127;
const OBJECT_DOOR: usize = OBJECT_START + 128;
const OBJECT_KEYHOLE_BLACK: usize = OBJECT_START + 136;
const OBJECT_KEYHOLE_RED: usize = OBJECT_START + 137;
const OBJECT_KEYHOLE_GREEN: usize = OBJECT_START + 138;
const OBJECT_KEYHOLE_BLUE: usize = OBJECT_START + 139;
const OBJECT_KEYHOLE_PINK: usize = OBJECT_START + 140;
const OBJECT_SPIKES_UP: usize = OBJECT_START + 148;
const OBJECT_SPIKES_DOWN: usize = OBJECT_START + 149;

const HERO_START: usize = OBJECT_START + 150;

const HERO_NUM_WALKING: usize = 4;
const HERO_WALKING_LEFT: usize = HERO_START;
const HERO_WALKING_RIGHT: usize = HERO_START + 0x10;
const HERO_NUM_JUMPING: usize = 1;
const HERO_JUMPING_LEFT: usize = HERO_START + 0x20;
const HERO_JUMPING_RIGHT: usize = HERO_START + 0x24;
const HERO_NUM_FALLING: usize = 1;
const HERO_FALLING_LEFT: usize = HERO_START + 0x28;
const HERO_FALLING_RIGHT: usize = HERO_START + 0x2C;
const HERO_NUM_STANDING: usize = 1;
const HERO_STANDING_LEFT: usize = HERO_START + 0x30;
const HERO_STANDING_RIGHT: usize = HERO_START + 0x34;
const HERO_JUMPING_LEFT_SOMERSAULT: usize = HERO_START + 0x38;
const HERO_JUMPING_RIGHT_SOMERSAULT: usize = HERO_START + 0x54;
const HERO_SKELETON_LEFT: usize = HERO_START + 0xB0;
const HERO_SKELETON_RIGHT: usize = HERO_START + 0xB4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyColor {
    Red,
    Blue,
    Pink,
    Green,
}

impl std::string::ToString for KeyColor {
    fn to_string(&self) -> String {
        match *self {
            KeyColor::Red => "red",
            KeyColor::Blue => "blue",
            KeyColor::Pink => "pink",
            KeyColor::Green => "green",
        }
        .to_string()
    }
}

fn directories() -> directories_next::ProjectDirs {
    directories_next::ProjectDirs::from("", "", "freenukum").unwrap()
}

fn config_dir() -> std::path::PathBuf {
    directories().config_dir().to_path_buf()
}

pub fn data_dir() -> std::path::PathBuf {
    directories().data_dir().to_path_buf()
}

fn collision_bounds_color() -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(182, 6, 0)
}

#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum HorizontalDirection {
    Left,
    Right,
}

impl HorizontalDirection {
    pub fn opposite(self) -> Self {
        match self {
            HorizontalDirection::Left => HorizontalDirection::Right,
            HorizontalDirection::Right => HorizontalDirection::Left,
        }
    }

    pub fn reverse(&mut self) {
        *self = self.opposite();
    }

    pub fn as_factor_i32(&self) -> i32 {
        match self {
            HorizontalDirection::Left => -1,
            HorizontalDirection::Right => 1,
        }
    }

    pub fn map<T>(&self, left: T, right: T) -> T {
        match self {
            HorizontalDirection::Left => left,
            HorizontalDirection::Right => right,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum VerticalDirection {
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UserEvent {
    Timer,
    Redraw,
}

pub trait Sizes {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn half_width(&self) -> u32;
    fn half_height(&self) -> u32;
}

pub struct DefaultSizes;

impl Sizes for DefaultSizes {
    fn width(&self) -> u32 {
        TILE_WIDTH
    }
    fn height(&self) -> u32 {
        TILE_HEIGHT
    }
    fn half_width(&self) -> u32 {
        HALFTILE_WIDTH
    }
    fn half_height(&self) -> u32 {
        HALFTILE_HEIGHT
    }
}
