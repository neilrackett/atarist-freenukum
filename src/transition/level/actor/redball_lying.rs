struct Specific {
    tile: usize,
    counter: u16,
    base_y: u16,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
        FnLevelActorShotParams,
    };
    use super::Specific;
    use crate::{ANIMATION_MINE, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_jumping_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let data = Box::new(Specific {
            tile: ANIMATION_MINE,
            counter: 0,
            base_y: general.position.y as u16,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_jumping_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_jumping_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };
        general.hurts_hero = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_jumping_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };
        general.hurts_hero = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_jumping_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let general = unsafe { &mut (*p.general) };

        let distance = match specific.counter {
            0 => 0,
            1 | 11 => 16,
            2 | 10 => 28,
            3 | 9 => 36,
            4 | 8 => 40,
            5 | 7 => 41,
            6 => 42,
            _ => unreachable!(),
        };
        general.position.y = specific.base_y as i16 - distance as i16;

        specific.counter += 1;
        specific.counter %= 12;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_jumping_blit(
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

        let tile = tilecache.get_tile(specific.tile as usize).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_jumping_shot(
        p: FnLevelActorShotParams,
    ) {
        /*
         * We don't need to do anything, this is just to absorb
         * the bullet when the actor is shot.
         */
    }
}
