use super::super::super::HorizontalDirection;

#[derive(Debug, PartialEq, Eq)]
enum State {
    Off,
    Ignition,
    Burning,
}

struct Specific {
    tile: usize,
    direction: HorizontalDirection,
    state: State,
    counter: usize,
    touching_hero: bool,
}

pub mod ffi {
    use super::super::super::super::HorizontalDirection;
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
    };
    use super::super::ActorType;
    use super::{Specific, State};
    use crate::{
        OBJECT_FIRELEFT, OBJECT_FIRERIGHT, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fire_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16 * 3;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        let (tile, direction) = match general.actor_type {
            ActorType::FireRight => {
                (OBJECT_FIRERIGHT, HorizontalDirection::Right)
            }
            ActorType::FireLeft => {
                general.position.x -= 2 * TILE_WIDTH as i16;
                (OBJECT_FIRELEFT, HorizontalDirection::Left)
            }
            _ => unreachable!(),
        };

        let data = Box::new(Specific {
            tile,
            direction,
            state: State::Off,
            counter: 0,
            touching_hero: false,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fire_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fire_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        general.hurts_hero = specific.state == State::Burning;
        specific.touching_hero = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fire_hero_touch_end(
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
    pub extern "C" fn fn_level_actor_function_fire_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        match specific.state {
            State::Off => {
                if specific.counter == 40 {
                    specific.counter = 0;
                    specific.state = State::Ignition;
                }
            }
            State::Ignition => {
                if specific.counter == 20 {
                    specific.counter = 0;
                    specific.state = State::Burning;
                    if specific.touching_hero {
                        general.hurts_hero = true;
                    }
                }
            }
            State::Burning => {
                if specific.counter == 20 {
                    specific.counter = 0;
                    specific.state = State::Off;
                    if specific.touching_hero {
                        general.hurts_hero = false;
                    }
                }
            }
        }

        specific.counter += 1;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fire_blit(
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

        let (tile0, tile1, tile2) = match specific.state {
            State::Off => (None, None, None),
            State::Ignition => {
                if (specific.counter % 2) > 0 {
                    match specific.direction {
                        HorizontalDirection::Left => {
                            (None, None, Some(specific.tile))
                        }
                        HorizontalDirection::Right => {
                            (Some(specific.tile), None, None)
                        }
                        HorizontalDirection::Center => unreachable!(),
                    }
                } else {
                    (None, None, None)
                }
            }
            State::Burning => {
                let offset = specific.counter % 2;
                match specific.direction {
                    HorizontalDirection::Left => (
                        Some(specific.tile + 3 + offset),
                        Some(specific.tile + 1 + offset),
                        Some(specific.tile + 1 + offset),
                    ),
                    HorizontalDirection::Right => (
                        Some(specific.tile + 1 + offset),
                        Some(specific.tile + 1 + offset),
                        Some(specific.tile + 3 + offset),
                    ),
                    HorizontalDirection::Center => unreachable!(),
                }
            }
        };

        let mut destrect = general.position;
        if let Some(tile) = tile0 {
            tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
                None,
                &mut target,
                Some(destrect),
            );
        }
        destrect.x += TILE_WIDTH as i16;
        if let Some(tile) = tile1 {
            tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
                None,
                &mut target,
                Some(destrect),
            );
        }
        destrect.x += TILE_WIDTH as i16;
        if let Some(tile) = tile2 {
            tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
                None,
                &mut target,
                Some(destrect),
            );
        }
    }
}
