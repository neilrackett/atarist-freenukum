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

pub mod ffi {
    use super::super::episodes::ffi::FnEpisodes;
    use transdl::ll::SDL_Surface;

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
