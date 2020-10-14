struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
    };
    use super::Specific;
    use crate::{ANIMATION_BOMBFIRE, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bombfire_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.is_in_foreground = false;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let data = Box::new(Specific {
            tile: ANIMATION_BOMBFIRE,
            current_frame: 0,
            num_frames: 6,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bombfire_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bombfire_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*(p.general)) };

        general.hurts_hero = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bombfire_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*(p.general)) };

        general.hurts_hero = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bombfire_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*(p.general)) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        specific.current_frame += 1;
        if specific.current_frame == specific.num_frames {
            general.is_alive = false;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bombfire_blit(
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

        tilecache
            .get_tile((specific.tile + specific.current_frame) as usize)
            .unwrap()
            .blit_to_sdl_surface(
                None,
                &mut target,
                Some(general.position),
            );
    }
}
