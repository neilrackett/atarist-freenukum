use super::episodes::Episodes;
use transdl::event::{Event, KeyCode};
use transdl::ttf::Font;
use transdl::video::Surface;

fn check_episodes(target: &mut Surface) -> Episodes {
    let episodes = Episodes::find_installed();
    if episodes.count() == 0 {
        show_missing_data_information(target);
    }
    episodes
}

fn show_missing_data_information(target: &mut Surface) {
    let msg = "Could not load data level and graphics files.\n\
    Please use the accompanied freenukum-data-tool\n\
    for installing the game data files";
    println!("{}", msg);

    let mut font = Font::load(10).unwrap();

    super::data::display_text(target, 0, 0, &mut font, msg);

    loop {
        match Event::wait() {
            Ok(Event::KeyDown {
                key: Some(KeyCode::Return),
                ..
            })
            | Ok(Event::KeyDown {
                key: Some(KeyCode::Escape),
                ..
            })
            | Ok(Event::Quit) => {
                return;
            }
            _ => {}
        }
    }
}

fn initialize_sdl() -> Result<(), String> {
    if unsafe {
        transdl::ll::SDL_Init(
            transdl::ll::SDL_INIT_VIDEO | transdl::ll::SDL_INIT_TIMER,
        )
    } < 0
    {
        use std::ffi::CString;
        let s = unsafe { CString::from_raw(transdl::ll::SDL_GetError()) };
        Err(format!(
            "Can't initialize SDL: {}",
            s.into_string().unwrap()
        ))
    } else {
        Ok(())
    }
}

pub fn sdl_surface_flags(fullscreen: bool) -> u32 {
    let init = if fullscreen {
        transdl::ll::SDL_FULLSCREEN
    } else {
        0u32
    };
    init | transdl::ll::SDL_HWSURFACE
        | transdl::ll::SDL_HWACCEL
        | transdl::ll::SDL_ANYFORMAT
}

pub fn create_screen(
    w: i32,
    h: i32,
    fullscreen: bool,
) -> transdl::video::Surface {
    let depth = 0;
    transdl::video::Surface::set_video_mode(
        w,
        h,
        depth,
        sdl_surface_flags(fullscreen),
    )
}

fn initialize_and_get_window(
    width: i32,
    height: i32,
    fullscreen: bool,
    title: String,
    icon: String,
) -> Result<Surface, String> {
    initialize_sdl()?;

    use std::ffi::CString;
    let title = CString::new(title).unwrap();
    let icon = CString::new(icon).unwrap();
    unsafe {
        transdl::ll::SDL_WM_SetCaption(title.as_ptr(), icon.as_ptr())
    };

    Ok(create_screen(width, height, fullscreen))
}

pub mod ffi {
    use super::super::episodes::ffi::FnEpisodes;
    use libc::c_char;
    use transdl::ll::SDL_Surface;

    #[no_mangle]
    pub extern "C" fn fn_game_initialize_and_get_window(
        width: i32,
        height: i32,
        fullscreen: bool,
        title: *const c_char,
        icon: *const c_char,
    ) -> *mut SDL_Surface {
        use std::ffi::CStr;
        let title =
            unsafe { CStr::from_ptr(title).to_str().unwrap().to_string() };
        let icon =
            unsafe { CStr::from_ptr(icon).to_str().unwrap().to_string() };
        match super::initialize_and_get_window(
            width, height, fullscreen, title, icon,
        ) {
            Ok(s) => s.raw,
            Err(e) => {
                eprintln!("{}", e);
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_game_check_episodes(
        target: *mut SDL_Surface,
    ) -> *mut FnEpisodes {
        assert!(!target.is_null());
        let mut target = transdl::video::Surface { raw: target };

        Box::into_raw(Box::new(super::check_episodes(&mut target)))
    }

    #[no_mangle]
    pub extern "C" fn fn_game_show_missing_data_information(
        target: *mut SDL_Surface,
    ) {
        assert!(!target.is_null());
        let mut target = transdl::video::Surface { raw: target };

        super::show_missing_data_information(&mut target);
    }
}
