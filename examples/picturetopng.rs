use anyhow::{anyhow, Result};
use freenukum::picture;
use sdl2::image::SaveSurface;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

/// Convert an original original Duke Nukem 1 tile file to a set of png files.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the file that should be converted.
    /// The file is usually named one of: `badguy.dn1`, `credits.dn1`,
    /// `dn.dn1`, `duke.dn1`, `end.dn1`.
    infile: PathBuf,

    /// The path to the output file.
    outfile: PathBuf,
}

fn main() -> Result<()> {
    let args = Arguments::from_args();

    let mut file = File::open(&args.infile)?;
    let picture = picture::load(&mut file)?;

    picture.save(args.outfile).map_err(|s| anyhow!(s))?;
    Ok(())
}
