use anyhow::{anyhow, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::load_default_font;
use freenukum::hero::Hero;
use freenukum::level::raw::LevelRaw;
use freenukum::level::LevelData;
use freenukum::rendering::{CanvasRenderer, MovePositionRenderer};
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::UserEvent;
use freenukum::{
    game, DefaultSizes, Sizes, LEVEL_HEIGHT, LEVEL_WIDTH, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
};
use std::fs::File;
use std::num::NonZeroUsize;
use std::num::ParseIntError;
use structopt::StructOpt;

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
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let args = Arguments::from_args();

    let settings = Settings::load_or_create();
    let sdl_context = sdl2::init().map_err(|s| anyhow!(s))?;
    let video_subsystem = sdl_context.video().map_err(|s| anyhow!(s))?;
    let ttf_context = sdl2::ttf::init()?;
    let event_subsystem = sdl_context.event().map_err(|s| anyhow!(s))?;
    let mut event_pump =
        sdl_context.event_pump().map_err(|s| anyhow!(s))?;
    event_subsystem
        .register_custom_event::<UserEvent>()
        .map_err(|s| anyhow!(s))?;

    let event_sender = event_subsystem.event_sender();

    let window = game::create_window(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        settings.fullscreen,
        &format!("Freenukum {} level loader example", VERSION),
        &video_subsystem,
    )?;
    let (win_w, win_h) = window.size();

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    let mut episodes = game::check_episodes(
        &mut canvas,
        &load_default_font(&ttf_context)?,
        &texture_creator,
        &mut event_pump,
    )?;

    let tilecache = TileCache::load_from_path(&original_data_dir())?;
    episodes.switch_to(args.episode.get() - 1)?;

    let level_file = format!(
        "worldal{:1x}.{}",
        args.level_number,
        episodes.file_extension()
    );

    let mut file = File::open(&original_data_dir().join(level_file))?;
    let sizes = DefaultSizes;

    let mut level_raw = LevelRaw::new();
    let mut hero = Hero::new(&sizes);
    let mut level_data = LevelData::load(
        &mut file,
        &mut hero,
        &mut Some(&mut level_raw),
        &sizes,
    )?;

    let mut r = Rect::new(0, 0, win_w, win_h);
    let level_rect = Rect::new(
        0,
        0,
        sizes.width() * LEVEL_WIDTH,
        sizes.height() * LEVEL_HEIGHT,
    );

    event_sender
        .push_custom_event(UserEvent::Redraw)
        .map_err(|s| anyhow!(s))?;

    let mut multiply = 10;
    'event_loop: loop {
        match event_pump.wait_event() {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                let (x, y) = (0, -1);
                scroll(x * multiply, y * multiply, &mut r, level_rect);
                event_sender
                    .push_custom_event(UserEvent::Redraw)
                    .map_err(|s| anyhow!(s))?;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                let (x, y) = (0, 1);
                scroll(x * multiply, y * multiply, &mut r, level_rect);
                event_sender
                    .push_custom_event(UserEvent::Redraw)
                    .map_err(|s| anyhow!(s))?;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                let (x, y) = (-1, 0);
                scroll(x * multiply, y * multiply, &mut r, level_rect);
                event_sender
                    .push_custom_event(UserEvent::Redraw)
                    .map_err(|s| anyhow!(s))?;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                let (x, y) = (1, 0);
                scroll(x * multiply, y * multiply, &mut r, level_rect);
                event_sender
                    .push_custom_event(UserEvent::Redraw)
                    .map_err(|s| anyhow!(s))?;
            }
            Event::KeyDown {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                multiply = 50;
            }
            Event::KeyUp {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                multiply = 10;
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let global_x = r.x() + x;
                let global_y = r.y() + y;
                let tile_x = global_x as u32 / sizes.width();
                let tile_y = global_y as u32 / sizes.height();

                let tilenr = level_raw.get(tile_x, tile_y);
                let is_solid = level_data.solids.get(tile_x, tile_y);

                println!(
                    "Tile at (x={}, y={}): 0x{:04x}. Solid: {}",
                    tile_x, tile_y, tilenr, is_solid
                );
            }
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => break 'event_loop,
            Event::Window {
                win_event: WindowEvent::Exposed,
                ..
            }
            | Event::Window {
                win_event: WindowEvent::Shown,
                ..
            } => canvas.present(),
            e if e.is_user_event() => {
                if e.as_user_event_type::<UserEvent>()
                    == Some(UserEvent::Redraw)
                {
                    let mut renderer = CanvasRenderer {
                        canvas: &mut canvas,
                        texture_creator: &texture_creator,
                        tileprovider: &tilecache,
                    };
                    let mut renderer = MovePositionRenderer {
                        offset_x: -r.x(),
                        offset_y: -r.y(),
                        upstream: &mut renderer,
                    };
                    level_data.render(
                        &mut renderer,
                        &sizes,
                        &mut hero,
                        settings.draw_collision_bounds,
                        r,
                        None,
                        None,
                    )?;
                }
                canvas.present()
            }
            _ => {}
        }
    }
    Ok(())
}

fn scroll(x_dist: i32, y_dist: i32, r: &mut Rect, level_rect: Rect) {
    r.offset(x_dist, y_dist);

    if r.left() < 0 {
        r.set_x(0);
    }
    if r.right() > level_rect.right() {
        r.set_right(level_rect.right());
    }

    if r.top() < 0 {
        r.set_y(0);
    }
    if r.bottom() > level_rect.bottom() {
        r.set_bottom(level_rect.bottom());
    }
}
