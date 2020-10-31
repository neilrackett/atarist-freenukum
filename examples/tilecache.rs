use anyhow::{anyhow, Result};
use freenukum::transition::data::original_data_dir;
use freenukum::transition::geometry::Geometry;
use freenukum::transition::settings::Settings;
use freenukum::transition::text;
use freenukum::transition::texture::Texture;
use freenukum::transition::tilecache::{FileProperties, TileCache};
use freenukum::transition::{game, sdl_surface_creation_params};
use freenukum::{TILE_HEIGHT, TILE_WIDTH};
use transdl::event::{Event, KeyCode};
use transdl::video::Surface;

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let file_properties = FileProperties::get_all();
    let max_tiles =
        file_properties.iter().map(|p| p.num_tiles).max().unwrap();

    let settings = Settings::load_or_create();
    let mut screen = game::initialize_and_get_window(
        ((max_tiles + 2) * TILE_WIDTH) as i32,
        ((file_properties.len() + 2) * TILE_HEIGHT) as i32,
        settings.fullscreen,
        format!("Freenukum {} tilecache example", VERSION),
        format!("Freenukum {} tilecache example", VERSION),
    )?;

    game::check_episodes(&mut screen);
    let texture_creation_params = sdl_surface_creation_params(&screen);
    let tilecache = TileCache::load_from_path(
        &original_data_dir(),
        texture_creation_params,
    )?;

    let mut destrect = Geometry {
        x: TILE_WIDTH as i16,
        y: 0,
        w: TILE_WIDTH as u16,
        h: TILE_HEIGHT as u16,
    };

    let blithex =
        |value: usize, destrect: Geometry, target: &mut Surface| {
            let mut text = Texture::create_with_params(
                TILE_WIDTH as u16,
                TILE_HEIGHT as u16,
                texture_creation_params,
            );
            text::print(
                &mut text,
                Geometry {
                    x: 0,
                    y: 0,
                    w: TILE_WIDTH as u16,
                    h: TILE_HEIGHT as u16,
                },
                &tilecache,
                &format!("{:02X}", value),
            );
            text.blit_to_sdl_surface(None, target, Some(destrect));
        };

    for x in 0..max_tiles {
        blithex(x, destrect, &mut screen);
        destrect.x += TILE_WIDTH as i16;
    }
    destrect.x = 0;
    destrect.y += TILE_HEIGHT as i16;

    let mut i = 0;
    for (row, file) in FileProperties::get_all().iter().enumerate() {
        blithex(row, destrect, &mut screen);
        destrect.x += TILE_WIDTH as i16;
        for _ in 0..file.num_tiles {
            tilecache.get_tile(i).unwrap().blit_to_sdl_surface(
                None,
                &mut screen,
                Some(destrect),
            );
            destrect.x += TILE_WIDTH as i16;
            i += 1;
        }
        destrect.x = (TILE_WIDTH * (max_tiles + 1)) as i16;
        blithex(row, destrect, &mut screen);
        destrect.x = 0;
        destrect.y += TILE_HEIGHT as i16;
    }
    destrect.x += TILE_WIDTH as i16;
    for x in 0..max_tiles {
        blithex(x, destrect, &mut screen);
        destrect.x += TILE_WIDTH as i16;
    }

    screen.update();

    'event_loop: loop {
        match Event::wait().map_err(|e| anyhow!("{}", e))? {
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
