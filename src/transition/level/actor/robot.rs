use super::super::super::HorizontalDirection;

struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    touching_hero: bool,
}

pub mod ffi {
    use super::super::super::super::HorizontalDirection;
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
        FnLevelActorShotParams,
    };
    use super::super::{ActorQueueItem, ActorType};
    use super::Specific;
    use crate::{
        ANIMATION_ROBOT, HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT,
        TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let data = Box::new(Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_ROBOT,
            current_frame: 0,
            num_frames: 3,
            touching_hero: false,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        general.hurts_hero = true;
        specific.touching_hero = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        general.hurts_hero = false;
        specific.touching_hero = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };

        specific.current_frame += 1;
        specific.current_frame %= specific.num_frames;

        if !level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            // In the air, falling down.
            general.position.y += HALFTILE_HEIGHT as i16;
        } else {
            // On the floor, walking.
            if specific.current_frame == 0 {
                let mut direction = match specific.direction {
                    HorizontalDirection::Left => -1,
                    HorizontalDirection::Right => 2,
                    HorizontalDirection::Center => unreachable!(),
                };
                // Check if the place next to the bot is free
                if !level_data.solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize/ TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT
            ) &&
            // Check if the tile below this free place is solid
            level_data.solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT
            ) {
                    if direction == 2 {
                        direction = 1;
                    }
                    general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                } else {
                    specific.direction = if specific.direction
                        == HorizontalDirection::Left
                    {
                        HorizontalDirection::Right
                    } else {
                        HorizontalDirection::Left
                    };
                    if direction == 2 {
                        direction = 1
                    };
                    direction *= -1;
                    general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_blit(
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

        let tile = tilecache.get_tile(specific.tile as usize).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_shot(
        p: FnLevelActorShotParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        hero_data.score.add(100);
        if specific.touching_hero {
            general.hurts_hero = false;
            specific.touching_hero = false;
        }
        actor_queue.push_back(ActorQueueItem {
            actor_type: ActorType::RobotDisappearing,
            x: general.position.x as u16,
            y: general.position.y as u16,
        });
        general.is_alive = false;
    }
}
