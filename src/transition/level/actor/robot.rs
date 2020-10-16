use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::super::HorizontalDirection;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    ANIMATION_ROBOT, HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    touching_hero: bool,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = true;

    Specific {
        direction: HorizontalDirection::Left,
        tile: ANIMATION_ROBOT,
        current_frame: 0,
        num_frames: 3,
        touching_hero: false,
    }
}

fn hero_touch_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    general.hurts_hero = true;
    specific.touching_hero = true;
}

fn hero_touch_end(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
) {
    general.hurts_hero = false;
    specific.touching_hero = false;
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    specific.current_frame += 1;
    specific.current_frame %= specific.num_frames;

    if !level_data.solids.get(
        general.position.x as usize / TILE_WIDTH,
        general.position.y as usize / TILE_HEIGHT + 1,
    ) {
        // In the air, falling down.
        general.position.y += HALFTILE_HEIGHT as i16;
    } else {
        // On the floor, walking.
        if specific.current_frame == 0 {
            let mut direction = match specific.direction {
                HorizontalDirection::Left => -1,
                HorizontalDirection::Right => 2,
                HorizontalDirection::Center => unreachable!(),
            };
            // Check if the place next to the bot is free
            if !level_data.solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize/ TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT
            ) &&
            // Check if the tile below this free place is solid
            level_data.solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT
            ) {
                if direction == 2 {
                    direction = 1;
                }
                general.position.x +=
                    direction as i16 * HALFTILE_WIDTH as i16;
            } else {
                specific.direction =
                    if specific.direction == HorizontalDirection::Left {
                        HorizontalDirection::Right
                    } else {
                        HorizontalDirection::Left
                    };
                if direction == 2 {
                    direction = 1
                };
                direction *= -1;
                general.position.x +=
                    direction as i16 * HALFTILE_WIDTH as i16;
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
    let tile = tilecache.get_tile(specific.tile as usize).unwrap();
    let destrect = general.position;
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

fn shot(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    hero_data.score.add(100);
    if specific.touching_hero {
        general.hurts_hero = false;
        specific.touching_hero = false;
    }
    actor_queue.push_back(
        ActorType::RobotDisappearing,
        general.position.x as u16,
        general.position.y as u16,
    );
    general.is_alive = false;
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
        FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call(super::hero_touch_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }
}
