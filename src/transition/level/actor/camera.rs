pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorFreeParams, FnLevelActorShotParams,
    };
    use super::super::ActorType;
    use crate::{
        ANIMATION_CAMERA_CENTER, ANIMATION_CAMERA_LEFT,
        ANIMATION_CAMERA_RIGHT, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_camera_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };

        general.is_in_foreground = false;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_camera_free(
        _p: FnLevelActorFreeParams,
    ) {
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_camera_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        assert!(!p.hero_data.is_null());

        let general = unsafe { &mut (*p.general) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };
        let hero_data = unsafe { &mut (*p.hero_data) };

        let mut target = Surface { raw: target };

        let x = hero_data.position.geometry.x;
        let tile = if x - 1 > general.position.x {
            ANIMATION_CAMERA_RIGHT
        } else if x + 1 < general.position.x {
            ANIMATION_CAMERA_LEFT
        } else {
            ANIMATION_CAMERA_CENTER
        };

        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            &mut target,
            Some(general.position),
        );
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_camera_shot(
        p: FnLevelActorShotParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_queue.is_null());

        let general = unsafe { &mut (*p.general) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        general.is_alive = false;
        hero_data.score.add(100);
        actor_queue.push_back(
            ActorType::Score100,
            general.position.x as u16,
            general.position.y as u16,
        );
        actor_queue.push_back(
            ActorType::Explosion,
            general.position.x as u16,
            general.position.y as u16,
        );
    }
}
