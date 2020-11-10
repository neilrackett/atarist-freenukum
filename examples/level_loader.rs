use anyhow::{anyhow, Result};
use freenukum::data::original_data_dir;
use freenukum::geometry::Geometry;
use freenukum::hero::HeroData;
use freenukum::level::raw::LevelRaw;
use freenukum::level::LevelData;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{
    game, sdl_surface_creation_params, BACKDROP_HEIGHT, BACKDROP_WIDTH,
    LEVEL_HEIGHT, LEVEL_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use std::fs::File;
use std::num::NonZeroUsize;
use std::num::ParseIntError;
use structopt::StructOpt;
use transdl::event::{Event, KeyCode, MouseButton};
use transdl::video::Surface;

/// Show an original Duke Nukem 1 level.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The number of the level in hexadecimal format.
    /// This is usually in the range from 1 to 'c'.
    #[structopt(parse(try_from_str=parse_hex))]
    level_number: usize,

    /// The episode number. Usually in the range 1 to 3.
    #[structopt(default_value = "1", long, name = "EPISODE_NUMBER")]
    episode: NonZeroUsize,
}

fn parse_hex(src: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(src, 16)
}

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let args = Arguments::from_args();

    let settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        (BACKDROP_WIDTH * TILE_WIDTH) as i32,
        (BACKDROP_HEIGHT * TILE_HEIGHT) as i32,
        settings.fullscreen,
        format!("Freenukum {} level loader example", VERSION),
        format!("Freenukum {} level loader example", VERSION),
    )?;
    let texture_creation_params = sdl_surface_creation_params(&screen);
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        texture_creation_params,
    )?;

    let mut episodes = game::check_episodes(&mut screen)?;
    episodes.switch_to(args.episode.get() - 1)?;

    let level_file = format!(
        "worldal{:1x}.{}",
        args.level_number,
        episodes.file_extension()
    );

    let mut file = File::open(&original_data_dir().join(level_file))?;

    let mut level_raw = LevelRaw::new();
    let mut hero = HeroData::new();
    let mut level_data = LevelData::load(
        &mut file,
        &mut hero,
        &tilecache,
        texture_creation_params,
        &mut Some(&mut level_raw),
    )?;

    let mut level_surface = texture_creation_params.create_surface(
        (TILE_WIDTH * LEVEL_WIDTH) as u16,
        (TILE_HEIGHT * LEVEL_HEIGHT) as u16,
    );
    let mut r = Geometry {
        x: 0,
        y: 0,
        w: (TILE_WIDTH * LEVEL_WIDTH) as u16,
        h: (TILE_HEIGHT * LEVEL_HEIGHT) as u16,
    };

    level_data.blit(
        &mut level_surface,
        &tilecache,
        &mut hero,
        settings.draw_collision_bounds,
        r,
        r,
        None,
        None,
    );
    level_surface.blit(Some(r.as_sdl_rect()), &mut screen, None);
    screen.update();

    let mut multiply = 10;
    'event_loop: loop {
        match Event::wait().map_err(|e| anyhow!("{}", e))? {
            Event::KeyDown {
                key: Some(KeyCode::Up),
                ..
            } => {
                let (x, y) = (0, -1);
                scroll(
                    x * multiply,
                    y * multiply,
                    &mut r,
                    &mut level_surface,
                    &mut screen,
                );
            }
            Event::KeyDown {
                key: Some(KeyCode::Down),
                ..
            } => {
                let (x, y) = (0, 1);
                scroll(
                    x * multiply,
                    y * multiply,
                    &mut r,
                    &mut level_surface,
                    &mut screen,
                );
            }
            Event::KeyDown {
                key: Some(KeyCode::Left),
                ..
            } => {
                let (x, y) = (-1, 0);
                scroll(
                    x * multiply,
                    y * multiply,
                    &mut r,
                    &mut level_surface,
                    &mut screen,
                );
            }
            Event::KeyDown {
                key: Some(KeyCode::Right),
                ..
            } => {
                let (x, y) = (1, 0);
                scroll(
                    x * multiply,
                    y * multiply,
                    &mut r,
                    &mut level_surface,
                    &mut screen,
                );
            }
            Event::KeyDown {
                key: Some(KeyCode::LeftShift),
                ..
            } => {
                multiply = 50;
            }
            Event::KeyUp {
                key: Some(KeyCode::LeftShift),
                ..
            } => {
                multiply = 10;
            }
            Event::MouseButtonDown {
                button: Some(MouseButton::Left),
                x,
                y,
            } => {
                let global_x = r.x as usize + x as usize;
                let global_y = r.y as usize + y as usize;
                let tile_x = global_x / TILE_WIDTH;
                let tile_y = global_y / TILE_HEIGHT;

                let tilenr = level_raw.get(tile_x, tile_y);
                let is_solid = level_data.solids.get(tile_x, tile_y);

                println!(
                    "Tile at (x={}, y={}): 0x{:04x}. Solid: {}",
                    tile_x, tile_y, tilenr, is_solid
                );
            }
            Event::Quit
            | Event::KeyDown {
                key: Some(KeyCode::Escape),
                ..
            }
            | Event::KeyDown {
                key: Some(KeyCode::Q),
                ..
            } => break 'event_loop,
            Event::VideoExpose => screen.update(),
            _ => {}
        }
    }
    Ok(())
}

fn scroll(
    x_dist: i16,
    y_dist: i16,
    r: &mut Geometry,
    level_surface: &mut Surface,
    screen: &mut Surface,
) {
    r.x += x_dist;
    r.y += y_dist;

    if r.x < 0 {
        r.x = 0;
    }
    if r.x as usize + screen.width() as usize > level_surface.width() {
        r.x = level_surface.width() as i16 - screen.width() as i16;
    }

    if r.y < 0 {
        r.y = 0;
    }
    if r.y as usize + screen.height() as usize > level_surface.height() {
        r.y = level_surface.height() as i16 - screen.height() as i16;
    }

    level_surface.blit(Some(r.as_sdl_rect()), screen, None);
    screen.update();
}
