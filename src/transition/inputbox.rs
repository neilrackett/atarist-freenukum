use super::geometry::Geometry;
use super::inputfield::InputField;
use super::messagebox::messagebox;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::{FONT_HEIGHT, FONT_WIDTH};
use transdl::event::{Event, KeyCode, Modifier};
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
) -> Answer {
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
        match Event::wait() {
            Ok(Event::KeyDown { key, modifiers }) => {
                match key {
                    Some(KeyCode::BackSpace) => {
                        input_field.backspace_pressed();
                    }
                    Some(KeyCode::Delete) => {
                        input_field.delete_pressed();
                    }
                    Some(KeyCode::Left) => {
                        input_field.left_pressed();
                    }
                    Some(KeyCode::Right) => {
                        input_field.right_pressed();
                    }
                    Some(KeyCode::Return) => {
                        background_backup.blit(
                            None,
                            screen,
                            Some(destrect.as_sdl_rect()),
                        );
                        let text = input_field.get_text();
                        return Answer::Ok(text.to_string());
                    }
                    Some(KeyCode::Escape) => {
                        background_backup.blit(
                            None,
                            screen,
                            Some(destrect.as_sdl_rect()),
                        );
                        return Answer::Quit;
                    }
                    Some(code)
                        if code == KeyCode::Space
                            || code == KeyCode::Exclaim
                            || code == KeyCode::QuoteDouble
                            || code == KeyCode::Hash
                            || code == KeyCode::Dollar
                            || code == KeyCode::Ampersand
                            || code == KeyCode::Quote
                            || code == KeyCode::LeftParen
                            || code == KeyCode::RightParen
                            || code == KeyCode::Asterisk
                            || code == KeyCode::Plus
                            || code == KeyCode::Comma
                            || code == KeyCode::Minus
                            || code == KeyCode::Period
                            || code == KeyCode::Slash
                            || code == KeyCode::Num0
                            || code == KeyCode::Num1
                            || code == KeyCode::Num2
                            || code == KeyCode::Num3
                            || code == KeyCode::Num4
                            || code == KeyCode::Num5
                            || code == KeyCode::Num6
                            || code == KeyCode::Num7
                            || code == KeyCode::Num8
                            || code == KeyCode::Num9
                            || code == KeyCode::Colon
                            || code == KeyCode::Semicolon
                            || code == KeyCode::Less
                            || code == KeyCode::Equals
                            || code == KeyCode::Greater
                            || code == KeyCode::Question
                            || code == KeyCode::At
                            || code == KeyCode::A
                            || code == KeyCode::B
                            || code == KeyCode::C
                            || code == KeyCode::D
                            || code == KeyCode::E
                            || code == KeyCode::F
                            || code == KeyCode::G
                            || code == KeyCode::H
                            || code == KeyCode::I
                            || code == KeyCode::J
                            || code == KeyCode::K
                            || code == KeyCode::L
                            || code == KeyCode::M
                            || code == KeyCode::N
                            || code == KeyCode::O
                            || code == KeyCode::P
                            || code == KeyCode::Q
                            || code == KeyCode::R
                            || code == KeyCode::S
                            || code == KeyCode::T
                            || code == KeyCode::U
                            || code == KeyCode::V
                            || code == KeyCode::W
                            || code == KeyCode::X
                            || code == KeyCode::Y
                            || code == KeyCode::Z =>
                    {
                        let mut c = code as u8 as char;
                        if modifiers.contains(&Modifier::LeftShift)
                            || modifiers.contains(&Modifier::RightShift)
                        {
                            c.make_ascii_uppercase();
                        }

                        input_field.symbol_pressed(c);
                    }
                    Some(_) | None => {}
                };
                input_field.blit(&mut input_field_surface, tilecache);
                input_field_surface.clone_to_texture(
                    None,
                    &mut msgbox,
                    Some(input_field_rect.clone()),
                );
                msgbox.blit_to_sdl_surface(
                    None,
                    screen,
                    Some(destrect.clone()),
                );
                screen.update_rect(0, 0, 0, 0);
            }
            Ok(_) => {
                // Ignore other events
            }
            Err(e) => {
                panic!("Error getting SDL event: {:?}", e);
            }
        }
    }
}

pub mod ffi {
    use super::super::texture::ffi::FnTextureCreationParams;
    use super::super::tilecache::ffi::FnTileCache;
    use super::Answer;
    use libc::{c_char, size_t};
    use std::ffi::CStr;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    #[repr(C)]
    pub enum FnInputBoxAnswer {
        Ok,
        Quit,
    }

    #[no_mangle]
    pub extern "C" fn fn_inputbox_show(
        screen: *mut SDL_Surface,
        tilecache: *const FnTileCache,
        params: FnTextureCreationParams,
        message: *const c_char,
        answer: *mut c_char,
        answer_length: size_t,
    ) -> FnInputBoxAnswer {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());
        assert!(!message.is_null());
        assert!(!answer.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };

        let message = match unsafe { CStr::from_ptr(message) }.to_str() {
            Ok(message) => message,
            Err(e) => panic!("Error reading inputbox message: {:?}", e),
        };

        let answer: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(
                answer as *mut u8,
                answer_length as usize,
            )
        };

        match super::show(
            &mut screen,
            tilecache,
            params,
            message,
            answer_length,
        ) {
            Answer::Ok(message) => {
                let message = message.as_bytes();
                let len = std::cmp::min(message.len(), answer_length);
                answer[0..len].copy_from_slice(&message[0..len]);
                FnInputBoxAnswer::Ok
            }
            Answer::Quit => FnInputBoxAnswer::Quit,
        }
    }
}
