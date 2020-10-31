use anyhow::Result;
use freenukum::transition::data::original_data_dir;
use freenukum::transition::hero::HeroData;
use freenukum::transition::infobox;
use freenukum::transition::mainmenu::{mainmenu, MainMenuEntry};
use freenukum::transition::picture::show_splash;
use freenukum::transition::settings::Settings;
use freenukum::transition::tilecache::TileCache;
use freenukum::transition::{game, sdl_surface_creation_params};
use freenukum::{WINDOW_HEIGHT, WINDOW_WIDTH};
use std::fs::File;

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let mut settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
        settings.fullscreen,
        format!("Freenukum {}", VERSION),
        format!("Freenukum {}", VERSION),
    )?;
    let mut episodes = game::check_episodes(&mut screen);
    let texture_creation_params = sdl_surface_creation_params(&screen);
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        texture_creation_params,
    )?;

    let mut bg_filepath = original_data_dir().join("dn.dn1");
    {
        let mut file = File::open(&bg_filepath)?;
        show_splash(
            &tilecache,
            texture_creation_params,
            &mut screen,
            &mut file,
        )?;
    }

    let mut hero = HeroData::new();

    'menu_loop: loop {
        match mainmenu(&mut screen, &tilecache, texture_creation_params) {
            MainMenuEntry::Start => {
                game::start(
                    &tilecache,
                    &mut hero,
                    texture_creation_params,
                    &mut screen,
                    &mut settings,
                    &episodes,
                )?;
                let mut file = File::open(&bg_filepath)?;
                show_splash(
                    &tilecache,
                    texture_creation_params,
                    &mut screen,
                    &mut file,
                )?;
            }
            MainMenuEntry::Restore => infobox::show(
                &mut screen,
                &tilecache,
                texture_creation_params,
                "Restore not implemented yet",
            ),
            MainMenuEntry::Instructions => infobox::show(
                &mut screen,
                &tilecache,
                texture_creation_params,
                "Instructions not implemented yet",
            ),
            MainMenuEntry::OrderingInfo => infobox::show(
                &mut screen,
                &tilecache,
                texture_creation_params,
                "Ordering info not implemented yet",
            ),
            MainMenuEntry::FullScreenToggle => {
                screen.toggle_fullscreen();
                settings.fullscreen = !settings.fullscreen;
                settings.save();
            }
            MainMenuEntry::EpisodeChange => {
                let old = episodes.current();
                let new = episodes.switch();

                bg_filepath = original_data_dir()
                    .join(format!("dn.{}", episodes.file_extension()));

                if old == new {
                    infobox::show(
                        &mut screen,
                        &tilecache,
                        texture_creation_params,
                        "\
                        You don't have another\n\
                        episode installed.\n\
                        \n
                        We stay in this episode",
                    );
                }
                let mut file = File::open(&bg_filepath)?;
                show_splash(
                    &tilecache,
                    texture_creation_params,
                    &mut screen,
                    &mut file,
                )?;
            }
            MainMenuEntry::HighScores => infobox::show(
                &mut screen,
                &tilecache,
                texture_creation_params,
                "Highscores not implemented yet",
            ),
            MainMenuEntry::Previews => infobox::show(
                &mut screen,
                &tilecache,
                texture_creation_params,
                "Previews not implemented yet",
            ),
            MainMenuEntry::ViewUserDemo => infobox::show(
                &mut screen,
                &tilecache,
                texture_creation_params,
                "Userdemo not implemented yet",
            ),
            MainMenuEntry::TitleScreen => {
                let mut file = File::open(&bg_filepath)?;
                show_splash(
                    &tilecache,
                    texture_creation_params,
                    &mut screen,
                    &mut file,
                )?;
            }
            MainMenuEntry::Credits => infobox::show(
                &mut screen,
                &tilecache,
                texture_creation_params,
                "Credits not implemented yet",
            ),
            MainMenuEntry::Quit => {
                break 'menu_loop;
            }
            MainMenuEntry::Invalid => {}
        }
    }
    Ok(())
}

/* TODO: reactivate for SDL2 migration when required
use freenukum_rs::{
    tile, Game, Settings, MAX_TILES_PER_FILE, TILE_HEIGHT, TILE_WIDTH,
};
use std::fs::create_dir_all;

fn main() -> Result<(), String> {
    println!("Starting FreeNukum…");

    let scale = 2f32;

    let settings = Settings { scale: 2f32 };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let w = (TILE_WIDTH + 2) * MAX_TILES_PER_FILE;
    let h = (TILE_HEIGHT + 2) * tile::Category::all().len();

    let window = video_subsystem
        .window(
            "FreeNukum",
            (scale * w as f32) as u32,
            (scale * h as f32) as u32,
        )
        .position_centered()
        .vulkan()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas =
        window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_scale(scale, scale).unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.clear();

    let path = dirs::data_local_dir()
        .unwrap()
        .join("freenukum")
        .join("data");
    create_dir_all(&path).unwrap();

    let tiles = tile::load(&path, &texture_creator).unwrap();

    let mut game = Game::new(settings, canvas, sdl_context, &tiles);

    game.run()
    Ok(())
}
    */
