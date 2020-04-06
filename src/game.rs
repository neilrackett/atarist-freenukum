use crate::settings::Settings;
use crate::{HALFTILE_HEIGHT, HALFTILE_WIDTH, TILES_PER_FILE};
use bitvec::order::Msb0 as Endian;
use bitvec::slice::{AsBits, BitSlice};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;

pub struct Game {
    settings: Settings,
}

impl Game {
    pub fn new(settings: Settings) -> Self {
        Game { settings }
    }

    pub fn run(&self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let scale = self.settings.scale;

        let files = vec![
            "BACK0.DN1",
            "BACK1.DN1",
            "BACK2.DN1",
            "BACK3.DN1",
            "SOLID0.DN1",
            "SOLID1.DN1",
            "SOLID2.DN1",
            "SOLID3.DN1",
            "ANIM0.DN1",
            "ANIM1.DN1",
            "ANIM2.DN1",
            "ANIM3.DN1",
            "ANIM4.DN1",
            "ANIM5.DN1",
            "OBJECT0.DN1",
            "OBJECT1.DN1",
            "OBJECT2.DN1",
            "MAN0.DN1",
            "MAN1.DN1",
            "MAN2.DN1",
            "MAN3.DN1",
            "MAN4.DN1",
            "FONT1.DN1",
            "FONT2.DN1",
            "BORDER.DN1",
            "NUMBERS.DN1",
        ];

        let window = video_subsystem
            .window(
                "FreeNukum",
                (scale
                    * ((HALFTILE_WIDTH as usize * 2 + 2) * TILES_PER_FILE)
                        as f32) as u32,
                (scale
                    * ((HALFTILE_HEIGHT as usize * 2 + 2) * files.len())
                        as f32) as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas =
            window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas
            .set_scale(self.settings.scale, self.settings.scale)
            .unwrap();
        let texture_creator = canvas.texture_creator();

        canvas.clear();

        let path = dirs::data_local_dir()
            .unwrap()
            .join("freenukum")
            .join("data");
        create_dir_all(&path).unwrap();

        for (row, file) in files.into_iter().enumerate() {
            let path = path.join(file);

            let mut file = File::open(path).unwrap();

            let mut header = [0u8; 3];
            file.read_exact(&mut header).unwrap();
            let tiles = header[0];
            let data_cols = header[1];
            let data_rows = header[2];

            for i in 0..tiles {
                let tile = load_tile(
                    &mut file,
                    &texture_creator,
                    data_cols as u32,
                    data_rows as u32,
                )
                .unwrap();

                let dst =
                    Rect::new(i as i32 * 18, row as i32 * 18, 16, 16);

                canvas.copy(&tile, None, dst).unwrap();
            }
        }
        canvas.present();

        let mut event_pump = sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
        }

        Ok(())
    }
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
