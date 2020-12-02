use anyhow::{anyhow, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::SurfaceCreatorProvider;
use freenukum::hero::{HeroData, Motion};
use freenukum::level::solids::LevelSolids;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::HorizontalDirection;
use freenukum::UserEvent;
use freenukum::{
    game, BACKDROP_HEIGHT, BACKDROP_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use std::collections::HashSet;
use transdl::event::{Event, KeyCode, MouseButton};
use transdl::timer::Timer;

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        (BACKDROP_WIDTH * TILE_WIDTH) as i32,
        (BACKDROP_HEIGHT * TILE_HEIGHT) as i32,
        settings.fullscreen,
        format!("Freenukum {} hero example", VERSION),
        format!("Freenukum {} hero example", VERSION),
    )?;

    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        &screen.surface_creator(),
    )?;

    let mut hero = HeroData::new();

    hero.position.geometry.x =
        (screen.width() as i16 - hero.position.geometry.w as i16) / 2;
    hero.position.geometry.y =
        (screen.height() as i16 - hero.position.geometry.h as i16) / 2;

    let solids = LevelSolids::new_all_solid();
    hero.blit(
        &mut screen,
        &tilecache,
        &solids,
        settings.draw_collision_bounds,
    );
    screen.update();

    let mut directions = HashSet::new();

    const GAME_INTERVAL: u32 = 80;
    let timer = Timer::add(GAME_INTERVAL, || {
        transdl::event::push_user_event(UserEvent::Timer as i32);
    });

    'event_loop: loop {
        match Event::wait().map_err(|e| anyhow!("{}", e))? {
            Event::UserEvent { code }
                if code == UserEvent::Timer as i32 =>
            {
                screen.fill(0);
                hero.next_frame();
                hero.update_animation();
                hero.act(&solids);
                hero.blit(
                    &mut screen,
                    &tilecache,
                    &solids,
                    settings.draw_collision_bounds,
                );
                screen.update();
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
            Event::KeyDown {
                key: Some(KeyCode::Right),
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
                key: Some(KeyCode::Left),
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
                key: Some(KeyCode::LeftAlt),
                ..
            }
            | Event::MouseButtonDown {
                button: Some(MouseButton::Left),
                ..
            } => {
                hero.is_shooting = true;
                hero.update_animation();
            }
            Event::KeyUp {
                key: Some(KeyCode::LeftAlt),
                ..
            }
            | Event::MouseButtonUp {
                button: Some(MouseButton::Left),
                ..
            } => {
                hero.is_shooting = false;
                hero.update_animation();
            }
            Event::KeyDown {
                key: Some(KeyCode::LeftCtrl),
                ..
            }
            | Event::MouseButtonDown {
                button: Some(MouseButton::Right),
                ..
            } => {
                hero.jump();
                hero.update_animation();
            }
            Event::KeyUp {
                key: Some(KeyCode::LeftCtrl),
                ..
            }
            | Event::MouseButtonUp {
                button: Some(MouseButton::Right),
                ..
            } => {
                hero.land();
                hero.update_animation();
            }
            Event::KeyUp {
                key: Some(KeyCode::Right),
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
                key: Some(KeyCode::Left),
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
            Event::VideoExpose => screen.update(),
            _ => {}
        }
    }
    timer.remove();
    Ok(())
}
