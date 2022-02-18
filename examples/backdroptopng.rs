// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use anyhow::{Error, Result};
use clap::Parser;
use freenukum::backdrop;
use sdl2::{image::SaveSurface, surface::Surface};
use std::fs::File;
use std::path::PathBuf;

/// Convert an original original Duke Nukem 1 backdrop file to a set of png files.
#[derive(Parser, Debug)]
struct Arguments {
    /// The path to the file that should be converted.
    /// The file is usually named `drop1.dn1` or similar.
    infile: PathBuf,

    /// The path to the output file.
    outfile: PathBuf,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut file = File::open(&args.infile)?;
    let backdrop: Surface = backdrop::load(&mut file)?;

    backdrop.save(args.outfile).map_err(Error::msg)?;
    Ok(())
}
