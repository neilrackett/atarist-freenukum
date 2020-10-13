enum State {
    Idle,
    Shooting,
    Expanded,
}

struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    state: State,
    countdown: usize,
}

pub mod ffi {
    use super::super::super::super::hero::InventoryItem;
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractStartParams,
    };
    use super::super::{ActorMessageType, ActorType};
    use super::{Specific, State};
    use crate::{OBJECT_GLOVE_SLOT, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_create(
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
            tile: OBJECT_GLOVE_SLOT,
            current_frame: 0,
            num_frames: 4,
            state: State::Idle,
            countdown: 0,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_message_queue.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_message_queue = unsafe { &mut (*p.actor_message_queue) };

        match specific.state {
            State::Idle => {
                if hero_data.inventory.is_set(InventoryItem::Glove) {
                    actor_message_queue.push_back(
                        ActorType::ExpandingFloor,
                        ActorMessageType::Expand,
                    );
                    specific.state = State::Expanded;
                } else {
                    specific.state = State::Shooting;
                    specific.countdown = 20;
                }
            }
            State::Shooting => {}
            State::Expanded => {}
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        match specific.state {
            State::Idle => {
                specific.current_frame += 1;
                specific.current_frame %= specific.num_frames;
            }
            State::Shooting => {
                specific.current_frame += 1;
                specific.current_frame %= specific.num_frames;
                specific.countdown -= 1;
                if specific.countdown % 4 == 0 {
                    actor_queue.push_back(
                        ActorType::HostileShotRight,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                } else if specific.countdown % 4 == 2 {
                    actor_queue.push_back(
                        ActorType::HostileShotLeft,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                }
                if specific.countdown == 0 {
                    specific.state = State::Idle;
                }
            }
            State::Expanded => {}
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_glove_slot_blit(
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

        let adder = if specific.current_frame == 0 { 0 } else { 1 };
        let mut destrect = general.position;
        tilecache
            .get_tile(specific.tile + adder)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));

        destrect.x -= TILE_WIDTH as i16;
        tilecache
            .get_tile(specific.tile + 2)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));

        destrect.x += 2 * TILE_WIDTH as i16;
        tilecache
            .get_tile(specific.tile + 3)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
