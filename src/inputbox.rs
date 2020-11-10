use super::geometry::Geometry;
use super::inputfield::InputField;
use super::messagebox::messagebox;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::event::{InputEvent, WaitEvent};
use crate::{FONT_HEIGHT, FONT_WIDTH};
use anyhow::Result;
use transdl::video::Surface;

pub enum Answer {
    Ok(String),
    Quit,
}

pub fn show(
    screen: &mut Surface,
    tilecache: &TileCache,
    texture_creation_params: TextureCreationParams,
    msg: &str,
    max_length: usize,
) -> Result<Answer> {
    let mut input_field_surface = Texture::create_with_params(
        (FONT_WIDTH * max_length) as u16,
        FONT_HEIGHT as u16,
        texture_creation_params,
    );
    let placeholder_msg = format!(
        "{}\n{}\n\nOk (Enter)   Abort (Esc)\n",
        msg,
        " ".repeat(max_length)
    );
    let mut msgbox =
        messagebox(&placeholder_msg, tilecache, texture_creation_params);

    let destrect = Geometry::new(
        (screen.width() as isize - msgbox.width() as isize) as i16 / 2,
        (screen.height() as isize - msgbox.height() as isize) as i16 / 2,
        msgbox.width(),
        msgbox.height(),
    );

    let input_field_rect = Geometry::new(
        FONT_WIDTH as i16,
        msgbox.height() as i16 - FONT_HEIGHT as i16 * 4,
        input_field_surface.width(),
        input_field_surface.height(),
    );

    // backup the background
    let mut background_backup =
        texture_creation_params.create_surface(destrect.w, destrect.h);
    screen.blit(
        Some(destrect.as_sdl_rect()),
        &mut background_backup,
        None,
    );

    let mut input_field = InputField::new(max_length);
    input_field.blit(&mut input_field_surface, tilecache);
    input_field_surface.clone_to_texture(
        None,
        &mut msgbox,
        Some(input_field_rect.clone()),
    );
    msgbox.blit_to_sdl_surface(None, screen, Some(destrect.clone()));
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
                background_backup.blit(
                    None,
                    screen,
                    Some(destrect.as_sdl_rect()),
                );
                let text = input_field.get_text();
                return Ok(Answer::Ok(text.to_string()));
            }
            InputEvent::Abort => {
                background_backup.blit(
                    None,
                    screen,
                    Some(destrect.as_sdl_rect()),
                );
                return Ok(Answer::Quit);
            }
            InputEvent::Letter(c) => {
                input_field.symbol_pressed(c);
            }
            InputEvent::RefreshScreen => {}
        }
        input_field.blit(&mut input_field_surface, tilecache);
        input_field_surface.clone_to_texture(
            None,
            &mut msgbox,
            Some(input_field_rect.clone()),
        );
        msgbox.blit_to_sdl_surface(None, screen, Some(destrect.clone()));
        screen.update_rect(0, 0, 0, 0);
    }
}
