use crate::settings::Settings;
use crate::tile;
use crate::{MAX_TILES_PER_FILE, TILE_HEIGHT, TILE_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::fs::create_dir_all;

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

        let w = (TILE_WIDTH + 2) * MAX_TILES_PER_FILE;
        let h = (TILE_HEIGHT + 2) * tile::Category::all().len();

        let window = video_subsystem
            .window(
                "FreeNukum",
                (scale * w as f32) as u32,
                (scale * h as f32) as u32,
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

        let tiles = tile::load(&path, &texture_creator).unwrap();

        for (row, (_category, tiles)) in tiles.iter().enumerate() {
            for (col, tile) in tiles.iter().enumerate() {
                let dst =
                    Rect::new(col as i32 * 18, row as i32 * 18, 16, 16);
                canvas.copy(tile, None, dst).unwrap();
            }
        }

        let dst = Rect::new(0, 0, w as u32, h as u32);

        crate::borders::draw_borders(
            &mut canvas,
            tiles.get(&crate::tile::Category::Border).unwrap(),
            &dst,
        )?;

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
