use anyhow::{anyhow, Result};
use freenukum::tilecache::{FileProperties, TileCache};
use freenukum::tileprovider::TileProvider;
use sdl2::image::SaveSurface;
use std::fs::create_dir_all;
use std::path::PathBuf;
use structopt::StructOpt;

/// Convert a set of original Duke Nukem 1 tile files to a set of png files.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the directory with the files that should be converted.
    /// The files must be lowercase, such as `anim0.dn1` to `anim5.dn1`,
    /// `border.dn1`, `font1.dn1`, `font2.dn1`, `numbers.dn1`,
    /// `object0.dn1` to `object2.dn1` or `solid0.dn1` to `solid3.dn1`.
    directory: PathBuf,

    /// The directory where the extracted tiles should be placed.
    /// The tiles are numbered as `tile_000.png` to `tile_102.png`.
    /// If the directory doesn't exist, the program will attempt to create it.
    destination: PathBuf,
}

fn main() -> Result<()> {
    let args = Arguments::from_args();

    let file_properties = FileProperties::get_all();
    let max_tiles =
        file_properties.iter().map(|p| p.num_tiles).sum::<usize>();

    let tilecache = TileCache::load_from_path(&args.directory)?;

    create_dir_all(&args.destination)?;

    for i in 0..max_tiles {
        let tile = tilecache.get_tile(i).unwrap();
        let filename = format!("tile_{:04}.png", i);
        tile.save(args.destination.join(filename))
            .map_err(|s| anyhow!(s))?;
    }
    Ok(())
}
