pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams,
    };
    use super::super::{ActorQueueItem, ActorType};
    use crate::{
        ANIMATION_SODAFLY, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_free(
        _p: FnLevelActorFreeParams,
    ) {
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        hero_data.score.add(1000);
        actor_queue.push_back(ActorQueueItem {
            actor_type: ActorType::Score1000,
            x: general.position.x as u16,
            y: general.position.y as u16,
        });
        general.is_alive = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.level_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let level_data = unsafe { &mut (*p.level_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        general.position.y -= HALFTILE_HEIGHT as i16;
        if level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT,
        ) {
            actor_queue.push_back(ActorQueueItem {
                actor_type: ActorType::Explosion,
                x: general.position.x as u16,
                y: general.position.y as u16,
            });
            general.is_alive = false;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        let tile = tilecache
            .get_tile(
                ANIMATION_SODAFLY
                    + ((general.position.y as usize / HALFTILE_HEIGHT)
                        % 4),
            )
            .unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
