use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{OBJECT_HOSTILESHOT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    touching_hero: bool,
    current_frame: usize,
    num_frames: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    let tile = match general.actor_type {
        ActorType::HostileShotLeft => OBJECT_HOSTILESHOT,
        ActorType::HostileShotRight => OBJECT_HOSTILESHOT + 2,
        _ => unreachable!(
            "Passed actor type {:?} to hostile shot actor \
                which is not a shot actor id",
            general.actor_type
        ),
    };

    Specific {
        tile,
        touching_hero: false,
        current_frame: 0,
        num_frames: 2,
    }
}

fn hero_touch_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    specific.touching_hero = true;
    general.hurts_hero = true;
}

fn hero_touch_end(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
) {
    specific.touching_hero = false;
    general.hurts_hero = false;
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    let offset = match general.actor_type {
        ActorType::HostileShotLeft => -(TILE_WIDTH as i16),
        ActorType::HostileShotRight => TILE_WIDTH as i16,
        _ => unreachable!(),
    };
    general.position.x += offset;

    if level_data.solids.get(
        general.position.x as usize / TILE_WIDTH,
        general.position.y as usize / TILE_HEIGHT,
    ) {
        general.is_alive = false;
        if specific.touching_hero {
            general.hurts_hero = false;
        }
    }

    specific.current_frame += 1;
    specific.current_frame %= specific.num_frames;
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = tilecache
        .get_tile(specific.tile + specific.current_frame)
        .unwrap();
    let destrect = general.position;
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call(super::hero_touch_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_hostileshot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
