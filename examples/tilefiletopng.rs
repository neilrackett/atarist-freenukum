use anyhow::{Error, Result};
use freenukum::tile::{self, TileHeader};
use sdl2::{image::SaveSurface, surface::Surface};
use std::fs::{create_dir_all, File};
use std::path::PathBuf;
use structopt::StructOpt;

/// Convert an original original Duke Nukem 1 tile file to a set of png files.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the file that should be converted.
    /// The file is usually named one of: `anim0.dn1` to `anim5.dn1`,
    /// `border.dn1`, `font1.dn1`, `font2.dn1`, `numbers.dn1`,
    /// `object0.dn1` to `object2.dn1` or `solid0.dn1` to `solid3.dn1`.
    filename: PathBuf,

    /// The directory where the extracted tiles should be placed.
    /// The tiles are numbered as `tile_00.png` to `tile_50.png`.
    /// If the directory doesn't exist, the program will attempt to create it.
    destination: PathBuf,
}

fn main() -> Result<()> {
    let args = Arguments::from_args();

    let mut file = File::open(&args.filename)?;
    let header = TileHeader::load_from(&mut file)?;

    create_dir_all(&args.destination)?;

    for i in 0..header.tiles {
        let tile: Surface = tile::load(&mut file, header, false)?;
        let filename = format!("tile_{:02}.png", i);
        tile.save(args.destination.join(filename))
            .map_err(Error::msg)?;
    }
    Ok(())
}
