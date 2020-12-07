use anyhow::{anyhow, Result};
use freenukum::data::original_data_dir;
use freenukum::graphics::load_default_font;
use freenukum::hero::HeroData;
use freenukum::infobox;
use freenukum::mainmenu::{mainmenu, MainMenuEntry};
use freenukum::picture::show_splash;
use freenukum::settings::Settings;
use freenukum::tilecache::TileCache;
use freenukum::{game, UserEvent};
use freenukum::{WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::pixels::Color;
use std::fs::File;

fn main() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut settings = Settings::load_or_create();

    let sdl_context = sdl2::init().map_err(|s| anyhow!(s))?;
    let video_subsystem = sdl_context.video().map_err(|s| anyhow!(s))?;
    let ttf_context = sdl2::ttf::init()?;
    let event_subsystem = sdl_context.event().map_err(|s| anyhow!(s))?;
    let timer_subsystem = sdl_context.timer().map_err(|s| anyhow!(s))?;
    let mut event_pump =
        sdl_context.event_pump().map_err(|s| anyhow!(s))?;
    let event_sender = event_subsystem.event_sender();

    event_subsystem
        .register_custom_event::<UserEvent>()
        .map_err(|s| anyhow!(s))?;

    let window = game::create_window(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        settings.fullscreen,
        &format!("Freenukum {}", VERSION),
        &video_subsystem,
    )?;
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

    let mut bg_filepath = original_data_dir().join("dn.dn1");
    {
        let mut file = File::open(&bg_filepath)?;
        show_splash(&mut canvas, &tilecache, &mut file, &mut event_pump)?;
    }

    let mut hero = HeroData::new();

    'menu_loop: loop {
        match mainmenu(
            &mut canvas,
            &tilecache,
            &mut event_pump,
            &event_sender,
            &timer_subsystem,
        )? {
            MainMenuEntry::Start => {
                game::start(
                    &mut canvas,
                    &tilecache,
                    &mut hero,
                    &mut settings,
                    &episodes,
                    &mut event_pump,
                    &event_sender,
                    &timer_subsystem,
                )?;
                let mut file = File::open(&bg_filepath)?;
                show_splash(
                    &mut canvas,
                    &tilecache,
                    &mut file,
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::Restore => {
                infobox::show(
                    &mut canvas,
                    &tilecache,
                    "Restore not implemented yet",
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::Instructions => {
                infobox::show(
                    &mut canvas,
                    &tilecache,
                    "Instructions not implemented yet",
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::OrderingInfo => {
                infobox::show(
                    &mut canvas,
                    &tilecache,
                    "Ordering info not implemented yet",
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::FullScreenToggle => {
                use sdl2::video::FullscreenType;
                // TODO: this causes the window to show artifacts
                // that are lying around in memory, will need to
                // fix that (and the scaling as well).
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
            MainMenuEntry::EpisodeChange => {
                let old = episodes.current();
                let new = episodes.switch_next();

                bg_filepath = original_data_dir()
                    .join(format!("dn.{}", episodes.file_extension()));

                if old == new {
                    infobox::show(
                        &mut canvas,
                        &tilecache,
                        "\
                             You don't have another\n\
                             episode installed.\n\
                             \n\
                             We stay in this episode",
                        &mut event_pump,
                    )?;
                }
                let mut file = File::open(&bg_filepath)?;
                show_splash(
                    &mut canvas,
                    &tilecache,
                    &mut file,
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::HighScores => {
                infobox::show(
                    &mut canvas,
                    &tilecache,
                    "Highscores not implemented yet",
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::Previews => {
                infobox::show(
                    &mut canvas,
                    &tilecache,
                    "Previews not implemented yet",
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::ViewUserDemo => infobox::show(
                &mut canvas,
                &tilecache,
                "Userdemo not implemented yet",
                &mut event_pump,
            )?,
            MainMenuEntry::TitleScreen => {
                let mut file = File::open(&bg_filepath)?;
                show_splash(
                    &mut canvas,
                    &tilecache,
                    &mut file,
                    &mut event_pump,
                )?;
            }
            MainMenuEntry::Credits => infobox::show(
                &mut canvas,
                &tilecache,
                "Credits not implemented yet",
                &mut event_pump,
            )?,
            MainMenuEntry::Quit => {
                break 'menu_loop;
            }
            MainMenuEntry::Invalid => {}
        }
    }
    Ok(())
}
