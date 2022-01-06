use super::inputfield::InputField;
use super::messagebox::messagebox;
use crate::event::{InputEvent, WaitEvent};
use crate::rendering::{CanvasRenderer, MovePositionRenderer};
use crate::{TileProvider, FONT_HEIGHT, FONT_WIDTH};
use anyhow::{Error, Result};
use sdl2::{
    rect::{Point, Rect},
    render::WindowCanvas,
    surface::Surface,
    EventPump,
};

pub enum Answer {
    Ok(String),
    Quit,
}

pub fn show(
    canvas: &mut WindowCanvas,
    tileprovider: &dyn TileProvider,
    msg: &str,
    max_length: usize,
    event_pump: &mut EventPump,
) -> Result<Answer> {
    let placeholder_msg = format!(
        "{}\n{}\n\nOk (Enter)   Abort (Esc)\n",
        msg,
        " ".repeat(max_length)
    );
    let texture_creator = canvas.texture_creator();
    let messagebox =
        messagebox(&placeholder_msg, tileprovider, &texture_creator)?;
    let surface =
        canvas.window().surface(event_pump).map_err(Error::msg)?;

    let destrect = Rect::from_center(
        Point::new(
            surface.width() as i32 / 2,
            surface.height() as i32 / 2,
        ),
        messagebox.width(),
        messagebox.height(),
    );

    let mut background_backup = Surface::new(
        destrect.width(),
        destrect.height(),
        surface.pixel_format_enum(),
    )
    .map_err(Error::msg)?;
    surface
        .blit(destrect, &mut background_backup, None)
        .map_err(Error::msg)?;

    canvas
        .copy(&messagebox.as_texture(&texture_creator)?, None, destrect)
        .map_err(Error::msg)?;

    let offset_x = destrect.left() + FONT_WIDTH as i32;
    let offset_y = destrect.bottom() - FONT_HEIGHT as i32 * 4;
    let mut input_field = InputField::new(max_length);
    {
        let mut input_field_renderer = CanvasRenderer {
            canvas,
            texture_creator: &texture_creator,
            tileprovider,
        };
        let mut input_field_renderer = MovePositionRenderer {
            offset_x,
            offset_y,
            upstream: &mut input_field_renderer,
        };

        input_field.render(&mut input_field_renderer)?;
    }
    canvas.present();

    loop {
        match InputEvent::wait(event_pump)? {
            InputEvent::DeleteLeft => {
                input_field.backspace_pressed();
            }
            InputEvent::DeleteRight => {
                input_field.delete_pressed();
            }
            InputEvent::MoveCursorLeft => {
                input_field.left_pressed();
            }
            InputEvent::MoveCursorRight => {
                input_field.right_pressed();
            }
            InputEvent::Confirm => {
                canvas
                    .copy(
                        &background_backup.as_texture(&texture_creator)?,
                        None,
                        destrect,
                    )
                    .map_err(Error::msg)?;
                canvas.present();
                let text = input_field.get_text();
                return Ok(Answer::Ok(text.to_string()));
            }
            InputEvent::Abort => {
                canvas
                    .copy(
                        &background_backup.as_texture(&texture_creator)?,
                        None,
                        destrect,
                    )
                    .map_err(Error::msg)?;
                canvas.present();
                return Ok(Answer::Quit);
            }
            InputEvent::Letter(c) => {
                input_field.symbol_pressed(c);
            }
            InputEvent::RefreshScreen => {}
        }

        {
            let mut input_field_renderer = CanvasRenderer {
                canvas,
                texture_creator: &texture_creator,
                tileprovider,
            };
            let mut input_field_renderer = MovePositionRenderer {
                offset_x,
                offset_y,
                upstream: &mut input_field_renderer,
            };
            input_field.render(&mut input_field_renderer)?;
        }
        canvas.present();
    }
}
