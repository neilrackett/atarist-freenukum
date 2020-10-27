use crate::data_dir;
use std::path::PathBuf;

pub fn required_file_names() -> Vec<&'static str> {
    vec![
        "anim0", "anim1", "anim2", "anim3", "anim4", "anim5", "back0",
        "back1", "back2", "back3", "badguy", "border", "credits", "dn",
        "drop0", "duke", "duke1-b", "duke1", "end", "font1", "font2",
        "man0", "man1", "man2", "man3", "man4", "numbers", "object0",
        "object1", "object2", "solid0", "solid1", "solid2", "solid3",
        "worldal1", "worldal2", "worldal3", "worldal4", "worldal5",
        "worldal6", "worldal7", "worldal8", "worldal9", "worldala",
        "worldalb", "worldalc",
    ]
}

pub fn original_data_dir() -> PathBuf {
    data_dir().join("data").join("original")
}

pub mod ffi {
    use libc::c_char;
    use std::ffi::CStr;
    use transdl::ll::{
        SDL_Color, SDL_FreeSurface, SDL_Rect, SDL_Surface, SDL_UpperBlit,
        TTF_Font, TTF_RenderText_Shaded,
    };

    use super::super::file::ffi::FnFile;

    #[no_mangle]
    pub extern "C" fn fn_data_display_text(
        target: *mut SDL_Surface,
        x: i16,
        y: i16,
        font: *mut TTF_Font,
        message: *const c_char,
    ) {
        assert!(!target.is_null());
        assert!(!font.is_null());

        let s = unsafe { CStr::from_ptr(message) }.to_str().unwrap();

        let mut destrect = SDL_Rect { x, y, w: 0, h: 0 };
        let fgcolor = SDL_Color {
            r: 255,
            g: 255,
            b: 255,
            unused: 0,
        };
        let bgcolor = SDL_Color {
            r: 0,
            g: 0,
            b: 0,
            unused: 0,
        };

        for line in s.lines() {
            unsafe {
                let line = line
                    .as_bytes()
                    .iter()
                    .map(|c| *c as i8)
                    .collect::<Vec<_>>();
                let text = TTF_RenderText_Shaded(
                    font,
                    line.as_ptr() as *const i8,
                    fgcolor,
                    bgcolor,
                );
                destrect.y += (*text).h as i16;
                SDL_UpperBlit(
                    text,
                    std::ptr::null_mut(),
                    target,
                    &mut destrect as *mut SDL_Rect,
                );
                SDL_FreeSurface(text);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_data_open_file(
        filename: *const c_char,
    ) -> *mut FnFile {
        let filename =
            unsafe { CStr::from_ptr(filename) }.to_str().unwrap();

        let data_path = super::original_data_dir();
        Box::into_raw(Box::new(
            FnFile::open(&format!(
                "{}/{}",
                data_path.to_str().unwrap(),
                filename
            ))
            .unwrap(),
        ))
    }
}
