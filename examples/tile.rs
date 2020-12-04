use anyhow::{anyhow, Result};
use freenukum::game;
use freenukum::graphics::SurfaceCreatorProvider;
use freenukum::settings::Settings;
use freenukum::tile::{self, TileHeader};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use transdl::event::{Event, KeyCode};
use transdl::video::Rect;

/// Show tiles from a Duke Nukem 1 grame graphics file.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the file that should be shown.
    /// The file is usually named one of: `anim0.dn1` to `anim5.dn1`,
    /// `border.dn1`, `font1.dn1`, `font2.dn1`, `numbers.dn1`,
    /// `object0.dn1` to `object2.dn1` or `solid0.dn1` to `solid3.dn1`.
    filename: PathBuf,
}

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let args = Arguments::from_args();

    let settings = Settings::load_or_create();

    let mut file = File::open(&args.filename)?;
    let header = TileHeader::load_from(&mut file)?;

    let mut r = Rect {
        x: 0,
        y: 0,
        w: header.width as u16 * 8,
        h: header.height as u16,
    };

    let mut screen = game::initialize_and_get_window(
        r.w as i32 * header.tiles as i32,
        r.h as i32,
        settings.fullscreen,
        format!("Freenukum {} tile example", VERSION),
        format!("Freenukum {} tile example", VERSION),
    )?;

    for _ in 0..header.tiles {
        let tile = tile::load(
            &mut file,
            &screen.surface_creator(),
            header,
            false,
        )?;
        tile.blit(None, &mut screen, Some(r));
        r.x += header.width as i16 * 8;
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
