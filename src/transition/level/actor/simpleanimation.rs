struct Specific {
    tile: u16,
    current_frame: u16,
    num_frames: u16,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{
        ANIMATION_BROKENWALLBG, ANIMATION_STONEWINDOWBG,
        ANIMATION_WINDOWBG, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.is_in_foreground = false;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let (tile, num_frames) = match general.actor_type {
            ActorType::TextOnScreenBackground => (0x0004, 4),
            ActorType::HighVoltageFlashBackground => (0x0008, 4),
            ActorType::RedFlashlightBackground => (0x000C, 4),
            ActorType::BlueFlashlightBackground => (0x0010, 4),
            ActorType::KeypanelBackground => (0x0014, 4),
            ActorType::RedRotationLightBackground => (0x0018, 4),
            ActorType::UpArrowBackground => (0x001C, 4),
            ActorType::BlueLightBackground1 => (0x0020, 4),
            ActorType::BlueLightBackground2 => (0x0021, 4),
            ActorType::BlueLightBackground3 => (0x0022, 4),
            ActorType::BlueLightBackground4 => (0x0023, 4),
            ActorType::GreenPoisonBackground => (0x0028, 4),
            ActorType::LavaBackground => (0x002C, 4),
            ActorType::WindowLeftBackground => {
                (ANIMATION_WINDOWBG as u16, 1)
            }
            ActorType::WindowRightBackground => {
                (ANIMATION_WINDOWBG as u16 + 1, 1)
            }
            ActorType::StoneWindowBackground => {
                (ANIMATION_STONEWINDOWBG as u16, 1)
            }
            ActorType::BrokenWallBackground => {
                (ANIMATION_BROKENWALLBG as u16, 1)
            }
            _ => {
                unreachable!(
                    "Actor type {:?} added as an animation \
                    which is not an animation id",
                    general.actor_type
                );
            }
        };

        let data = Box::new(Specific {
            tile,
            current_frame: 0,
            num_frames,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.specific.is_null());
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        specific.current_frame += 1;
        specific.current_frame %= specific.num_frames;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_blit(
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

        let tile = tilecache
            .get_tile((specific.tile + specific.current_frame) as usize)
            .unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }
}
