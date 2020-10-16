use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{ANIMATION_BOMB, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug, PartialEq)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    counter: usize,
    explode_left: bool,
    explode_right: bool,
    explode_threshold: usize,
    num_flames: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    Specific {
        tile: ANIMATION_BOMB,
        current_frame: 0,
        num_frames: 2,
        counter: 0,
        explode_left: true,
        explode_right: true,
        explode_threshold: 12,
        num_flames: 4,
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
    specific.current_frame %= specific.num_frames;

    specific.counter += 1;

    if specific.counter < specific.explode_threshold {
    } else if specific.counter
        < specific.explode_threshold + specific.num_flames
    {
        let distance = specific.counter - specific.explode_threshold;
        if specific.explode_left {
            // explode to the left if possible
            let space_is_free = !level_data.solids.get(
                general.position.x as usize / TILE_WIDTH - distance,
                general.position.y as usize / TILE_HEIGHT,
            );
            let space_has_solid_below = level_data.solids.get(
                general.position.x as usize / TILE_WIDTH - distance,
                general.position.y as usize / TILE_HEIGHT + 1,
            );
            if space_is_free && space_has_solid_below {
                actor_queue.push_back(
                    ActorType::BombFire,
                    general.position.x as u16
                        - distance as u16 * TILE_WIDTH as u16,
                    general.position.y as u16,
                );
            } else {
                specific.explode_left = false;
            }
        }
        if specific.explode_right {
            // explode to the right if possible
            let space_is_free = !level_data.solids.get(
                general.position.x as usize / TILE_WIDTH + distance,
                general.position.y as usize / TILE_HEIGHT,
            );
            let space_has_solid_below = level_data.solids.get(
                general.position.x as usize / TILE_WIDTH + distance,
                general.position.y as usize / TILE_HEIGHT + 1,
            );
            if space_is_free && space_has_solid_below {
                actor_queue.push_back(
                    ActorType::BombFire,
                    general.position.x as u16
                        + distance as u16 * TILE_WIDTH as u16,
                    general.position.y as u16,
                );
            } else {
                specific.explode_right = false;
            }
        }
    } else {
        general.is_alive = false;
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    if specific.counter < specific.explode_threshold {
        tilecache
            .get_tile(specific.tile + specific.current_frame)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
