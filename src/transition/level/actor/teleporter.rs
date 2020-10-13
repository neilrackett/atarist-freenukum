pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorFreeParams, FnLevelActorHeroInteractStartParams,
        FnLevelActorReceiveMessageParams,
    };
    use super::super::{ActorMessageType, ActorType};
    use crate::{ANIMATION_TELEPORTER1, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_teleporter_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_teleporter_free(
        _p: FnLevelActorFreeParams,
    ) {
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_teleporter_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.actor_message_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let actor_message_queue = unsafe { &mut (*p.actor_message_queue) };

        let other = if general.actor_type == ActorType::Teleporter1 {
            ActorType::Teleporter2
        } else {
            ActorType::Teleporter1
        };

        actor_message_queue.push_back(other, ActorMessageType::Teleport);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_teleporter_blit(
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
        for i in 0..3 {
            for j in 0..3 {
                destrect.x =
                    general.position.x - (1 - j) * TILE_WIDTH as i16;
                destrect.y =
                    general.position.y - (2 - i) * TILE_HEIGHT as i16;
                let tile = tilecache
                    .get_tile(
                        ANIMATION_TELEPORTER1
                            + i as usize * 3
                            + j as usize,
                    )
                    .unwrap();
                tile.blit_to_sdl_surface(
                    None,
                    &mut target,
                    Some(destrect),
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_teleporter_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.hero_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let hero_data = unsafe { &mut (*p.hero_data) };

        if p.message != ActorMessageType::Teleport {
            return;
        }

        hero_data.position.move_to(
            general.position.x as u16,
            general.position.y as u16 - TILE_HEIGHT as u16,
        );
    }
}
