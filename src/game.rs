use crate::settings::Settings;
use crate::tile;
use crate::{MAX_TILES_PER_FILE, TILE_HEIGHT, TILE_WIDTH};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget};

const MIN_W: u32 = TILE_WIDTH as u32 * 20;
const MIN_H: u32 = TILE_HEIGHT as u32 * 12;

enum Continuation {
    Continue,
    Leave,
}

enum State {
    MainMenu,
}

pub struct Game<'a, C: RenderTarget> {
    settings: Settings,
    canvas: Canvas<C>,
    sdl_context: sdl2::Sdl,
    tiles: &'a tile::Tiles<'a>,
    state: State,
}

impl<'a, C: RenderTarget> Game<'a, C> {
    pub fn new(
        settings: Settings,
        canvas: Canvas<C>,
        sdl_context: sdl2::Sdl,
        tiles: &'a tile::Tiles<'a>,
    ) -> Self {
        Game {
            settings,
            canvas,
            tiles,
            sdl_context,
            state: State::MainMenu,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let w = (TILE_WIDTH + 2) * MAX_TILES_PER_FILE;
        let h = (TILE_HEIGHT + 2) * tile::Category::all().len();

        /* TODO: move this to external tool
        for (row, (_category, tiles)) in tiles.iter().enumerate() {
            for (col, tile) in tiles.iter().enumerate() {
                let dst =
                    Rect::new(col as i32 * 18, row as i32 * 18, 16, 16);
                canvas.copy(tile, None, dst).unwrap();
            }
        }
        */

        let dst = Rect::new(0, 0, w as u32, h as u32);

        let msg = b"Hello World!\nHow are you today?\nFine, thanks a lot $$$#\":&;<=>@";

        crate::borders::draw(&mut self.canvas, &self.tiles, &dst)?;

        crate::text::print(
            &mut self.canvas,
            &self.tiles,
            &Point::new(100, 100),
            msg,
        )?;

        self.canvas.present();

        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            for e in event_pump.poll_iter() {
                match self.process_event(e)? {
                    Continuation::Continue => {}
                    Continuation::Leave => break 'running,
                }
            }
        }

        Ok(())
    }

    fn process_event(&mut self, e: Event) -> Result<Continuation, String> {
        match e {
            Event::Window { win_event, .. } => match win_event {
                WindowEvent::SizeChanged(w, h) => {
                    let scale = self.settings.scale;
                    let w =
                        std::cmp::max((w as f32 / scale) as u32, MIN_W);
                    let h =
                        std::cmp::max((h as f32 / scale) as u32, MIN_H);
                    let dst = Rect::new(0, 0, w, h);
                    crate::borders::draw(
                        &mut self.canvas,
                        &self.tiles,
                        &dst,
                    )?;
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGB(
                        150, 0, 0,
                    ));
                    let msg = b"Hello!";
                    crate::text::print(
                        &mut self.canvas,
                        &self.tiles,
                        &Point::new(100, 100),
                        msg,
                    )?;
                    self.canvas.present();
                    Ok(Continuation::Continue)
                }
                _ => match self.state {
                    State::MainMenu => self.process_event_main_menu(e),
                },
            },
            _ => match self.state {
                State::MainMenu => self.process_event_main_menu(e),
            },
        }
    }

    fn process_event_main_menu(
        &mut self,
        e: Event,
    ) -> Result<Continuation, String> {
        match e {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Ok(Continuation::Leave),
            _ => Ok(Continuation::Continue),
        }
    }
}
