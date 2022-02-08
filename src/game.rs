// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::actor::{
    ActorExt, ActorMessageQueue, ActorType, ActorsList, ParticleColor,
};
use crate::borders::Borders;
use crate::data::original_data_dir;
use crate::episodes::Episodes;
use crate::event::{ConfirmEvent, GameEvent, InputContext, WaitEvent};
use crate::hero::{Hero, Motion};
use crate::infobox::{self, InfoMessageQueue};
use crate::level::{BackgroundTileStrategy, Level, LevelTiles, PlayState};
use crate::picture::show_splash_with_message;
use crate::rendering::{CanvasRenderer, MovePositionRenderer};
use crate::settings::Settings;
use crate::sound::{SoundCache, SoundIndex, SoundPlayer};
use crate::tile::TileHeader;
use crate::{backdrop, HorizontalDirection, TileProvider, UserEvent};
use crate::{
    Result, Sizes, GAME_INTERVAL, LEVELWINDOW_HEIGHT, LEVELWINDOW_WIDTH,
    LEVEL_HEIGHT, LEVEL_WIDTH,
};

use anyhow::Error;
use sdl2::{
    event::EventSender,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, RenderTarget, TextureCreator, WindowCanvas},
    ttf::Font,
    video::Window,
    AudioSubsystem, EventPump, TimerSubsystem, VideoSubsystem,
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
    soundcache: &SoundCache,
    hero: &mut Hero,
    settings: &mut Settings,
    episodes: &Episodes,
    borders: &Borders,
    event_pump: &mut EventPump,
    event_sender: &EventSender,
    timer_subsystem: &TimerSubsystem,
    audio_subsystem: &AudioSubsystem,
    sizes: &dyn Sizes,
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

    let mut level = {
        let filename = format!(
            "worldal{:x}.{}",
            level_number,
            episodes.file_extension()
        );
        let filepath = original_data_dir().join(filename);
        let mut file = File::open(filepath)?;
        Level::load(&mut file, hero, sizes)?
    };

    let destrect = Rect::new(
        sizes.width() as i32,
        sizes.height() as i32,
        (LEVELWINDOW_WIDTH) * sizes.width(),
        (LEVELWINDOW_HEIGHT) * sizes.height(),
    );
    let heropos = hero.position.geometry;
    let mut srcrect = Rect::new(
        (heropos.x() as u32 + sizes.width())
            .saturating_sub(destrect.width() / 2) as i32,
        (heropos.y() as u32).saturating_sub(destrect.height() / 2) as i32,
        LEVELWINDOW_WIDTH * sizes.width(),
        LEVELWINDOW_HEIGHT * sizes.height(),
    );

    let timer = timer_subsystem.add_timer(
        GAME_INTERVAL,
        Box::new(move || {
            event_sender.push_custom_event(UserEvent::Timer).unwrap();
            GAME_INTERVAL
        }),
    );

    let mut sound_player =
        SoundPlayer::create(audio_subsystem, soundcache);

    let mut command_queue = GameCommandQueue::new();
    let mut info_message_queue = InfoMessageQueue::new();
    let mut actor_message_queue = ActorMessageQueue::new();

    let mut do_update = true;
    let mut walking_left = BTreeSet::new();
    let mut walking_right = BTreeSet::new();

    if level_number == 1 {
        command_queue.add_sound(SoundIndex::STARTGAME);
    }

    while level.play_state.keep_acting() {
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

            match level.play_state {
                PlayState::KilledPlayingAnimation(0) => {
                    level.play_state = PlayState::RestartLevel;
                    info_message_queue.push_back(
                        "You died.\nRestarting level.".to_string(),
                    );
                }
                PlayState::KilledPlayingAnimation(i) => {
                    hero.hidden = true;

                    if i > 30 && i % 2 == 0 {
                        command_queue
                            .add_particle_firework(heropos.center(), 4);
                    }
                    level.play_state =
                        PlayState::KilledPlayingAnimation(i - 1);
                }
                _ => {}
            }

            srcrect.x = std::cmp::min(
                (heropos.center().x as u32)
                    .saturating_sub(LEVELWINDOW_WIDTH * sizes.width() / 2),
                LEVEL_WIDTH * sizes.height() - srcrect.width(),
            ) as i32;
            srcrect.y = std::cmp::min(
                (heropos.y() as u32).saturating_sub(
                    LEVELWINDOW_HEIGHT * sizes.height() / 2,
                ),
                LEVEL_HEIGHT * sizes.height() - srcrect.height(),
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
                offset_x: -srcrect.x() + sizes.width() as i32,
                offset_y: -srcrect.y() + sizes.height() as i32,
                upstream: &mut renderer,
            };

            level.render(
                &mut level_renderer,
                sizes,
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

        match GameEvent::wait(event_pump)? {
            GameEvent::Escape => {
                level.play_state = PlayState::GoToMainScreen;
            }
            GameEvent::GetInventoryItem(item) => {
                hero.inventory.set(item);
            }
            GameEvent::IncreaseLife => {
                hero.firepower.increase(1);
            }
            GameEvent::FinishLevel => {
                level.play_state = PlayState::LevelFinished
            }
            GameEvent::ToggleFullscreen => {
                use sdl2::video::FullscreenType;
                match canvas.window().fullscreen_state() {
                    FullscreenType::Off => {
                        settings.fullscreen = true;
                        canvas
                            .window_mut()
                            .set_fullscreen(FullscreenType::Desktop)
                            .map_err(Error::msg)?;
                    }
                    FullscreenType::True | FullscreenType::Desktop => {
                        settings.fullscreen = false;
                        canvas
                            .window_mut()
                            .set_fullscreen(FullscreenType::Off)
                            .map_err(Error::msg)?;
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
                if srcrect.right() > (LEVEL_WIDTH * sizes.width()) as i32 {
                    srcrect
                        .set_right((LEVEL_WIDTH * sizes.width()) as i32);
                }
                if srcrect.bottom()
                    > (LEVEL_HEIGHT * sizes.height()) as i32
                {
                    srcrect.set_bottom(
                        (LEVEL_HEIGHT * sizes.height()) as i32,
                    );
                }
                do_update = true;
            }
            GameEvent::HeroInteractionStart => {
                level.hero_interact_start(
                    hero,
                    &mut command_queue,
                    &mut info_message_queue,
                    &mut actor_message_queue,
                );
                do_update = true;
            }
            GameEvent::HeroInteractionEnd => {
                level.hero_interact_end(hero);
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
                hero.jump(&mut command_queue);
                hero.update_animation();
            }
            GameEvent::HeroStartFiring => {
                hero.is_shooting = true;
                level.fire_shot(
                    sizes,
                    hero,
                    &mut command_queue,
                    &mut actor_message_queue,
                );
                hero.update_animation();
            }
            GameEvent::HeroStopFiring => {
                hero.is_shooting = false;
                hero.update_animation();
            }
            GameEvent::TimerTriggered => {
                let mut play_sounds = Vec::new();
                level.act(
                    sizes,
                    hero,
                    &mut command_queue,
                    &mut actor_message_queue,
                    &mut play_sounds,
                    srcrect,
                )?;
                for index in play_sounds {
                    sound_player.play_sound(index);
                }
                do_update = true;
                info_message_queue.process(
                    canvas,
                    tileprovider,
                    event_pump,
                )?;
            }
        }
    }
    drop(timer);

    let next_action = match level.play_state {
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
    soundcache: &SoundCache,
    hero: &mut Hero,
    settings: &mut Settings,
    episodes: &Episodes,
    event_pump: &mut EventPump,
    event_sender: &EventSender,
    timer_subsystem: &TimerSubsystem,
    audio_subsystem: &AudioSubsystem,
    sizes: &dyn Sizes,
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

    hero.reset(sizes);

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
                soundcache,
                hero,
                settings,
                episodes,
                &borders,
                event_pump,
                event_sender,
                timer_subsystem,
                audio_subsystem,
                sizes,
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
                soundcache,
                hero,
                settings,
                episodes,
                &borders,
                event_pump,
                event_sender,
                timer_subsystem,
                audio_subsystem,
                sizes,
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

pub struct ActorQueueItem {
    pub actor_type: ActorType,
    pub pos: Point,
}

pub trait GameCommands {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point);

    fn add_sound(&mut self, index: SoundIndex);

    fn add_particle_firework(&mut self, pos: Point, count: usize) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let color = match rng.gen_range(0..4) {
                0 => ParticleColor::Pink,
                1 => ParticleColor::Blue,
                2 => ParticleColor::White,
                3 => ParticleColor::Green,
                _ => unreachable!(),
            };
            self.add_actor(ActorType::Particle(color), pos);
        }
    }
}

#[derive(Default)]
pub struct GameCommandQueue {
    pub actors: Vec<ActorQueueItem>,
    pub sounds: Vec<SoundIndex>,
}

impl GameCommands for GameCommandQueue {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point) {
        self.push_back_actor(actor_type, pos);
    }

    fn add_sound(&mut self, index: SoundIndex) {
        self.push_back_sound(index);
    }
}

impl GameCommandQueue {
    pub fn new() -> Self {
        Self {
            actors: Vec::new(),
            sounds: Vec::new(),
        }
    }

    pub fn push_back_actor(&mut self, actor_type: ActorType, pos: Point) {
        self.actors.push(ActorQueueItem { actor_type, pos });
    }

    pub fn push_back_sound(&mut self, index: SoundIndex) {
        self.sounds.push(index);
    }

    pub(crate) fn process(&mut self, destination: &mut dyn GameCommands) {
        for ActorQueueItem { actor_type, pos } in self.actors.drain(..) {
            destination.add_actor(actor_type, pos);
        }
        for index in self.sounds.drain(..) {
            destination.add_sound(index);
        }
    }
}

pub struct LevelCommandProcessor<'a> {
    pub sizes: &'a dyn Sizes,
    pub tiles: &'a mut LevelTiles,
    pub actors: &'a mut ActorsList,
    pub play_sounds: &'a mut Vec<SoundIndex>,
    pub copy_background: bool,
}

impl<'a> GameCommands for LevelCommandProcessor<'a> {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point) {
        let actor =
            actor_type.create_actor_boxed(pos, self.sizes, self.tiles);
        if self.copy_background {
            let x = pos.x / self.sizes.width() as i32;
            let y = pos.y / self.sizes.height() as i32;

            let effective_number = match actor.background_tile_strategy() {
                BackgroundTileStrategy::KeepEmpty => 0,
                BackgroundTileStrategy::SetTile(tile) => tile,
                BackgroundTileStrategy::CopyFromAbove => self
                    .tiles
                    .get(x, y - 1)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
                BackgroundTileStrategy::CopyFromLeft => self
                    .tiles
                    .get(x - 1, y)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
                BackgroundTileStrategy::CopyFromRight => self
                    .tiles
                    .get(x + 1, y)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
                BackgroundTileStrategy::CopyFromBelow => self
                    .tiles
                    .get(x, y + 1)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
            };

            if let Ok(t) = self.tiles.get_mut(x, y) {
                t.effective_number = effective_number;
            }
        }
        self.actors.actors.push(actor);
    }

    fn add_sound(&mut self, index: SoundIndex) {
        self.play_sounds.push(index);
    }
}
