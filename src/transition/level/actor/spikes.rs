struct Specific {
    touching_hero: bool,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorFreeParams, FnLevelActorHeroTouchEndParams,
        FnLevelActorHeroTouchStartParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{
        OBJECT_SPIKE, OBJECT_SPIKES_DOWN, OBJECT_SPIKES_UP, TILE_HEIGHT,
        TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        let data = Box::new(Specific {
            touching_hero: false,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        general.hurts_hero = true;
        specific.touching_hero = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        general.hurts_hero = false;
        specific.touching_hero = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_spikes_blit(
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

        let tile = match general.actor_type {
            ActorType::SpikesUp => OBJECT_SPIKES_UP,
            ActorType::SpikesDown => OBJECT_SPIKES_DOWN,
            ActorType::Spike if specific.touching_hero => OBJECT_SPIKE + 1,
            ActorType::Spike => OBJECT_SPIKE,
            _ => unreachable!(),
        };

        let tile = tilecache.get_tile(tile).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
