struct Specific {
    tile: usize,
    touching_hero: u8,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{
        ANIMATION_MINE, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_create(
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
            touching_hero: 0,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        general.hurts_hero = true;
        specific.touching_hero = 1;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        if !level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            general.position.y += HALFTILE_HEIGHT as i16;
        }

        match specific.touching_hero {
            1 => specific.touching_hero += 1,
            2 => {
                general.hurts_hero = false;
                general.is_alive = false;
                actor_queue.push_back(
                    ActorType::Fire,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            _ => {}
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_redball_lying_blit(
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
    pub extern "C" fn fn_level_actor_function_redball_lying_shot(
        _p: FnLevelActorShotParams,
    ) {
        /*
         * We don't need to do anything, this is just to absorb
         * the bullet when the actor is shot.
         */
    }
}
