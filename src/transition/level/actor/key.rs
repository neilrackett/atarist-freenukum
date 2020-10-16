pub mod ffi {
    use super::super::super::super::hero::InventoryItem;
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorHeroTouchStartParams,
    };
    use super::super::ActorType;
    use crate::{
        OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK,
        OBJECT_KEY_RED, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_key_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        let general = unsafe { &mut (*p.general) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_key_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        let item = match general.actor_type {
            ActorType::KeyRed => InventoryItem::KeyRed,
            ActorType::KeyBlue => InventoryItem::KeyBlue,
            ActorType::KeyPink => InventoryItem::KeyPink,
            ActorType::KeyGreen => InventoryItem::KeyGreen,
            _ => unreachable!(),
        };

        hero_data.inventory.set(item);
        hero_data.score.add(1000);
        actor_queue.push_back(
            ActorType::Score1000,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_key_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        let tile = match general.actor_type {
            ActorType::KeyRed => OBJECT_KEY_RED,
            ActorType::KeyBlue => OBJECT_KEY_BLUE,
            ActorType::KeyPink => OBJECT_KEY_PINK,
            ActorType::KeyGreen => OBJECT_KEY_GREEN,
            _ => unreachable!(),
        };

        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            &mut target,
            Some(general.position),
        );
    }
}
