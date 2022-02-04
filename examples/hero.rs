use anyhow::{Error, Result};
use freenukum::{
    data::original_data_dir,
    game,
    graphics::load_default_font,
    hero::{Hero, Motion},
    level::tiles::LevelTiles,
    rendering::{CanvasRenderer, Renderer},
    settings::Settings,
    tilecache::TileCache,
    DefaultSizes, HorizontalDirection, Sizes, UserEvent, BACKDROP_HEIGHT,
    BACKDROP_WIDTH, GAME_INTERVAL,
};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
};
use std::collections::HashSet;

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let settings = Settings::load_or_create();
    let sdl_context = sdl2::init().map_err(Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(Error::msg)?;
    let ttf_context = sdl2::ttf::init()?;
    let event_subsystem = sdl_context.event().map_err(Error::msg)?;
    let timer_subsystem = sdl_context.timer().map_err(Error::msg)?;
    let mut event_pump = sdl_context.event_pump().map_err(Error::msg)?;

    event_subsystem
        .register_custom_event::<UserEvent>()
        .map_err(Error::msg)?;

    let sizes = DefaultSizes;

    let win_width = BACKDROP_WIDTH * sizes.width();
    let win_height = BACKDROP_HEIGHT * sizes.height();
    let window = game::create_window(
        win_width,
        win_height,
        settings.fullscreen,
        &format!("Freenukum {} hero example", VERSION),
        &video_subsystem,
    )?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();

    game::check_episodes(
        &mut canvas,
        &load_default_font(&ttf_context)?,
        &texture_creator,
        &mut event_pump,
    )?;
    let tilecache = TileCache::load_from_path(&original_data_dir())?;

    let mut hero = Hero::new(&sizes);

    hero.position.geometry.x =
        (win_width as i32 - hero.position.geometry.width() as i32) / 2;
    hero.position.geometry.y =
        (win_height as i32 - hero.position.geometry.height() as i32) / 2;

    let mut renderer = CanvasRenderer {
        canvas: &mut canvas,
        texture_creator: &texture_creator,
        tileprovider: &tilecache,
    };

    let tiles = LevelTiles::new_all_solid();
    hero.render(
        &mut renderer,
        &sizes,
        &tiles,
        settings.draw_collision_bounds,
    )?;
    renderer.canvas.present();

    let mut directions = HashSet::new();

    let event_sender = event_subsystem.event_sender();

    let timer = timer_subsystem.add_timer(
        GAME_INTERVAL,
        Box::new(move || {
            event_sender.push_custom_event(UserEvent::Timer).unwrap();
            GAME_INTERVAL
        }),
    );

    let mut game_commands = game::GameCommandQueue::new();

    'event_loop: loop {
        match event_pump.wait_event() {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => break 'event_loop,
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                directions.insert(HorizontalDirection::Right);
                if directions.contains(&HorizontalDirection::Left) {
                    hero.motion = Motion::NotMoving;
                } else {
                    hero.motion = Motion::Walking;
                    hero.direction = HorizontalDirection::Right;
                }
                hero.update_animation();
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                directions.insert(HorizontalDirection::Left);
                if directions.contains(&HorizontalDirection::Right) {
                    hero.motion = Motion::NotMoving;
                } else {
                    hero.motion = Motion::Walking;
                    hero.direction = HorizontalDirection::Left;
                }
                hero.update_animation();
            }
            Event::KeyDown {
                keycode: Some(Keycode::LAlt),
                ..
            }
            | Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                hero.is_shooting = true;
                hero.update_animation();
            }
            Event::KeyUp {
                keycode: Some(Keycode::LAlt),
                ..
            }
            | Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                hero.is_shooting = false;
                hero.update_animation();
            }
            Event::KeyDown {
                keycode: Some(Keycode::LCtrl),
                ..
            }
            | Event::MouseButtonDown {
                mouse_btn: MouseButton::Right,
                ..
            } => {
                hero.jump(&mut game_commands);
                hero.update_animation();
            }
            Event::KeyUp {
                keycode: Some(Keycode::LCtrl),
                ..
            }
            | Event::MouseButtonUp {
                mouse_btn: MouseButton::Right,
                ..
            } => {
                hero.land();
                hero.update_animation();
            }
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => {
                directions.remove(&HorizontalDirection::Right);
                if directions.contains(&HorizontalDirection::Left) {
                    hero.motion = Motion::Walking;
                    hero.direction = HorizontalDirection::Left;
                } else {
                    hero.motion = Motion::NotMoving;
                }
                hero.update_animation();
            }
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => {
                directions.remove(&HorizontalDirection::Left);
                if directions.contains(&HorizontalDirection::Right) {
                    hero.motion = Motion::Walking;
                    hero.direction = HorizontalDirection::Right;
                } else {
                    hero.motion = Motion::NotMoving;
                }
                hero.update_animation();
            }
            e if e.is_user_event() => {
                if e.as_user_event_type::<UserEvent>()
                    == Some(UserEvent::Timer)
                {
                    renderer.fill(Color::RGB(0, 0, 0))?;
                    hero.next_frame();
                    hero.update_animation();
                    hero.act(&sizes, &tiles, &mut game_commands)?;
                    hero.render(
                        &mut renderer,
                        &sizes,
                        &tiles,
                        settings.draw_collision_bounds,
                    )?;
                    renderer.canvas.present();
                }
            }
            Event::Window {
                win_event: WindowEvent::Exposed,
                ..
            }
            | Event::Window {
                win_event: WindowEvent::Shown,
                ..
            } => {
                renderer.canvas.present();
            }
            _ => {}
        }
    }
    drop(timer);
    Ok(())
}
