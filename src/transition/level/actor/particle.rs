struct Specific {
    tile: usize,
    countdown: usize,
    hspeed: i16,
    vspeed: i16,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{
        HALFTILE_HEIGHT, HALFTILE_WIDTH, OBJECT_SPARK_BLUE,
        OBJECT_SPARK_GREEN, OBJECT_SPARK_PINK, OBJECT_SPARK_WHITE,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_particle_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.is_in_foreground = true;
        general.position.w = HALFTILE_WIDTH as u16;
        general.position.h = HALFTILE_HEIGHT as u16;

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let vspeed = rng.gen_range(-12, 5);
        let hspeed = rng.gen_range(-8, 9);

        let tile = match general.actor_type {
            ActorType::ParticlePink => OBJECT_SPARK_PINK,
            ActorType::ParticleBlue => OBJECT_SPARK_BLUE,
            ActorType::ParticleWhite => OBJECT_SPARK_WHITE,
            ActorType::ParticleGreen => OBJECT_SPARK_GREEN,
            _ => unreachable!(),
        };

        let data = Box::new(Specific {
            tile,
            countdown: 20,
            hspeed,
            vspeed,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_particle_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_particle_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*(p.general)) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        if specific.countdown > 0 {
            specific.countdown -= 1;
            general.position.x += specific.hspeed;
            general.position.y += specific.vspeed;
            specific.vspeed += 2;
        } else {
            general.is_alive = false;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_particle_blit(
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

        let tile = tilecache.get_tile((specific.tile) as usize).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
