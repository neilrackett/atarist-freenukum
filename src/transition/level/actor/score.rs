struct Specific {
    tile: usize,
    countdown: usize,
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
    };
    use super::super::ActorType;
    use super::Specific;
    use crate::{
        NUMBER_100, NUMBER_1000, NUMBER_10000, NUMBER_200, NUMBER_2000,
        NUMBER_500, NUMBER_5000, NUMBER_BONUS_1_LEFT,
        NUMBER_BONUS_1_RIGHT, NUMBER_BONUS_2_LEFT, NUMBER_BONUS_2_RIGHT,
        NUMBER_BONUS_3_LEFT, NUMBER_BONUS_3_RIGHT, NUMBER_BONUS_4_LEFT,
        NUMBER_BONUS_4_RIGHT, NUMBER_BONUS_5_LEFT, NUMBER_BONUS_5_RIGHT,
        NUMBER_BONUS_6_LEFT, NUMBER_BONUS_6_RIGHT, NUMBER_BONUS_7_LEFT,
        NUMBER_BONUS_7_RIGHT, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.is_in_foreground = true;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let tile = match general.actor_type {
            ActorType::Score100 => NUMBER_100,
            ActorType::Score200 => NUMBER_200,
            ActorType::Score500 => NUMBER_500,
            ActorType::Score1000 => NUMBER_1000,
            ActorType::Score2000 => NUMBER_2000,
            ActorType::Score5000 => NUMBER_5000,
            ActorType::Score10000 => NUMBER_10000,
            ActorType::ScoreBonus1Left => NUMBER_BONUS_1_LEFT,
            ActorType::ScoreBonus1Right => NUMBER_BONUS_1_RIGHT,
            ActorType::ScoreBonus2Left => NUMBER_BONUS_2_LEFT,
            ActorType::ScoreBonus2Right => NUMBER_BONUS_2_RIGHT,
            ActorType::ScoreBonus3Left => NUMBER_BONUS_3_LEFT,
            ActorType::ScoreBonus3Right => NUMBER_BONUS_3_RIGHT,
            ActorType::ScoreBonus4Left => NUMBER_BONUS_4_LEFT,
            ActorType::ScoreBonus4Right => NUMBER_BONUS_4_RIGHT,
            ActorType::ScoreBonus5Left => NUMBER_BONUS_5_LEFT,
            ActorType::ScoreBonus5Right => NUMBER_BONUS_5_RIGHT,
            ActorType::ScoreBonus6Left => NUMBER_BONUS_6_LEFT,
            ActorType::ScoreBonus6Right => NUMBER_BONUS_6_RIGHT,
            ActorType::ScoreBonus7Left => NUMBER_BONUS_7_LEFT,
            ActorType::ScoreBonus7Right => NUMBER_BONUS_7_RIGHT,
            _ => {
                unreachable!(
                    "Actor type {:?} added as an score \
                    which is not a score id",
                    general.actor_type
                );
            }
        };

        let data = Box::new(Specific {
            tile,
            countdown: 40,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*(p.general)) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        specific.countdown -= 1;
        general.position.y -= 1;
        if specific.countdown == 0
            || general.position.y == -(TILE_HEIGHT as i16)
        {
            general.is_alive = false;
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_blit(
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
}
