// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::tile::{self, TileHeader};
use crate::Result;
use crate::TileProvider;
use sdl2::surface::Surface;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct TileCache<'t> {
    tiles: Vec<Surface<'t>>,
}

pub struct FileProperties {
    pub transparent: bool,
    pub name: &'static str,
    pub num_tiles: usize,
}

impl FileProperties {
    fn build(
        transparent: bool,
        name: &'static str,
        num_tiles: usize,
    ) -> FileProperties {
        FileProperties {
            transparent,
            name,
            num_tiles,
        }
    }

    pub fn get_all() -> Vec<Self> {
        let p = Self::build;
        vec![
            p(true, "back0.dn1", 48),
            p(false, "back1.dn1", 48),
            p(false, "back2.dn1", 48),
            p(false, "back3.dn1", 48),
            p(true, "solid0.dn1", 48),
            p(false, "solid1.dn1", 48),
            p(false, "solid2.dn1", 48),
            p(false, "solid3.dn1", 48),
            p(true, "anim0.dn1", 48),
            p(true, "anim1.dn1", 48),
            p(true, "anim2.dn1", 48),
            p(true, "anim3.dn1", 48),
            p(true, "anim4.dn1", 48),
            p(true, "anim5.dn1", 48),
            p(true, "object0.dn1", 50),
            p(true, "object1.dn1", 50),
            p(true, "object2.dn1", 50),
            p(true, "man0.dn1", 48),
            p(true, "man1.dn1", 48),
            p(true, "man2.dn1", 48),
            p(true, "man3.dn1", 48),
            p(true, "man4.dn1", 48),
            p(true, "font1.dn1", 50),
            p(true, "font2.dn1", 50),
            p(true, "border.dn1", 48),
            p(true, "numbers.dn1", 44),
        ]
    }
}

impl<'t> TileProvider for TileCache<'t> {
    fn get_tile(&self, index: usize) -> Option<&Surface> {
        self.tiles.get(index)
    }
}

impl<'t> TileCache<'t> {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let mut tiles = Vec::new();

        for FileProperties {
            transparent,
            name,
            num_tiles,
        } in FileProperties::get_all().into_iter()
        {
            let path = path.join(name);
            let mut file = File::open(path)?;
            let header = TileHeader::load_from(&mut file)?;
            let num_tiles =
                std::cmp::min(num_tiles, header.tiles as usize);
            tiles.append(&mut Self::load_file(
                &mut file,
                header,
                num_tiles,
                transparent,
            )?);
        }

        Ok(TileCache { tiles })
    }

    fn load_file<R: Read>(
        r: &mut R,
        header: TileHeader,
        num_tiles: usize,
        has_transparency: bool,
    ) -> Result<Vec<Surface<'t>>> {
        let mut tiles = Vec::new();
        for _ in 0..num_tiles {
            tiles.push(tile::load(r, header, has_transparency)?);
        }
        Ok(tiles)
    }
}
