struct Specific {
    destroyed: bool,
    current_frame: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{OBJECT_BALLOON, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16 * 2;

        let data = Box::new(Specific {
            destroyed: false,
            current_frame: 0,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        if !specific.destroyed {
            general.is_alive = false;
            hero_data.score.add(10000);
            actor_queue.push_back(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16,
            );
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_act(
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

        specific.current_frame += 1;
        specific.current_frame %= 9;

        if specific.destroyed {
            general.is_alive = false;
        } else {
            general.position.y -= 1;
            if level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_WIDTH,
            ) {
                // balloon bumps against wall
                specific.destroyed = true;
                actor_queue.push_back(
                    ActorType::Steam,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_blit(
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

        let tile = if specific.destroyed {
            OBJECT_BALLOON + 4
        } else {
            OBJECT_BALLOON
        };
        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            &mut target,
            Some(destrect),
        );

        destrect.y += TILE_HEIGHT as i16;
        tilecache
            .get_tile(OBJECT_BALLOON + 1 + specific.current_frame / 3)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_shot(
        p: FnLevelActorShotParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        specific.destroyed = true;
        actor_queue.push_back(
            ActorType::Steam,
            general.position.x as u16,
            general.position.y as u16,
        );
    }
}
