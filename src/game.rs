use crate::actor::{ActorMessageQueue, ActorQueue};
use crate::borders::Borders;
use crate::data::original_data_dir;
use crate::episodes::Episodes;
use crate::event::{ConfirmEvent, GameEvent, InputContext, WaitEvent};
use crate::hero::{HeroData, Motion};
use crate::infobox::{self, InfoMessageQueue};
use crate::level::{LevelData, PlayState};
use crate::picture::show_splash_with_message;
use crate::rendering::{CanvasRenderer, MovePositionRenderer};
use crate::settings::Settings;
use crate::tile::TileHeader;
use crate::{backdrop, HorizontalDirection, TileProvider, UserEvent};
use crate::{
    Result, GAME_INTERVAL, LEVELWINDOW_HEIGHT, LEVELWINDOW_WIDTH,
    LEVEL_HEIGHT, LEVEL_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};

use anyhow::anyhow;
use sdl2::{
    event::EventSender,
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget, TextureCreator, WindowCanvas},
    ttf::Font,
    video::Window,
    EventPump, TimerSubsystem, VideoSubsystem,
};
use std::collections::BTreeSet;
use std::fs::File;

#[derive(PartialEq, Eq)]
enum NextAction {
    RestartLevel,
    NextLevel,
    GoToMainScreen,
}

#[allow(clippy::too_many_arguments)]
fn start_in_level(
    level_number: usize,
    canvas: &mut WindowCanvas,
    tileprovider: &dyn TileProvider,
    hero: &mut HeroData,
    settings: &mut Settings,
    episodes: &Episodes,
    borders: &Borders,
    event_pump: &mut EventPump,
    event_sender: &EventSender,
    timer_subsystem: &TimerSubsystem,
) -> Result<NextAction> {
    let backdrop = {
        let backdrop_number = match level_number {
            1 | 3 => 0,
            4 => 7,
            5 | 8 => 3,
            6 => 2,
            7 | 10 => 1,
            9 => 5,
            _ => 1,
        };

        let filename = format!(
            "drop{}.{}",
            backdrop_number,
            episodes.file_extension()
        );
        let filepath = original_data_dir().join(filename);
        let mut file = File::open(filepath)?;
        TileHeader::load_from(&mut file)?;
        backdrop::load(&mut file)?
    };

    let mut level_data = {
        let filename = format!(
            "worldal{:x}.{}",
            level_number,
            episodes.file_extension()
        );
        let filepath = original_data_dir().join(filename);
        let mut file = File::open(filepath)?;
        LevelData::load(&mut file, hero, &mut None)?
    };

    let destrect = Rect::new(
        TILE_WIDTH as i32,
        TILE_HEIGHT as i32,
        (LEVELWINDOW_WIDTH) * TILE_WIDTH,
        (LEVELWINDOW_HEIGHT) * TILE_HEIGHT,
    );
    let heropos = hero.position.geometry;
    let mut srcrect = Rect::new(
        (heropos.x() as u32 + TILE_WIDTH)
            .saturating_sub(destrect.width() / 2) as i32,
        (heropos.y() as u32).saturating_sub(destrect.height() / 2) as i32,
        LEVELWINDOW_WIDTH * TILE_WIDTH,
        LEVELWINDOW_HEIGHT * TILE_HEIGHT,
    );

    let timer = timer_subsystem.add_timer(
        GAME_INTERVAL,
        Box::new(move || {
            event_sender.push_custom_event(UserEvent::Timer).unwrap();
            GAME_INTERVAL
        }),
    );

    let mut actor_queue = ActorQueue::new();
    let mut info_message_queue = InfoMessageQueue::new();
    let mut actor_message_queue = ActorMessageQueue::new();

    let mut do_update = true;
    let mut walking_left = BTreeSet::new();
    let mut walking_right = BTreeSet::new();

    while level_data.play_state.keep_acting() {
        let texture_creator = canvas.texture_creator();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let mut renderer = CanvasRenderer {
            canvas,
            texture_creator: &texture_creator,
            tileprovider,
        };

        if do_update {
            let heropos = hero.position.geometry;

            match level_data.play_state {
                PlayState::KilledPlayingAnimation(0) => {
                    level_data.play_state = PlayState::RestartLevel;
                    info_message_queue.push_back(
                        "You died.\nRestarting level.".to_string(),
                    );
                }
                PlayState::KilledPlayingAnimation(i) => {
                    hero.hidden = true;

                    if i > 30 && i % 2 == 0 {
                        use crate::actor::ActorAdder;
                        actor_queue
                            .add_particle_firework(heropos.center(), 4);
                    }
                    level_data.play_state =
                        PlayState::KilledPlayingAnimation(i - 1);
                }
                _ => {}
            }

            srcrect.x = std::cmp::min(
                (heropos.center().x as u32)
                    .saturating_sub(LEVELWINDOW_WIDTH * TILE_WIDTH / 2),
                LEVEL_WIDTH * TILE_WIDTH - srcrect.width(),
            ) as i32;
            srcrect.y = std::cmp::min(
                (heropos.y() as u32)
                    .saturating_sub(LEVELWINDOW_HEIGHT * TILE_HEIGHT / 2),
                LEVEL_HEIGHT * TILE_HEIGHT - srcrect.height(),
            ) as i32;

            borders.render(&mut renderer)?;
            borders.render_life(
                hero.health.life().unwrap_or(0),
                &mut renderer,
            )?;
            borders.render_firepower(&hero.firepower, &mut renderer)?;
            borders.render_inventory(&hero.inventory, &mut renderer)?;
            borders.render_score(hero.score.value(), &mut renderer)?;

            renderer.canvas.set_clip_rect(destrect);
            let mut level_renderer = MovePositionRenderer {
                offset_x: -srcrect.x() + TILE_WIDTH as i32,
                offset_y: -srcrect.y() + TILE_HEIGHT as i32,
                upstream: &mut renderer,
            };

            level_data.render(
                &mut level_renderer,
                hero,
                settings.draw_collision_bounds,
                srcrect,
                Some(&backdrop),
                None,
            )?;
            canvas.set_clip_rect(None);
            canvas.present();
            do_update = false;
        }

        info_message_queue.process(canvas, tileprovider, event_pump)?;

        match GameEvent::wait(event_pump)? {
            GameEvent::Escape => {
                level_data.play_state = PlayState::GoToMainScreen;
            }
            GameEvent::GetInventoryItem(item) => {
                hero.inventory.set(item);
            }
            GameEvent::IncreaseLife => {
                hero.firepower.increase(1);
            }
            GameEvent::FinishLevel => {
                level_data.play_state = PlayState::LevelFinished
            }
            GameEvent::ToggleFullscreen => {
                use sdl2::video::FullscreenType;
                match canvas.window().fullscreen_state() {
                    FullscreenType::Off => {
                        settings.fullscreen = true;
                        canvas
                            .window_mut()
                            .set_fullscreen(FullscreenType::Desktop)
                            .map_err(|s| anyhow!(s))?;
                    }
                    FullscreenType::True | FullscreenType::Desktop => {
                        settings.fullscreen = false;
                        canvas
                            .window_mut()
                            .set_fullscreen(FullscreenType::Off)
                            .map_err(|s| anyhow!(s))?;
                    }
                }
                settings.save();
            }
            GameEvent::MoveViewPoint { x, y } => {
                srcrect.offset(x, y);

                if srcrect.x() < 0 {
                    srcrect.set_x(0);
                }
                if srcrect.y() < 0 {
                    srcrect.set_y(0);
                }
                if srcrect.right() > (LEVEL_WIDTH * TILE_WIDTH) as i32 {
                    srcrect.set_right((LEVEL_WIDTH * TILE_WIDTH) as i32);
                }
                if srcrect.bottom() > (LEVEL_HEIGHT * TILE_HEIGHT) as i32 {
                    srcrect
                        .set_bottom((LEVEL_HEIGHT * TILE_HEIGHT) as i32);
                }
                do_update = true;
            }
            GameEvent::HeroInteractionStart => {
                level_data.hero_interact_start(
                    hero,
                    &mut info_message_queue,
                    &mut actor_message_queue,
                );
                do_update = true;
            }
            GameEvent::HeroInteractionEnd => {
                level_data.hero_interact_end(hero);
                do_update = true;
            }
            GameEvent::HeroSetWalkingDirectionEnabled {
                directions,
                context,
                enabled,
            } => {
                for direction in directions {
                    match direction {
                        HorizontalDirection::Left => {
                            if enabled {
                                walking_left.insert(context);
                                if context == InputContext::ControllerAxis
                                {
                                    walking_right.remove(
                                        &InputContext::ControllerAxis,
                                    );
                                }
                            } else {
                                walking_left.remove(&context);
                            }
                        }
                        HorizontalDirection::Right => {
                            if enabled {
                                walking_right.insert(context);
                                if context == InputContext::ControllerAxis
                                {
                                    walking_left.remove(
                                        &InputContext::ControllerAxis,
                                    );
                                }
                            } else {
                                walking_right.remove(&context);
                            }
                        }
                    }
                }

                match (!walking_left.is_empty(), !walking_right.is_empty())
                {
                    (true, true) | (false, false) => {
                        hero.motion = Motion::NotMoving
                    }
                    (true, false) => {
                        hero.motion = Motion::Walking;
                        hero.direction = HorizontalDirection::Left;
                    }
                    (false, true) => {
                        hero.motion = Motion::Walking;
                        hero.direction = HorizontalDirection::Right;
                    }
                }
                hero.update_animation();
            }
            GameEvent::RefreshScreen => {
                canvas.present();
            }
            GameEvent::HeroJump => {
                hero.jump();
                hero.update_animation();
            }
            GameEvent::HeroStartFiring => {
                hero.is_shooting = true;
                level_data.fire_shot(
                    hero,
                    &mut actor_queue,
                    &mut actor_message_queue,
                );
                hero.update_animation();
            }
            GameEvent::HeroStopFiring => {
                hero.is_shooting = false;
                hero.update_animation();
            }
            GameEvent::TimerTriggered => {
                level_data.act(
                    hero,
                    &mut actor_queue,
                    &mut actor_message_queue,
                )?;
                do_update = true;
            }
        }
    }
    drop(timer);

    let next_action = match level_data.play_state {
        PlayState::LevelFinished => NextAction::NextLevel,
        PlayState::GoToMainScreen => NextAction::GoToMainScreen,
        PlayState::RestartLevel => NextAction::RestartLevel,
        _ => unreachable!(),
    };
    Ok(next_action)
}

#[allow(clippy::too_many_arguments)]
pub fn start(
    canvas: &mut WindowCanvas,
    tileprovider: &dyn TileProvider,
    hero: &mut HeroData,
    settings: &mut Settings,
    episodes: &Episodes,
    event_pump: &mut EventPump,
    event_sender: &EventSender,
    timer_subsystem: &TimerSubsystem,
) -> Result<()> {
    {
        let filename = format!("badguy.{}", episodes.file_extension());
        let filepath = original_data_dir().join(filename);
        let mut file = File::open(filepath)?;
        let message = "\
            So you're the pitiful\n\
            hero they sent to stop\n\
            me. I, Dr. Proton, will\n\
            soon rule the world!";
        show_splash_with_message(
            canvas,
            tileprovider,
            &mut file,
            event_pump,
            Some(message),
            0,
            144,
        )?;
    }
    {
        let filename = format!("duke.{}", episodes.file_extension());
        let filepath = original_data_dir().join(filename);
        let mut file = File::open(filepath)?;
        let message = "\
            You're wrong, Proton\n\
            breath. I'll be done\n\
            with you and still have\n\
            time to watch Oprah!";
        show_splash_with_message(
            canvas,
            tileprovider,
            &mut file,
            event_pump,
            Some(message),
            79,
            144,
        )?;
    }

    hero.reset();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let borders = Borders {};

    // start the game itself
    let mut level = 1;
    let mut interlevel = false;

    infobox::show(
        canvas,
        tileprovider,
        "Get ready FreeNukum,\nyou are going in.\n",
        event_pump,
    )?;
    let mut finished = false;
    let mut initial_lives = hero.health.life().unwrap();
    let mut initial_score = hero.score.value();
    let mut initial_inventory = hero.inventory.get_items();

    'level_loop: loop {
        if interlevel {
            match start_in_level(
                2,
                canvas,
                tileprovider,
                hero,
                settings,
                episodes,
                &borders,
                event_pump,
                event_sender,
                timer_subsystem,
            )? {
                NextAction::NextLevel => {
                    initial_lives = hero.health.life().unwrap();
                    initial_score = hero.score.value();
                    initial_inventory = hero.inventory.get_items();
                    level = if level == 1 { level + 2 } else { level + 1 };
                    interlevel = false;
                }
                NextAction::RestartLevel => unreachable!(),
                NextAction::GoToMainScreen => break 'level_loop,
            }
        } else {
            hero.reset_for_level();
            hero.health.set(initial_lives);
            hero.score.set_value(initial_score);
            hero.inventory.set_items(initial_inventory.clone());
            match start_in_level(
                level,
                canvas,
                tileprovider,
                hero,
                settings,
                episodes,
                &borders,
                event_pump,
                event_sender,
                timer_subsystem,
            )? {
                NextAction::NextLevel => {
                    if level == 13 {
                        finished = true;
                        break 'level_loop;
                    } else {
                        interlevel = true;
                    }
                }
                NextAction::RestartLevel => {
                    // Restart the level
                }
                NextAction::GoToMainScreen => break 'level_loop,
            }
        }
    }

    if finished {
        // TODO: the player finished, so we should show the end sequence
    }

    Ok(())
}

pub fn check_episodes<RT: RenderTarget, T>(
    target: &mut Canvas<RT>,
    font: &Font,
    texture_creator: &TextureCreator<T>,
    event_pump: &mut EventPump,
) -> Result<Episodes> {
    let episodes = Episodes::find_installed();
    if episodes.count() == 0 {
        show_missing_data_information(
            target,
            font,
            texture_creator,
            event_pump,
        )?;
    }
    Ok(episodes)
}

fn show_missing_data_information<RT: RenderTarget, T>(
    target: &mut Canvas<RT>,
    font: &Font,
    texture_creator: &TextureCreator<T>,
    event_pump: &mut EventPump,
) -> Result<()> {
    let msg = "Could not load data level and graphics files.\n\
    Please use the accompanied freenukum-data-tool\n\
    for installing the game data files";
    println!("{}", msg);

    super::data::display_text(target, 0, 0, font, msg, texture_creator)?;

    loop {
        match ConfirmEvent::wait(event_pump)? {
            ConfirmEvent::Confirmed | ConfirmEvent::Aborted => {
                return Ok(())
            }
            ConfirmEvent::RefreshScreen => {
                target.present();
            }
        }
    }
}

pub fn create_window(
    w: u32,
    h: u32,
    fullscreen: bool,
    title: &str,
    video_subsystem: &VideoSubsystem,
) -> Result<Window> {
    let mut builder = video_subsystem.window(title, w, h);
    if fullscreen {
        builder.fullscreen();
    }
    Ok(builder.build()?)
}
