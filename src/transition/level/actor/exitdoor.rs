#[derive(PartialEq, Eq, Debug)]
enum State {
    Closed,
    Opening,
    Closing,
}

#[derive(Debug)]
struct Specific {
    tile: usize,
    counter: usize,
    state: State,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractStartParams,
    };
    use super::{Specific, State};
    use crate::{ANIMATION_EXITDOOR, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16 * 2;
        general.is_in_foreground = false;

        let data = Box::new(Specific {
            tile: ANIMATION_EXITDOOR,
            counter: 0,
            state: State::Closed,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };

        if specific.state == State::Closed {
            specific.state = State::Opening;
        }
        level_data.level_passed = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        assert!(!p.hero_data.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };
        let hero_data = unsafe { &mut (*p.hero_data) };

        match specific.state {
            State::Closed => {}
            State::Opening => {
                specific.counter += 1;
                if specific.counter >= 4 {
                    hero_data.hidden = true;
                    specific.state = State::Closing;
                    specific.counter -= 1;
                }
            }
            State::Closing => {
                if specific.counter == 0 {
                    level_data.do_play = false;
                    hero_data.hidden = false;
                } else {
                    specific.counter -= 1;
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_exitdoor_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        let mut destrect = general.position;
        tilecache
            .get_tile(specific.tile + specific.counter * 4)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(specific.tile + specific.counter * 4 + 1)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
        destrect.x -= TILE_WIDTH as i16;
        destrect.y += TILE_HEIGHT as i16;
        tilecache
            .get_tile(specific.tile + specific.counter * 4 + 2)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(specific.tile + specific.counter * 4 + 3)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
