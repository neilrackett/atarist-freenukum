pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorHeroInteractStartParams,
    };
    use crate::{ANIMATION_BADGUYSCREEN, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_surveillancescreen_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };

        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_surveillancescreen_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.info_message_queue.is_null());
        let info_message_queue = unsafe { &mut (*p.info_message_queue) };

        info_message_queue.push_back("Not implemented yet.".to_string());
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_surveillancescreen_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        let mut destrect = general.position;
        tilecache
            .get_tile(ANIMATION_BADGUYSCREEN)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(ANIMATION_BADGUYSCREEN + 1)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
