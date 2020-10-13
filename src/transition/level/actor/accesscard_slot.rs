struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

pub mod ffi {
    use super::super::super::super::hero::InventoryItem;
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractStartParams,
    };
    use super::super::{ActorMessageType, ActorType};
    use super::Specific;
    use crate::{OBJECT_ACCESS_CARD_SLOT, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_create(
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
            tile: OBJECT_ACCESS_CARD_SLOT,
            current_frame: 0,
            num_frames: 8,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_message_queue.is_null());
        assert!(!p.info_message_queue.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_message_queue = unsafe { &mut (*p.actor_message_queue) };
        let info_message_queue = unsafe { &mut (*p.info_message_queue) };

        if hero_data.inventory.is_set(InventoryItem::AccessCard) {
            actor_message_queue.push_back(
                ActorType::AccessCardDoor,
                ActorMessageType::OpenDoor,
            );
            specific.current_frame = 0;
            specific.num_frames = 1;
            specific.tile = OBJECT_ACCESS_CARD_SLOT + 8;
            hero_data.inventory.unset(InventoryItem::AccessCard);
        } else {
            info_message_queue
                .push_back("You don't have the access card\n".to_string());
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        specific.current_frame += 1;
        specific.current_frame %= specific.num_frames;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_blit(
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

        tilecache
            .get_tile(specific.tile + specific.current_frame)
            .unwrap()
            .blit_to_sdl_surface(
                None,
                &mut target,
                Some(general.position),
            );
    }
}
