struct Specific {
    tile: usize,
    touching_hero: bool,
    current_frame: usize,
    num_frames: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{OBJECT_HOSTILESHOT, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let tile = match general.actor_type {
            ActorType::HostileShotLeft => OBJECT_HOSTILESHOT,
            ActorType::HostileShotRight => OBJECT_HOSTILESHOT + 2,
            _ => unreachable!(
                "Passed actor type {:?} to hostile shot actor \
                which is not a shot actor id",
                general.actor_type
            ),
        };

        let data = Box::new(Specific {
            tile,
            touching_hero: false,
            current_frame: 0,
            num_frames: 2,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*(p.general)) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        specific.touching_hero = true;
        general.hurts_hero = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*(p.general)) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        specific.touching_hero = false;
        general.hurts_hero = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*(p.general)) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*(p.level_data)) };

        let offset = match general.actor_type {
            ActorType::HostileShotLeft => -(TILE_WIDTH as i16),
            ActorType::HostileShotRight => TILE_WIDTH as i16,
            _ => unreachable!(),
        };
        general.position.x += offset;

        if level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT,
        ) {
            general.is_alive = false;
        }
        if specific.touching_hero {
            general.hurts_hero = false;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_blit(
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

        let tile = tilecache
            .get_tile(specific.tile + specific.current_frame)
            .unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
