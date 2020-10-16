use super::super::super::HorizontalDirection;

#[derive(Debug)]
struct Specific {
    current_frame: usize,
    num_frames: usize,
    direction: HorizontalDirection,
}

pub mod ffi {
    use super::super::super::super::HorizontalDirection;
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{
        HALFTILE_WIDTH, SOLID_BLACK, SOLID_CONVEYORBELT_CENTER,
        SOLID_CONVEYORBELT_LEFTEND, SOLID_CONVEYORBELT_RIGHTEND,
        TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_conveyor_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };
        let level_data = unsafe { &mut (*p.level_data) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let direction = match general.actor_type {
            ActorType::ConveyorLeftMovingRightEnd => {
                HorizontalDirection::Left
            }
            ActorType::ConveyorRightMovingRightEnd => {
                HorizontalDirection::Right
            }
            _ => unreachable!(),
        };

        let data = Box::new(Specific {
            current_frame: 0,
            num_frames: 4,
            direction,
        });

        // find the beginning of the conveyor belt
        let mut found_begin = false;
        let mut tile;
        while !found_begin {
            general.position.x -= TILE_WIDTH as i16;
            general.position.w += TILE_WIDTH as u16;
            tile = level_data.tiles.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT,
            );
            if tile as usize == SOLID_CONVEYORBELT_LEFTEND
                || general.position.x == 0
                || tile == 0
            {
                found_begin = true;
                level_data.tiles.set(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT,
                    SOLID_BLACK as u16,
                );
            }
        }

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_conveyor_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_conveyor_act(
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

        let hero_push_offset = match specific.direction {
            HorizontalDirection::Left => {
                if specific.current_frame == 0 {
                    specific.current_frame = specific.num_frames;
                }
                specific.current_frame -= 1;
                -1 * HALFTILE_WIDTH as i16
            }
            HorizontalDirection::Right => {
                specific.current_frame += 1;
                specific.current_frame %= specific.num_frames;
                HALFTILE_WIDTH as i16
            }
            _ => unreachable!(),
        };

        let hero_geometry = hero_data.position.geometry;

        if hero_geometry.x + hero_geometry.w as i16 > general.position.x
            && hero_geometry.x
                < general.position.x + general.position.w as i16
            && hero_geometry.y + hero_geometry.h as i16
                == general.position.y
        {
            hero_data
                .position
                .push_horizontally(&level_data.solids, hero_push_offset);
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_conveyor_blit(
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

        let mut tile = tilecache
            .get_tile(SOLID_CONVEYORBELT_LEFTEND + specific.current_frame)
            .unwrap();
        let mut destrect = general.position;

        let num_elements = general.position.w as usize / TILE_WIDTH;
        for i in 0..num_elements {
            if i == num_elements - 1 {
                // right end of the conveyor
                tile = tilecache
                    .get_tile(
                        SOLID_CONVEYORBELT_RIGHTEND
                            + specific.current_frame,
                    )
                    .unwrap();
            } else if i == 1 {
                // center parts of the conveyor
                tile = tilecache
                    .get_tile(
                        SOLID_CONVEYORBELT_CENTER
                            + specific.current_frame % 2,
                    )
                    .unwrap();
            }
            tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }
}
