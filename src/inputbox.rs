use super::inputfield::InputField;
use super::messagebox::messagebox;
use super::tilecache::TileCache;
use crate::event::{InputEvent, WaitEvent};
use crate::graphics::{SurfaceCreator, SurfaceCreatorProvider};
use crate::rendering::{MovePositionRenderer, SurfaceRenderer};
use crate::{FONT_HEIGHT, FONT_WIDTH};
use anyhow::Result;
use transdl::video::{Rect, Surface};

pub enum Answer {
    Ok(String),
    Quit,
}

pub fn show(
    screen: &mut Surface,
    tilecache: &TileCache,
    msg: &str,
    max_length: usize,
) -> Result<Answer> {
    let placeholder_msg = format!(
        "{}\n{}\n\nOk (Enter)   Abort (Esc)\n",
        msg,
        " ".repeat(max_length)
    );
    let mut msgbox = messagebox(
        &placeholder_msg,
        tilecache,
        &mut screen.surface_creator(),
    );

    let destrect = Rect {
        x: (screen.width() as isize - msgbox.width() as isize) as i16 / 2,
        y: (screen.height() as isize - msgbox.height() as isize) as i16
            / 2,
        w: msgbox.width() as u16,
        h: msgbox.height() as u16,
    };

    // backup the background
    let mut background_backup = screen
        .surface_creator()
        .create(destrect.w as u32, destrect.h as u32);
    screen.blit(Some(destrect), &mut background_backup, None);

    let mut input_field = InputField::new(max_length);
    {
        let offset_y = msgbox.height() as i32 - FONT_HEIGHT as i32 * 4;
        let mut input_field_renderer = SurfaceRenderer {
            target: &mut msgbox,
            tilecache,
        };
        let mut input_field_renderer = MovePositionRenderer {
            offset_x: FONT_WIDTH as i32,
            offset_y,
            upstream: &mut input_field_renderer,
        };

        input_field.render(&mut input_field_renderer);
    }
    msgbox.blit(None, screen, Some(destrect));
    screen.update_rect(0, 0, 0, 0);

    loop {
        match InputEvent::wait()? {
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
                background_backup.blit(None, screen, Some(destrect));
                let text = input_field.get_text();
                return Ok(Answer::Ok(text.to_string()));
            }
            InputEvent::Abort => {
                background_backup.blit(None, screen, Some(destrect));
                return Ok(Answer::Quit);
            }
            InputEvent::Letter(c) => {
                input_field.symbol_pressed(c);
            }
            InputEvent::RefreshScreen => {}
        }

        {
            let offset_y = msgbox.height() as i32 - FONT_HEIGHT as i32 * 4;
            let mut input_field_renderer = SurfaceRenderer {
                target: &mut msgbox,
                tilecache,
            };
            let mut input_field_renderer = MovePositionRenderer {
                offset_x: FONT_WIDTH as i32,
                offset_y,
                upstream: &mut input_field_renderer,
            };
            input_field.render(&mut input_field_renderer);
        }
        msgbox.blit(None, screen, Some(destrect));
        screen.update_rect(0, 0, 0, 0);
    }
}
