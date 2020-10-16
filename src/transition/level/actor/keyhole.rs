struct Specific {
    tile: usize,
    counter: usize,
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
    use crate::{
        OBJECT_KEYHOLE_BLACK, OBJECT_KEYHOLE_BLUE, OBJECT_KEYHOLE_GREEN,
        OBJECT_KEYHOLE_PINK, OBJECT_KEYHOLE_RED, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        let data = Box::new(Specific {
            tile: OBJECT_KEYHOLE_BLACK,
            counter: 0,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        if specific.counter < 5 {
            specific.counter += 1;
            specific.counter %= 4;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_blit(
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

        let tile = match (specific.counter, general.actor_type) {
            (0, _) => specific.tile,
            (_, ActorType::KeyholeRed) => OBJECT_KEYHOLE_RED,
            (_, ActorType::KeyholeBlue) => OBJECT_KEYHOLE_BLUE,
            (_, ActorType::KeyholePink) => OBJECT_KEYHOLE_PINK,
            (_, ActorType::KeyholeGreen) => OBJECT_KEYHOLE_GREEN,
            _ => unreachable!(),
        };

        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            &mut target,
            Some(general.position),
        );
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_message_queue.is_null());
        assert!(!p.info_message_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_message_queue = unsafe { &mut (*p.actor_message_queue) };
        let info_message_queue = unsafe { &mut (*p.info_message_queue) };

        let (required_item, door_actor_type) = match general.actor_type {
            ActorType::KeyholeRed => {
                (InventoryItem::KeyRed, ActorType::DoorRed)
            }
            ActorType::KeyholeBlue => {
                (InventoryItem::KeyBlue, ActorType::DoorBlue)
            }
            ActorType::KeyholePink => {
                (InventoryItem::KeyPink, ActorType::DoorPink)
            }
            ActorType::KeyholeGreen => {
                (InventoryItem::KeyGreen, ActorType::DoorGreen)
            }
            _ => unreachable!(),
        };

        if hero_data.inventory.is_set(required_item) {
            actor_message_queue
                .push_back(door_actor_type, ActorMessageType::OpenDoor);
            specific.counter = 5;
            hero_data.inventory.unset(required_item);
        } else if specific.counter < 5 {
            let color = match required_item {
                InventoryItem::KeyRed => "red",
                InventoryItem::KeyBlue => "blue",
                InventoryItem::KeyPink => "pink",
                InventoryItem::KeyGreen => "green",
                _ => unreachable!(),
            };
            info_message_queue
                .push_back(format!("You don't have the {} key.", color));
        }
    }
}
