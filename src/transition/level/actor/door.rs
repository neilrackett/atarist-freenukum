#[derive(Debug, PartialEq, Eq)]
enum State {
    Closed,
    Opening,
    Open,
}

#[derive(Debug)]
struct Specific {
    tile: usize,
    counter: usize,
    state: State,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorReceiveMessageParams,
    };
    use super::super::ActorMessageType;
    use super::{Specific, State};
    use crate::{OBJECT_DOOR, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let data = Box::new(Specific {
            tile: OBJECT_DOOR,
            counter: 0,
            state: State::Closed,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };

        match specific.state {
            State::Closed => {}
            State::Opening => {
                if specific.counter == 0 {
                    level_data.solids.set(
                        general.position.x as usize / TILE_WIDTH,
                        general.position.y as usize / TILE_HEIGHT,
                        false,
                    );
                }
                specific.counter += 1;
                if specific.counter == 8 {
                    specific.state = State::Open;
                    general.is_alive = false;
                }
            }
            State::Open => {}
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_blit(
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
            .get_tile(specific.tile + specific.counter)
            .unwrap()
            .blit_to_sdl_surface(
                None,
                &mut target,
                Some(general.position),
            );
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_door_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        if p.message != ActorMessageType::OpenDoor {
            return;
        }
        specific.state = State::Opening;
    }
}
