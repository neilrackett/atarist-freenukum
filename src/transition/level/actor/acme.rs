struct Specific {
    tile: usize,
    counter: usize,
    touching_hero: bool,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };
    use super::super::{ActorQueueItem, ActorType};
    use super::Specific;
    use crate::{OBJECT_FALLINGBLOCK, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        let data = Box::new(Specific {
            tile: OBJECT_FALLINGBLOCK,
            counter: 0,
            touching_hero: false,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        let hero_geometry = hero_data.position.geometry;

        match specific.counter {
            0 => {
                let xl = general.position.x as u16;
                let xr = xl + general.position.w;
                let y = general.position.y;
                let hxl = hero_geometry.x as u16;
                let hxr = hxl + hero_geometry.w;
                let hy = hero_geometry.y;

                if y < hy && xl < hxr && xr > hxl {
                    let mut solid_between = false;
                    for i in (y as usize / TILE_HEIGHT) + 1
                        ..hy as usize / TILE_HEIGHT
                    {
                        let x = xl as usize / TILE_WIDTH;
                        if level_data.solids.get(x, i)
                            || level_data.solids.get(x + 1, i)
                        {
                            solid_between = true;
                            break;
                        }
                    }
                    if !solid_between {
                        specific.counter += 1;
                    }
                }
            }
            c if c <= 10 && c % 2 == 0 => {
                general.position.y -= 1;
                specific.counter += 1;
            }
            c if c <= 10 && c % 2 == 1 => {
                general.position.y += 1;
                specific.counter += 1;
            }
            _ => {
                if level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT + 1,
                ) {
                    actor_queue.push_back(ActorQueueItem {
                        actor_type: ActorType::Steam,
                        x: general.position.x as u16,
                        y: general.position.y as u16,
                    });
                    actor_queue.push_particle_firework(
                        general.position.x as u16,
                        general.position.y as u16,
                        4,
                    );
                    general.is_alive = false;
                } else {
                    general.position.y += TILE_HEIGHT as i16;
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_blit(
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
            .get_tile(specific.tile)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(specific.tile + 1)
            .unwrap()
            .blit_to_sdl_surface(None, &mut target, Some(destrect));
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_shot(
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

        if specific.counter > 0 {
            hero_data.score.add(500);
            actor_queue.push_back(ActorQueueItem {
                actor_type: ActorType::Score500,
                x: general.position.x as u16,
                y: general.position.y as u16,
            });
            actor_queue.push_particle_firework(
                general.position.x as u16,
                general.position.y as u16,
                4,
            );

            general.is_alive = false;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        if specific.counter > 10 && !specific.touching_hero {
            specific.touching_hero = true;
            general.hurts_hero = true;
        }
    }
}
