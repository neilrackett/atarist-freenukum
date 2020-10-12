#[derive(PartialEq, Eq, Debug)]
enum State {
    Idle,
    Ascending,
    Descending,
}

#[derive(Debug)]
struct Specific {
    state: State,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractEndParams,
        FnLevelActorHeroInteractStartParams,
    };
    use super::{Specific, State};
    use crate::{
        HALFTILE_HEIGHT, OBJECT_ELEVATOR_TOP, SOLID_ELEVATOR, TILE_HEIGHT,
        TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        let data = Box::new(Specific { state: State::Idle });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_act(
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

        let hero_geometry = hero_data.position.geometry;

        if specific.state == State::Ascending
            || specific.state == State::Idle
                && general.position.h as usize > TILE_HEIGHT
        {
            // check if hero leaves elevator
            if !hero_geometry.touches(general.position)
                || general.position.x != hero_geometry.x
            {
                specific.state = State::Descending;
            }
        }

        match specific.state {
            State::Ascending => {
                if level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT - 3,
                ) {
                    // hero touches solid with head
                    specific.state = State::Idle;
                } else {
                    let offset = hero_data.position.push_vertically(
                        &level_data.solids,
                        -(TILE_HEIGHT as i16),
                    );
                    if -offset < TILE_HEIGHT as i16 {
                        hero_data
                            .position
                            .push_vertically(&level_data.solids, -offset);
                        specific.state = State::Idle;
                    } else {
                        general.position.h += (-offset) as u16;
                        general.position.y += offset as i16;

                        level_data.solids.set(
                            general.position.x as usize / TILE_WIDTH,
                            general.position.y as usize / TILE_HEIGHT,
                            true,
                        );
                    }
                }
            }
            State::Descending => {
                for _ in 0..2 {
                    if general.position.h as usize > TILE_HEIGHT {
                        level_data.solids.set(
                            general.position.x as usize / TILE_WIDTH,
                            general.position.y as usize / TILE_HEIGHT,
                            false,
                        );
                        general.position.y += TILE_HEIGHT as i16;
                        general.position.h -= TILE_HEIGHT as u16;
                    } else {
                        specific.state = State::Idle;
                    }
                }
            }
            State::Idle => {}
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };

        if hero_data.position.geometry.touches(general.position)
            && hero_data.position.geometry.y
                + hero_data.position.geometry.h as i16
                == general.position.y
        {
            specific.state = State::Ascending;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_hero_interact_end(
        p: FnLevelActorHeroInteractEndParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };

        if hero_data.position.geometry.touches(general.position)
            && hero_data.position.geometry.x == general.position.x
        {
            specific.state = State::Idle;
        } else {
            specific.state = State::Descending;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        let tile = tilecache.get_tile(SOLID_ELEVATOR).unwrap();
        let mut destrect = general.position;
        for _ in 0..(general.position.h as usize / TILE_HEIGHT - 1) * 2 {
            destrect.y += HALFTILE_HEIGHT as i16;
            tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
        }
        destrect = general.position;
        let tile = tilecache.get_tile(OBJECT_ELEVATOR_TOP).unwrap();
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
