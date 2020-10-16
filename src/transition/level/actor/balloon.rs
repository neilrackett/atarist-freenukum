use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{OBJECT_BALLOON, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    destroyed: bool,
    current_frame: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16 * 2;

    Specific {
        destroyed: false,
        current_frame: 0,
    }
}

fn hero_touch_start(
    general: &mut ActorData,
    specific: &mut Specific,
    actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    if !specific.destroyed {
        general.is_alive = false;
        hero_data.score.add(10000);
        actor_queue.push_back(
            ActorType::Score10000,
            general.position.x as u16,
            general.position.y as u16,
        );
    }
}
fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    specific.current_frame += 1;
    specific.current_frame %= 9;

    if specific.destroyed {
        general.is_alive = false;
    } else {
        general.position.y -= 1;
        if level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_WIDTH,
        ) {
            // balloon bumps against wall
            specific.destroyed = true;
            actor_queue.push_back(
                ActorType::Steam,
                general.position.x as u16,
                general.position.y as u16,
            );
        }
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let mut destrect = general.position;

    let tile = if specific.destroyed {
        OBJECT_BALLOON + 4
    } else {
        OBJECT_BALLOON
    };
    tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
        None,
        target,
        Some(destrect),
    );

    destrect.y += TILE_HEIGHT as i16;
    tilecache
        .get_tile(OBJECT_BALLOON + 1 + specific.current_frame / 3)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
}

fn shot(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    specific.destroyed = true;
    actor_queue.push_back(
        ActorType::Steam,
        general.position.x as u16,
        general.position.y as u16,
    );
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }
}
