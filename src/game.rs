use crate::actor::{ActorMessageQueue, ActorQueue, ActorType};
use crate::borders::Borders;
use crate::data::original_data_dir;
use crate::episodes::Episodes;
use crate::geometry::Geometry;
use crate::hero::{HeroData, InventoryItem, Motion};
use crate::infobox::{self, InfoMessageQueue};
use crate::level::LevelData;
use crate::picture::show_splash_with_message;
use crate::settings::Settings;
use crate::texture::TextureCreationParams;
use crate::tile::TileHeader;
use crate::tilecache::TileCache;
use crate::{backdrop, HorizontalDirection, UserEvent};
use crate::{
    Result, HALFTILE_HEIGHT, HALFTILE_WIDTH, LEVELWINDOW_HEIGHT,
    LEVELWINDOW_WIDTH, LEVEL_HEIGHT, LEVEL_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};

use anyhow::anyhow;
use std::collections::HashSet;
use std::fs::File;
use transdl::event::{Event, KeyCode, Modifier, MouseButton};
use transdl::timer::Timer;
use transdl::ttf::Font;
use transdl::video::Surface;

#[derive(PartialEq, Eq)]
enum Ending {
    Passed,
    Failed,
}

fn start_in_level(
    level_number: usize,
    tilecache: &TileCache,
    hero: &mut HeroData,
    texture_creation_params: TextureCreationParams,
    target: &mut Surface,
    settings: &mut Settings,
    episodes: &Episodes,
    borders: &Borders,
) -> Result<Ending> {
    let mut level_surface = texture_creation_params.create_surface(
        (TILE_WIDTH * LEVEL_WIDTH) as u16,
        (TILE_HEIGHT * LEVEL_HEIGHT) as u16,
    );

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
        backdrop::load(&mut file, texture_creation_params)?
    };

    let mut level_data = {
        let filename = format!(
            "worldal{:x}.{}",
            level_number,
            episodes.file_extension()
        );
        let filepath = original_data_dir().join(filename);
        let mut file = File::open(filepath)?;
        LevelData::load(
            &mut file,
            hero,
            tilecache,
            texture_creation_params,
            &mut None,
        )?
    };

    let destrect = Geometry {
        x: TILE_WIDTH as i16,
        y: TILE_HEIGHT as i16,
        w: ((LEVELWINDOW_WIDTH + 2) * TILE_WIDTH) as u16,
        h: ((LEVELWINDOW_HEIGHT + 2) * TILE_HEIGHT) as u16,
    };
    let heropos = hero.position.geometry;
    let mut srcrect = Geometry {
        x: (heropos.x as usize + TILE_WIDTH)
            .saturating_sub(destrect.w as usize / 2) as i16,
        y: (heropos.y as usize).saturating_sub(destrect.h as usize / 2)
            as i16,
        w: (LEVELWINDOW_WIDTH * TILE_WIDTH) as u16,
        h: (LEVELWINDOW_HEIGHT * TILE_HEIGHT) as u16,
    };

    const GAME_INTERVAL: u32 = 80;
    let timer = Timer::add(GAME_INTERVAL, || {
        transdl::event::push_user_event(UserEvent::Timer as i32);
    });

    // Make the first frame appear
    transdl::event::push_user_event(UserEvent::HeroMoved as i32);

    let mut actor_queue = ActorQueue::new();
    let mut info_message_queue = InfoMessageQueue::new();
    let mut actor_message_queue = ActorMessageQueue::new();

    let mut do_update = true;
    let mut update_whole_screen = true;
    let mut directions = HashSet::new();

    'game_loop: while level_data.do_play {
        if do_update {
            level_data.blit(
                &mut level_surface,
                tilecache,
                hero,
                settings.draw_collision_bounds,
                srcrect,
                srcrect,
                Some(&backdrop),
                None,
            );
            level_surface.blit(
                Some(srcrect.as_sdl_rect()),
                target,
                Some(destrect.as_sdl_rect()),
            );

            if update_whole_screen {
                target.update();
            } else {
                target.update_rect(
                    destrect.x as i32,
                    destrect.y as i32,
                    destrect.w as u32,
                    destrect.h as u32,
                );
            }
            do_update = false;
        }

        info_message_queue.process(
            target,
            tilecache,
            texture_creation_params,
        );

        match Event::wait().map_err(|e| anyhow!("{}", e))? {
            Event::Quit
            | Event::KeyDown {
                key: Some(KeyCode::Escape),
                ..
            }
            | Event::KeyDown {
                key: Some(KeyCode::Q),
                ..
            } => break 'game_loop,
            Event::KeyDown {
                key: Some(KeyCode::Num1),
                ..
            } => {
                hero.inventory.set(InventoryItem::KeyRed);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num2),
                ..
            } => {
                hero.inventory.set(InventoryItem::KeyGreen);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num3),
                ..
            } => {
                hero.inventory.set(InventoryItem::KeyBlue);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num4),
                ..
            } => {
                hero.inventory.set(InventoryItem::KeyPink);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num5),
                ..
            } => {
                hero.inventory.set(InventoryItem::Boot);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num6),
                ..
            } => {
                hero.inventory.set(InventoryItem::Glove);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num7),
                ..
            } => {
                hero.inventory.set(InventoryItem::Clamp);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num8),
                ..
            } => {
                hero.inventory.set(InventoryItem::AccessCard);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num9),
                ..
            } => {
                hero.firepower.increase(1);
                update_whole_screen = true;
            }
            Event::KeyDown {
                key: Some(KeyCode::Num0),
                ..
            } => {
                level_data.level_passed = true;
                level_data.do_play = false;
            }
            Event::KeyDown {
                key: Some(KeyCode::F),
                ..
            }
            | Event::KeyDown {
                key: Some(KeyCode::F11),
                ..
            } => {
                if target.toggle_fullscreen() {
                    settings.fullscreen = !settings.fullscreen;
                    settings.save();
                }
            }
            Event::KeyDown {
                key: Some(KeyCode::Down),
                modifiers,
            } if modifiers.contains(&Modifier::LeftShift)
                || modifiers.contains(&Modifier::RightShift) =>
            {
                if (srcrect.y as usize + srcrect.h as usize)
                    < LEVEL_HEIGHT * TILE_HEIGHT
                {
                    srcrect.y += HALFTILE_HEIGHT as i16;
                    do_update = true;
                }
            }
            Event::KeyDown {
                key: Some(KeyCode::Up),
                modifiers,
            } if modifiers.contains(&Modifier::LeftShift)
                || modifiers.contains(&Modifier::RightShift) =>
            {
                if (srcrect.y as usize + srcrect.h as usize) > 0 {
                    srcrect.y -= HALFTILE_HEIGHT as i16;
                    do_update = true;
                }
            }
            Event::KeyDown {
                key: Some(KeyCode::Right),
                modifiers,
            } if modifiers.contains(&Modifier::LeftShift)
                || modifiers.contains(&Modifier::RightShift) =>
            {
                if (srcrect.x as usize + srcrect.w as usize)
                    < LEVEL_WIDTH * TILE_WIDTH
                {
                    srcrect.x += HALFTILE_WIDTH as i16;
                    do_update = true;
                }
            }
            Event::KeyDown {
                key: Some(KeyCode::Left),
                modifiers,
            } if modifiers.contains(&Modifier::LeftShift)
                || modifiers.contains(&Modifier::RightShift) =>
            {
                if (srcrect.x as usize + srcrect.w as usize) > 0 {
                    srcrect.x -= HALFTILE_WIDTH as i16;
                    do_update = true;
                }
            }
            Event::KeyDown {
                key: Some(KeyCode::Up),
                ..
            }
            | Event::MouseButtonDown {
                button: Some(MouseButton::Middle),
                ..
            } => {
                level_data.hero_interact_start(
                    hero,
                    &mut info_message_queue,
                    &mut actor_message_queue,
                );
                do_update = true;
            }
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
            Event::KeyDown {
                key: Some(KeyCode::LeftAlt),
                ..
            }
            | Event::MouseButtonDown {
                button: Some(MouseButton::Left),
                ..
            } => {
                hero.is_shooting = true;
                level_data.fire_shot(hero, &mut actor_queue);
                hero.update_animation();
            }
            Event::KeyUp {
                key: Some(KeyCode::Up),
                ..
            }
            | Event::MouseButtonUp {
                button: Some(MouseButton::Middle),
                ..
            } => {
                level_data.hero_interact_end(hero);
                do_update = true;
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
            Event::VideoExpose => {
                target.update();
            }
            Event::UserEvent { code }
                if code == UserEvent::Timer as i32 =>
            {
                level_data.act(
                    hero,
                    &mut actor_queue,
                    &mut actor_message_queue,
                );
                do_update = true;
            }
            Event::UserEvent { code }
                if code == UserEvent::HeroMoved as i32 =>
            {
                let heropos = hero.position.geometry;
                srcrect.x = std::cmp::min(
                    (heropos.x as usize + heropos.w as usize / 2)
                        .saturating_sub(
                            LEVELWINDOW_WIDTH * TILE_WIDTH / 2,
                        ),
                    LEVEL_WIDTH * TILE_WIDTH - srcrect.w as usize,
                ) as i16;
                srcrect.y = std::cmp::min(
                    (heropos.y as usize).saturating_sub(
                        LEVELWINDOW_HEIGHT * TILE_HEIGHT / 2,
                    ),
                    LEVEL_HEIGHT * TILE_HEIGHT - srcrect.h as usize,
                ) as i16;
            }
            Event::UserEvent { code }
                if code == UserEvent::HeroScored as i32 =>
            {
                borders.blit_score(
                    target,
                    texture_creation_params,
                    tilecache,
                    hero.score.value(),
                );
                update_whole_screen = true;
            }
            Event::UserEvent { code }
                if code == UserEvent::HeroFirepowerChanged as i32 =>
            {
                borders.blit_firepower(
                    target,
                    texture_creation_params,
                    tilecache,
                    &hero.firepower,
                );
                update_whole_screen = true;
            }
            Event::UserEvent { code }
                if code == UserEvent::HeroInventoryChanged as i32 =>
            {
                borders.blit_inventory(
                    target,
                    texture_creation_params,
                    tilecache,
                    &hero.inventory,
                );
                update_whole_screen = true;
            }
            Event::UserEvent { code }
                if code == UserEvent::HeroHealthChanged as i32 =>
            {
                borders.blit_life(
                    target,
                    texture_creation_params,
                    tilecache,
                    hero.health.life(),
                );
                update_whole_screen = true;
            }
            Event::UserEvent { code }
                if code == UserEvent::HeroLanded as i32 =>
            {
                actor_queue.push_back(
                    ActorType::DustCloud,
                    hero.position.geometry.x as u16,
                    (hero.position.geometry.y as usize + TILE_HEIGHT)
                        as u16,
                );
            }
            _ => {}
        }
    }
    timer.remove();

    let ending = if level_data.level_passed {
        Ending::Passed
    } else {
        Ending::Failed
    };
    Ok(ending)
}

pub fn start(
    tilecache: &TileCache,
    hero: &mut HeroData,
    texture_creation_params: TextureCreationParams,
    target: &mut Surface,
    settings: &mut Settings,
    episodes: &Episodes,
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
            tilecache,
            texture_creation_params,
            target,
            &mut file,
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
            tilecache,
            texture_creation_params,
            target,
            &mut file,
            Some(message),
            79,
            144,
        )?;
    }

    hero.reset();

    target.fill(0);

    let borders = Borders {};
    borders.blit(target, texture_creation_params, tilecache);
    borders.blit_life(
        target,
        texture_creation_params,
        tilecache,
        hero.health.life(),
    );
    borders.blit_score(
        target,
        texture_creation_params,
        tilecache,
        hero.score.value(),
    );
    borders.blit_firepower(
        target,
        texture_creation_params,
        tilecache,
        &hero.firepower,
    );
    borders.blit_inventory(
        target,
        texture_creation_params,
        tilecache,
        &hero.inventory,
    );

    target.update();

    // start the game itself
    let mut level = 1;
    let mut interlevel = false;
    let mut success = Ending::Passed;

    infobox::show(
        target,
        tilecache,
        texture_creation_params,
        "Get ready FreeNukum,\nyou are going in.\n",
    );

    while success == Ending::Passed && level < 13 {
        if interlevel {
            success = start_in_level(
                2,
                tilecache,
                hero,
                texture_creation_params,
                target,
                settings,
                episodes,
                &borders,
            )?;
            level = if level == 1 { level + 2 } else { level + 1 };
            interlevel = false;
        } else {
            success = start_in_level(
                level,
                tilecache,
                hero,
                texture_creation_params,
                target,
                settings,
                episodes,
                &borders,
            )?;
            interlevel = true;
        }
    }

    if success == Ending::Passed {
        // TODO: the player finished, so we should show the end sequence
    }

    Ok(())
}

pub fn check_episodes(target: &mut Surface) -> Episodes {
    let episodes = Episodes::find_installed();
    if episodes.count() == 0 {
        show_missing_data_information(target);
    }
    episodes
}

fn show_missing_data_information(target: &mut Surface) {
    let msg = "Could not load data level and graphics files.\n\
    Please use the accompanied freenukum-data-tool\n\
    for installing the game data files";
    println!("{}", msg);

    let mut font = Font::load(10).unwrap();

    super::data::display_text(target, 0, 0, &mut font, msg);

    loop {
        match Event::wait() {
            Ok(Event::KeyDown {
                key: Some(KeyCode::Return),
                ..
            })
            | Ok(Event::KeyDown {
                key: Some(KeyCode::Escape),
                ..
            })
            | Ok(Event::Quit) => {
                return;
            }
            _ => {}
        }
    }
}

fn initialize_sdl() -> Result<()> {
    if unsafe {
        transdl::ll::SDL_Init(
            transdl::ll::SDL_INIT_VIDEO | transdl::ll::SDL_INIT_TIMER,
        )
    } < 0
    {
        use std::ffi::CString;
        let s = unsafe { CString::from_raw(transdl::ll::SDL_GetError()) };
        Err(anyhow!(
            "Can't initialize SDL: {}",
            s.into_string().unwrap()
        ))
    } else {
        Ok(())
    }
}

pub fn sdl_surface_flags(fullscreen: bool) -> u32 {
    let init = if fullscreen {
        transdl::ll::SDL_FULLSCREEN
    } else {
        0u32
    };
    init | transdl::ll::SDL_HWSURFACE
        | transdl::ll::SDL_HWACCEL
        | transdl::ll::SDL_ANYFORMAT
}

pub fn create_screen(
    w: i32,
    h: i32,
    fullscreen: bool,
) -> transdl::video::Surface {
    let depth = 0;
    transdl::video::Surface::set_video_mode(
        w,
        h,
        depth,
        sdl_surface_flags(fullscreen),
    )
}

pub fn initialize_and_get_window(
    width: i32,
    height: i32,
    fullscreen: bool,
    title: String,
    icon: String,
) -> Result<Surface> {
    initialize_sdl()?;

    use std::ffi::CString;
    let title = CString::new(title).unwrap();
    let icon = CString::new(icon).unwrap();
    unsafe {
        transdl::ll::SDL_WM_SetCaption(title.as_ptr(), icon.as_ptr())
    };

    Ok(create_screen(width, height, fullscreen))
}
