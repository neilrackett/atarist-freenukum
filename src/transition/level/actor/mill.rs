struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    lives: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{OBJECT_ROTATINGCYLINDER, TILE_HEIGHT, TILE_WIDTH};
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_create(
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
        general.is_in_foreground = false;

        while general.position.y > 0
            && !level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT - 1,
            )
        {
            general.position.y -= TILE_HEIGHT as i16;
            general.position.h += TILE_HEIGHT as u16;
        }

        let data = Box::new(Specific {
            tile: OBJECT_ROTATINGCYLINDER,
            current_frame: 0,
            num_frames: 5,
            lives: 10,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.hero_data.is_null());
        let hero_data = unsafe { &mut (*p.hero_data) };

        hero_data.health.kill();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        if specific.lives > 0 {
            specific.current_frame += 1;
            specific.current_frame %= specific.num_frames;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_blit(
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
        let tile = tilecache
            .get_tile(specific.tile + specific.current_frame)
            .unwrap();

        for _ in 0..general.position.h as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
            destrect.y += TILE_HEIGHT as i16;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_shot(
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

        specific.lives -= 1;
        if specific.lives > 0 {
            actor_queue.push_particle_firework(
                general.position.x as u16 + general.position.w / 2,
                general.position.y as u16 + general.position.h / 2,
                4,
            );
        } else {
            // TODO: add removal animation (destroyed body)
            general.is_alive = false;
            hero_data.score.add(20000);
            actor_queue.push_particle_firework(
                general.position.x as u16 + general.position.w / 2,
                general.position.y as u16 + general.position.h / 2,
                20,
            );
            actor_queue.push_back(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16 + general.position.h / 2
                    - TILE_HEIGHT as u16,
            );
            actor_queue.push_back(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16 + general.position.h / 2,
            );
        }
    }
}
