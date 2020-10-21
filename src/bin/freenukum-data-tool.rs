use anyhow::Context;
use explode::explode;
use freenukum::data_dir;
use std::fs::{create_dir_all, File};
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::iter::Iterator;
use std::ops::Shl;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use zip::read::ZipArchive;

trait Skip {
    fn skip(&mut self, count: usize) -> std::io::Result<()>;
}

impl<R: Read> Skip for R {
    fn skip(&mut self, count: usize) -> std::io::Result<()> {
        for _ in 0..count {
            let mut buffer = [0; 1];
            self.read_exact(&mut buffer)?;
        }
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
struct InstallFromShrArguments {
    /// Path of the input SHR file (usually named "DN1SW20.SHR")
    input: PathBuf,
}

#[derive(StructOpt, Debug)]
struct InstallFromZipArguments {
    /// The input Zip file
    input: PathBuf,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Install the data from a SHR file.
    InstallFromShr(InstallFromShrArguments),

    /// Install the data from a ZIP file.
    InstallFromZip(InstallFromZipArguments),
}

/// FreeNukum data tool
///
/// Manages game data for the FreeNukum game
///
/// The shareware episode of the original game can be downloaded from the
/// official FTP server at
/// ftp://ftp.3drealms.com/share/1duke.zip
/// and then be installed by this tool:
///
/// `freenukum-data-tool install-from-zip /path/to/downloaded/1duke.zip`
///
/// After the game data has been successfully installed, the FreeNukum
/// game can use the data.
#[derive(StructOpt, Debug)]
struct Arguments {
    #[structopt(subcommand)]
    command: Command,
}

fn read_file_entry<R: Read>(
    reader: &mut R,
) -> Result<Option<(String, Vec<u8>)>, anyhow::Error> {
    let mut filename = [0; 16];

    match reader.read_exact(&mut filename) {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => Err(e)?,
    }

    let filename = std::str::from_utf8(
        filename
            .split(|c| *c == 0)
            .nth(0)
            .context("Couldn't extract filename")?,
    )?
    .to_string();

    reader.skip(120)?;

    let compressed_size: usize = {
        let mut b = [0; 4];
        reader.read_exact(&mut b)?;
        (b[3] as usize).shl(24)
            | (b[2] as usize).shl(16)
            | (b[1] as usize).shl(8)
            | (b[0] as usize).shl(0)
    };

    reader.skip(28)?;

    let mut buffer = Vec::new();
    buffer.resize_with(compressed_size, Default::default);

    reader.read_exact(&mut buffer)?;

    Ok(Some((filename, explode(&buffer)?)))
}

fn extract_shr_data_to_dir<R: Read>(
    reader: &mut R,
    directory: &Path,
) -> Result<(), anyhow::Error> {
    // skip header
    reader.skip(58)?;

    create_dir_all(directory)?;

    while let Some((filename, data)) = read_file_entry(reader)? {
        println!("Extracting {}", filename);
        let path = directory.join(filename);
        let mut file = File::create(path)?;
        file.write_all(&data)?;
    }
    Ok(())
}

fn extract_shr_file_to_dir(
    file: &Path,
    directory: &Path,
) -> Result<(), anyhow::Error> {
    println!(
        "Extracting data from {:?} to directory {:?}",
        file, directory
    );

    let mut file = File::open(file)?;
    extract_shr_data_to_dir(&mut file, directory)
}

fn extract_zip_nested_shr_file_to_dir(
    file: &Path,
    directory: &Path,
) -> Result<(), anyhow::Error> {
    println!(
        "Extracting data from {:?} to directory {:?}",
        file, directory
    );

    let file = File::open(file)?;
    let mut zip = ZipArchive::new(file)?;
    let filename = "DN1SW20.SHR";
    let mut zip_file = zip
        .by_name(filename)
        .context(format!("Couldn't extract file {:?}", filename))?;
    extract_shr_data_to_dir(&mut zip_file, directory)
}

fn main() -> Result<(), anyhow::Error> {
    let arguments = Arguments::from_args();

    let data_path = data_dir().join("data").join("original");

    match arguments.command {
        Command::InstallFromShr(args) => {
            extract_shr_file_to_dir(&args.input, &data_path).unwrap();
        }
        Command::InstallFromZip(args) => {
            extract_zip_nested_shr_file_to_dir(&args.input, &data_path)
                .unwrap();
        }
    }
    Ok(())
}
