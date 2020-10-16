pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorHeroInteractStartParams,
    };
    use crate::{OBJECT_NOTEBOOK, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_notebook_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_notebook_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.info_message_queue.is_null());
        let info_message_queue = unsafe { &mut (*p.info_message_queue) };

        // TODO: implement functionality.
        info_message_queue.push_back("Not implemented yet.".to_string());
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_notebook_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        tilecache
            .get_tile(OBJECT_NOTEBOOK)
            .unwrap()
            .blit_to_sdl_surface(
                None,
                &mut target,
                Some(general.position),
            );
    }
}
