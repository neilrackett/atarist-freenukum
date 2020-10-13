struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{
        ANIMATION_BOMBFIRE, ANIMATION_ROBOT, OBJECT_DUSTCLOUD,
        OBJECT_STEAM, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.is_in_foreground = false;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let (tile, num_frames) = match general.actor_type {
            ActorType::Fire => (ANIMATION_BOMBFIRE, 6),
            ActorType::DustCloud => (OBJECT_DUSTCLOUD, 5),
            ActorType::Steam => (OBJECT_STEAM, 5),
            ActorType::RobotDisappearing => (ANIMATION_ROBOT + 3, 7),
            _ => {
                unreachable!(
                    "Actor type {:?} added as an animation \
                    which is not an animation id",
                    general.actor_type
                );
            }
        };

        let data = Box::new(Specific {
            tile,
            current_frame: 0,
            num_frames,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*(p.general)) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let actor_queue = unsafe { &mut (*(p.actor_queue)) };

        specific.current_frame += 1;
        if specific.current_frame == specific.num_frames {
            general.is_alive = false;
            if general.actor_type == ActorType::RobotDisappearing {
                actor_queue.push_back(
                    ActorType::Explosion,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_blit(
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

        let tile = tilecache
            .get_tile((specific.tile + specific.current_frame) as usize)
            .unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
