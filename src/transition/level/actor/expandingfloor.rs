#[derive(Debug)]
struct Specific {
    expanding: bool,
    finished: bool,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorReceiveMessageParams,
    };
    use super::super::ActorMessageType;
    use super::Specific;
    use crate::{SOLID_EXPANDINGFLOOR, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let data = Box::new(Specific {
            expanding: false,
            finished: false,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };

        if specific.expanding {
            let x = (general.position.x as usize
                + general.position.w as usize)
                / TILE_WIDTH;
            let y = general.position.y as usize / TILE_HEIGHT;
            let can_expand = !level_data.solids.get(x, y);
            if can_expand {
                level_data.solids.set(x, y, true);
                general.position.w += TILE_WIDTH as u16;
            } else {
                specific.expanding = false;
                specific.finished = true;
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        let tile =
            tilecache.get_tile(SOLID_EXPANDINGFLOOR as usize).unwrap();
        let mut destrect = general.position;
        for _ in 0..general.position.w as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        if p.message != ActorMessageType::Expand {
            return;
        }
        if !specific.expanding && !specific.finished {
            specific.expanding = true;
        }
    }
}
