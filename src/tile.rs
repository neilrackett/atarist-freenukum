use crate::HALFTILE_WIDTH;
use bitvec::order::Msb0 as Endian;
use bitvec::slice::{AsBits, BitSlice};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub type Tiles<'t> = BTreeMap<Category, Vec<Texture<'t>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Back0,
    Back1,
    Back2,
    Back3,
    Solid0,
    Solid1,
    Solid2,
    Solid3,
    Anim0,
    Anim1,
    Anim2,
    Anim3,
    Anim4,
    Anim5,
    Object0,
    Object1,
    Object2,
    Man0,
    Man1,
    Man2,
    Man3,
    Man4,
    Font1,
    Font2,
    Border,
    Numbers,
}

impl Category {
    pub fn basename(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }

    pub fn filename(&self) -> PathBuf {
        let mut s = self.basename();
        s.push_str(".dn1");
        PathBuf::from(s)
    }

    pub fn count(&self) -> u8 {
        use Category as C;
        match self {
            C::Back0
            | C::Back1
            | C::Back2
            | C::Back3
            | C::Solid0
            | C::Solid1
            | C::Solid2
            | C::Solid3
            | C::Anim0
            | C::Anim1
            | C::Anim2
            | C::Anim3
            | C::Anim4
            | C::Anim5
            | C::Man0
            | C::Man1
            | C::Man2
            | C::Man3
            | C::Man4
            | C::Font1
            | C::Font2
            | C::Border
            | C::Numbers => 48,

            C::Object0 | C::Object1 | C::Object2 => 50,
        }
    }

    pub fn transparent(&self) -> bool {
        use Category as C;
        match self {
            C::Back0
            | C::Solid0
            | C::Anim0
            | C::Anim1
            | C::Anim2
            | C::Anim3
            | C::Anim4
            | C::Anim5
            | C::Object0
            | C::Object1
            | C::Object2
            | C::Man0
            | C::Man1
            | C::Man2
            | C::Man3
            | C::Man4
            | C::Font1
            | C::Font2
            | C::Border
            | C::Numbers => true,

            C::Back1
            | C::Back2
            | C::Back3
            | C::Solid1
            | C::Solid2
            | C::Solid3 => false,
        }
    }

    pub fn all() -> Vec<Category> {
        use Category as C;
        vec![
            C::Back0,
            C::Back1,
            C::Back2,
            C::Back3,
            C::Solid0,
            C::Solid1,
            C::Solid2,
            C::Solid3,
            C::Anim0,
            C::Anim1,
            C::Anim2,
            C::Anim3,
            C::Anim4,
            C::Anim5,
            C::Object0,
            C::Object1,
            C::Object2,
            C::Man0,
            C::Man1,
            C::Man2,
            C::Man3,
            C::Man4,
            C::Font1,
            C::Font2,
            C::Border,
            C::Numbers,
        ]
    }
}

pub fn load<'t, T>(
    path: &Path,
    creator: &'t TextureCreator<T>,
) -> Result<Tiles<'t>, String> {
    let mut data: BTreeMap<Category, Vec<Texture<'t>>> = BTreeMap::new();
    for category in Category::all().into_iter() {
        data.insert(
            category.clone(),
            load_category_from_directory(path, category, creator)?,
        );
    }
    Ok(data)
}

fn load_category_from_directory<'t, T>(
    path: &Path,
    category: Category,
    creator: &'t TextureCreator<T>,
) -> Result<Vec<Texture<'t>>, String> {
    load_category_from_path(&path.join(category.filename()), creator)
}

fn load_category_from_path<'t, T>(
    path: &Path,
    creator: &'t TextureCreator<T>,
) -> Result<Vec<Texture<'t>>, String> {
    let mut file = File::open(path).unwrap();

    let mut header = [0u8; 3];
    file.read_exact(&mut header).unwrap();
    let tiles = header[0];
    let data_cols = header[1];
    let data_rows = header[2];

    let mut v = Vec::new();
    for _ in 0..tiles {
        v.push(load_tile(
            &mut file,
            creator,
            data_cols as u32,
            data_rows as u32,
        )?);
    }
    Ok(v)
}

fn load_tile<'t, T, R: Read>(
    r: &mut R,
    creator: &'t TextureCreator<T>,
    data_cols: u32,
    data_rows: u32,
) -> Result<Texture<'t>, String> {
    let w = data_cols * HALFTILE_WIDTH as u32;
    let h = data_rows;
    let mut texture = creator
        .create_texture_static(PixelFormatEnum::RGBA32, w, h)
        .map_err(|e| e.to_string())?;

    let mut buffer = [0u8; 5];
    for y in 0..data_rows {
        for x_row in 0..data_cols as usize {
            r.read_exact(&mut buffer).unwrap();
            let opaque_row: &BitSlice<_, _> = buffer[0].bits::<Endian>();
            let blue_row: &BitSlice<_, _> = buffer[1].bits::<Endian>();
            let green_row: &BitSlice<_, _> = buffer[2].bits::<Endian>();
            let red_row: &BitSlice<_, _> = buffer[3].bits::<Endian>();
            let bright_row: &BitSlice<_, _> = buffer[4].bits::<Endian>();

            for x in 0..HALFTILE_WIDTH as usize {
                let bright = *bright_row.get(x).unwrap();
                let red = *red_row.get(x).unwrap();
                let green = *green_row.get(x).unwrap();
                let blue = *blue_row.get(x).unwrap();
                let opaque = *opaque_row.get(x).unwrap();

                let ugly_yellow = red && green && !blue && !bright;
                let r = 0x54 * (red as u8 * 2 + bright as u8);
                let g = 0x54
                    * (green as u8 * 2 + bright as u8 - ugly_yellow as u8);
                let b = 0x54 * (blue as u8 * 2 + bright as u8);
                let o = opaque as u8 * 0xff;

                let rect = Rect::new(
                    x as i32 + x_row as i32 * HALFTILE_WIDTH as i32,
                    y as i32,
                    1,
                    1,
                );
                texture
                    .update(rect, &[r, g, b, o], 4)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(texture)
}
