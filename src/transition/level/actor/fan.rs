#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    running: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorShotParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{ANIMATION_FAN, HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.y -= TILE_HEIGHT as i16;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16 * 2;

        let data = Box::new(Specific {
            tile: ANIMATION_FAN,
            current_frame: 0,
            num_frames: 4,
            running: 10,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        assert!(!p.hero_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };
        let hero_data = unsafe { &mut (*p.hero_data) };

        match specific.running {
            0 => {}
            1 => {
                specific.current_frame += 1;
            }
            2 => {}
            3 => {}
            4 => {}
            5 => {
                specific.current_frame += 1;
            }
            6 => {}
            7 => {}
            8 => {
                specific.current_frame += 1;
            }
            9 => {}
            10 => {
                specific.current_frame += 1;
            }
            _ => unreachable!(),
        }
        specific.current_frame %= specific.num_frames;
        if specific.running < 10 && specific.running > 0 {
            specific.running -= 1;
        } else if specific.running == 10 {
            if hero_data
                .position
                .geometry
                .overlaps_vertically(general.position)
            {
                let mut hdistance = hero_data
                    .position
                    .geometry
                    .horizontal_distance(general.position);

                let fan_direction = match general.actor_type {
                    ActorType::FanLeft => -1,
                    ActorType::FanRight => 1,
                    _ => unreachable!(),
                };
                if (fan_direction > 0 && hdistance > 0)
                    || (fan_direction < 0 && hdistance < 0)
                {
                    return;
                }

                if hdistance == 0 {
                    hdistance = HALFTILE_WIDTH as i32 * fan_direction;
                }

                if hdistance.abs() < 8 * HALFTILE_WIDTH as i32 {
                    hero_data.position.push_horizontally(
                        &level_data.solids,
                        fan_direction as i16 * TILE_WIDTH as i16,
                    );
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_blit(
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

        let mut destrect = general.position;
        tilecache
            .get_tile(specific.tile + specific.current_frame * 2)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
        destrect.y += TILE_HEIGHT as i16;
        tilecache
            .get_tile(specific.tile + specific.current_frame * 2 + 1)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_shot(
        p: FnLevelActorShotParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        specific.running = 9;
        actor_queue.push_back(
            ActorType::Steam,
            general.position.x as u16,
            general.position.y as u16,
        );
    }
}
