use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::super::HorizontalDirection;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    ANIMATION_CARBOT, HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: usize,
    touching_hero: bool,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16 * 2;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = true;

    Specific {
        orientation: HorizontalDirection::Left,
        tile: ANIMATION_CARBOT,
        current_frame: 0,
        num_frames: 4,
        was_shot: 0,
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
    actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    specific.current_frame += 1;
    specific.current_frame %= specific.num_frames;

    if specific.was_shot == 2 {
        general.is_alive = false;
        actor_queue.push_back(
            ActorType::Explosion,
            general.position.x as u16 + HALFTILE_WIDTH as u16,
            general.position.y as u16,
        );
        actor_queue.push_particle_firework(
            general.position.x as u16,
            general.position.y as u16,
            4,
        );
        hero_data.score.add(2500);
    } else {
        if level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) && !level_data.solids.get(
            general.position.x as usize / TILE_WIDTH + 1,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            // still in the air, falling down
            general.position.y += HALFTILE_HEIGHT as i16;
        } else {
            // on the floor, walking
            let mut direction = match specific.orientation {
                HorizontalDirection::Left => -1,
                HorizontalDirection::Right => 4,
                HorizontalDirection::Center => unreachable!(),
            };

            if !level_data.solids.get(
                // check if the place next ot the bot is free
                (general.position.x as isize
                    + direction * HALFTILE_WIDTH as isize)
                    as usize
                    / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT,
            ) && level_data.solids.get(
                // check if the tile below is solid
                (general.position.x as isize
                    + direction * HALFTILE_WIDTH as isize)
                    as usize
                    / TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT,
            ) {
                if direction > 0 {
                    direction = 1;
                }
                general.position.x += (direction as f64
                    * HALFTILE_WIDTH as f64
                    * 0.7) as i16;
            } else {
                // reached the end, turning around
                specific.orientation = match specific.orientation {
                    HorizontalDirection::Left => {
                        HorizontalDirection::Right
                    }
                    HorizontalDirection::Right => {
                        HorizontalDirection::Left
                    }
                    HorizontalDirection::Center => unreachable!(),
                };
                if direction > 0 {
                    direction = 1;
                }
                direction *= -1;
                general.position.x +=
                    direction as i16 * HALFTILE_WIDTH as i16;
                let tile = specific.tile as isize + 4 * direction;
                specific.tile = tile as usize;

                if direction > 0 {
                    actor_queue.push_back(
                        ActorType::HostileShotRight,
                        general.position.x as u16,
                        general.position.y as u16 - 6,
                    );
                } else {
                    actor_queue.push_back(
                        ActorType::HostileShotLeft,
                        general.position.x as u16,
                        general.position.y as u16 - 6,
                    );
                }
            }
        }
    }
    if specific.was_shot == 1 {
        // create steam clouds
        if specific.current_frame == 0 {
            actor_queue.push_back(
                ActorType::Steam,
                general.position.x as u16 + HALFTILE_WIDTH as u16,
                general.position.y as u16 - TILE_HEIGHT as u16,
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
    let tile = tilecache
        .get_tile(specific.tile + (specific.current_frame / 2) * 2)
        .unwrap();
    tile.blit_to_sdl_surface(None, target, Some(destrect));

    let tile = tilecache
        .get_tile(specific.tile + (specific.current_frame / 2) * 2 + 1)
        .unwrap();
    destrect.x += TILE_WIDTH as i16;
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

fn shot(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    if specific.was_shot == 1 && specific.touching_hero {
        general.hurts_hero = false;
        specific.touching_hero = false;
    }
    if specific.was_shot != 2 {
        specific.was_shot += 1;
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
        FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_tankbot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_tankbot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_tankbot_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_tankbot_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call(super::hero_touch_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_tankbot_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_tankbot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_tankbot_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }
}
