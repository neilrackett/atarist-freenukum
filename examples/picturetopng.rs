// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use anyhow::{Error, Result};
use clap::Parser;
use freenukum::picture;
use sdl2::{image::SaveSurface, surface::Surface};
use std::fs::File;
use std::path::PathBuf;

/// Convert an original original Duke Nukem 1 tile file to a set of png files.
#[derive(Parser, Debug)]
struct Arguments {
    /// The path to the file that should be converted.
    /// The file is usually named one of: `badguy.dn1`, `credits.dn1`,
    /// `dn.dn1`, `duke.dn1`, `end.dn1`.
    infile: PathBuf,

    /// The path to the output file.
    outfile: PathBuf,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut file = File::open(&args.infile)?;
    let picture: Surface = picture::load(&mut file)?;

    picture.save(args.outfile).map_err(Error::msg)?;
    Ok(())
}
