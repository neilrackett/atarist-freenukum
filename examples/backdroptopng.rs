use anyhow::{anyhow, Result};
use freenukum::backdrop;
use sdl2::image::SaveSurface;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

/// Convert an original original Duke Nukem 1 backdrop file to a set of png files.
#[derive(StructOpt, Debug)]
struct Arguments {
    /// The path to the file that should be converted.
    /// The file is usually named `drop1.dn1` or similar.
    infile: PathBuf,

    /// The path to the output file.
    outfile: PathBuf,
}

fn main() -> Result<()> {
    let args = Arguments::from_args();

    let mut file = File::open(&args.infile)?;
    let backdrop = backdrop::load(&mut file)?;

    backdrop.save(args.outfile).map_err(|s| anyhow!(s))?;
    Ok(())
}
