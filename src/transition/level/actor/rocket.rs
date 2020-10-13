#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Flying,
}

#[derive(Debug, PartialEq)]
struct Specific {
    state: State,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorShotParams,
    };
    use super::{Specific, State};
    use crate::{HALFTILE_HEIGHT, OBJECT_ROCKET, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_create(
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

        let tile_x = general.position.x as usize / TILE_WIDTH;
        let tile_y = general.position.y as usize / TILE_HEIGHT;
        level_data
            .tiles
            .copy_from_to(tile_x, tile_y - 1, tile_x, tile_y);

        let data = Box::new(Specific { state: State::Idle });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };

        match specific.state {
            State::Idle => {}
            State::Flying => {
                general.position.y -= HALFTILE_HEIGHT as i16;
                if level_data.solids.collides(general.position) {
                    let tile_x = general.position.x as usize / TILE_WIDTH;
                    let tile_y = general.position.y as usize / TILE_HEIGHT;
                    level_data.solids.set(tile_x, tile_y + 1, false);
                    // TODO: trigger a re-rendering of the affected tiles
                    level_data.tiles.copy_from_to(
                        tile_x,
                        tile_y - 1,
                        tile_x,
                        tile_y,
                    );
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_blit(
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
        destrect.y -= TILE_HEIGHT as i16 * 3;

        let tile = tilecache.get_tile(OBJECT_ROCKET).unwrap();
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));

        let tile = tilecache.get_tile(OBJECT_ROCKET + 1).unwrap();
        for _ in 0..2 {
            destrect.y += TILE_HEIGHT as i16;
            tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
        }

        let tile = tilecache.get_tile(OBJECT_ROCKET + 2).unwrap();
        destrect.y += TILE_HEIGHT as i16;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));

        let tile = tilecache.get_tile(OBJECT_ROCKET + 3).unwrap();
        destrect.x -= TILE_WIDTH as i16;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));

        let tile = tilecache.get_tile(OBJECT_ROCKET + 4).unwrap();
        destrect.x += 2 * TILE_WIDTH as i16;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));

        if specific.state == State::Flying {
            let tile = tilecache.get_tile(OBJECT_ROCKET + 6).unwrap();
            destrect.x -= TILE_WIDTH as i16;
            destrect.y += TILE_HEIGHT as i16;
            tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_shot(
        p: FnLevelActorShotParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };

        if specific.state == State::Idle {
            // TODO: create animation
            specific.state = State::Flying;
            let tile_x = general.position.x as usize / TILE_WIDTH;
            let tile_y = (general.position.y as usize
                + general.position.h as usize)
                / TILE_HEIGHT;

            level_data.solids.set(tile_x, tile_y, false);
            // TODO: trigger a re-rendering of the affected tiles
            level_data.tiles.copy_from_to(
                tile_x,
                tile_y + 1,
                tile_x,
                tile_y,
            );
        }
    }
}
