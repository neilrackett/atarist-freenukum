#[macro_use]
extern crate serde_derive;

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

pub use game::Game;
pub use settings::Settings;

pub mod transition;

pub use transition::geometry::Geometry;
