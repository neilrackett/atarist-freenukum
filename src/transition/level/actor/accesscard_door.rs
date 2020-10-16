#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorReceiveMessageParams,
    };
    use super::super::ActorMessageType;
    use super::Specific;
    use crate::{OBJECT_LASERBEAM, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_create(
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
            tile: OBJECT_LASERBEAM,
            current_frame: 0,
            num_frames: 4,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        specific.current_frame += 1;
        specific.current_frame %= specific.num_frames;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_blit(
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

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let level_data = unsafe { &mut (*p.level_data) };

        if p.message != ActorMessageType::OpenDoor {
            return;
        }
        let x = general.position.x as usize / TILE_WIDTH;
        let y = general.position.y as usize / TILE_HEIGHT;
        level_data.solids.set(x, y, false);
        general.is_alive = false;
    }
}
