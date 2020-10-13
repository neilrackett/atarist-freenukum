#[derive(Debug, PartialEq)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    counter: usize,
    explode_left: bool,
    explode_right: bool,
    explode_threshold: usize,
    num_flames: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{ANIMATION_BOMB, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let data = Box::new(Specific {
            tile: ANIMATION_BOMB,
            current_frame: 0,
            num_frames: 2,
            counter: 0,
            explode_left: true,
            explode_right: true,
            explode_threshold: 12,
            num_flames: 4,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        specific.current_frame += 1;
        specific.current_frame %= specific.num_frames;

        specific.counter += 1;

        if specific.counter < specific.explode_threshold {
        } else if specific.counter
            < specific.explode_threshold + specific.num_flames
        {
            let distance = specific.counter - specific.explode_threshold;
            if specific.explode_left {
                // explode to the left if possible
                let space_is_free = !level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH - distance,
                    general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH - distance,
                    general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    actor_queue.push_back(
                        ActorType::BombFire,
                        general.position.x as u16
                            - distance as u16 * TILE_WIDTH as u16,
                        general.position.y as u16,
                    );
                } else {
                    specific.explode_left = false;
                }
            }
            if specific.explode_right {
                // explode to the right if possible
                let space_is_free = !level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH + distance,
                    general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH + distance,
                    general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    actor_queue.push_back(
                        ActorType::BombFire,
                        general.position.x as u16
                            + distance as u16 * TILE_WIDTH as u16,
                        general.position.y as u16,
                    );
                } else {
                    specific.explode_right = false;
                }
            }
        } else {
            general.is_alive = false;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_blit(
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

        if specific.counter < specific.explode_threshold {
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
}
