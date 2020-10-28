use crate::data_dir;
use std::path::PathBuf;
use transdl::ll::{SDL_Color, SDL_Rect};
use transdl::ttf::Font;
use transdl::video::Surface;

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

pub fn display_text(
    target: &mut Surface,
    x: i16,
    y: i16,
    font: &Font,
    message: &str,
) {
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

    for line in message.lines() {
        let text = font.render_utf8_shaded(line, fgcolor, bgcolor);
        destrect.y += text.height() as i16;
        text.blit(None, target, Some(destrect));
    }
    target.update_rect(0, 0, 0, 0);
}

pub mod ffi {
    use libc::c_char;
    use std::ffi::CStr;
    use transdl::ll::{SDL_Surface, TTF_Font};

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
        let mut target = transdl::video::Surface { raw: target };
        let mut font = transdl::ttf::Font { raw: font };

        super::display_text(&mut target, x, y, &mut font, s);
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
