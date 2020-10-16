use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{ANIMATION_FAN, HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    running: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.y -= TILE_HEIGHT as i16;
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16 * 2;

    Specific {
        tile: ANIMATION_FAN,
        current_frame: 0,
        num_frames: 4,
        running: 10,
    }
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    match specific.running {
        0 => {}
        1 => {
            specific.current_frame += 1;
        }
        2 => {}
        3 => {}
        4 => {}
        5 => {
            specific.current_frame += 1;
        }
        6 => {}
        7 => {}
        8 => {
            specific.current_frame += 1;
        }
        9 => {}
        10 => {
            specific.current_frame += 1;
        }
        _ => unreachable!(),
    }
    specific.current_frame %= specific.num_frames;
    if specific.running < 10 && specific.running > 0 {
        specific.running -= 1;
    } else if specific.running == 10 {
        if hero_data
            .position
            .geometry
            .overlaps_vertically(general.position)
        {
            let mut hdistance = hero_data
                .position
                .geometry
                .horizontal_distance(general.position);

            let fan_direction = match general.actor_type {
                ActorType::FanLeft => -1,
                ActorType::FanRight => 1,
                _ => unreachable!(),
            };
            if (fan_direction > 0 && hdistance > 0)
                || (fan_direction < 0 && hdistance < 0)
            {
                return;
            }

            if hdistance == 0 {
                hdistance = HALFTILE_WIDTH as i32 * fan_direction;
            }

            if hdistance.abs() < 8 * HALFTILE_WIDTH as i32 {
                hero_data.position.push_horizontally(
                    &level_data.solids,
                    fan_direction as i16 * TILE_WIDTH as i16,
                );
            }
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
    tilecache
        .get_tile(specific.tile + specific.current_frame * 2)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.y += TILE_HEIGHT as i16;
    tilecache
        .get_tile(specific.tile + specific.current_frame * 2 + 1)
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
    specific.running = 9;
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
        FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_fan_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }
}
