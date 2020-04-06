use crate::tile::{Category, Tiles};
use crate::{FONT_HEIGHT, FONT_WIDTH};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget};

const FONT1_START_INDEX: usize = 10usize;
const FONT2_START_INDEX: usize = 19usize;

pub fn dimensions(text: &[u8]) -> (usize, usize) {
    let mut x = 0usize;
    let mut y = 0usize;
    let mut w = 0usize;

    for letter in text {
        w = std::cmp::max(x, w);
        if x == 0 {
            y += 1;
        }
        x += 1;
        if *letter == b'\n' {
            x = 0;
        }
    }
    w = std::cmp::max(x, w);

    (w * FONT_WIDTH, y * FONT_HEIGHT)
}

pub fn print<'t, C: RenderTarget>(
    target: &mut Canvas<C>,
    tiles: &Tiles<'t>,
    pos: &Point,
    text: &[u8],
) -> Result<(), String> {
    let mut dest = pos.clone();
    for letter in text {
        if *letter == b'\n' {
            dest = Point::new(pos.x(), dest.y() + FONT_HEIGHT as i32);
        } else {
            if *letter >= b' ' as u8 && *letter <= b'z' as u8 {
                print_letter(target, tiles, &dest, *letter)?;
            }
            dest = Point::new(dest.x() + FONT_WIDTH as i32, dest.y());
        }
    }

    Ok(())
}

fn print_letter<'t, C: RenderTarget>(
    target: &mut Canvas<C>,
    tiles: &Tiles<'t>,
    pos: &Point,
    letter: u8,
) -> Result<(), String> {
    let (category, index) = match letter {
        b' '..=b'G' => (
            Category::Font1,
            letter as usize - ' ' as usize + FONT1_START_INDEX,
        ),
        b'H'..=b'Z' => (Category::Font2, (letter - b'H') as usize),
        b'a'..=b'z' => (
            Category::Font2,
            (letter - b'a') as usize + FONT2_START_INDEX,
        ),
        _ => (Category::Font1, FONT1_START_INDEX as usize),
    };
    let letter = &tiles.get(&category).unwrap().get(index).unwrap();

    let dest =
        Rect::new(pos.x(), pos.y(), FONT_WIDTH as u32, FONT_HEIGHT as u32);

    target.copy(letter, None, dest)?;

    Ok(())
}
