use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::super::HorizontalDirection;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    ANIMATION_FIREWHEEL_OFF, ANIMATION_FIREWHEEL_ON, HALFTILE_HEIGHT,
    HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: usize,
    fire_is_on: bool,
    counter: usize,
    touching_hero: bool,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    Specific {
        direction: HorizontalDirection::Left,
        tile: ANIMATION_FIREWHEEL_OFF,
        counter: 0,
        touching_hero: false,
        current_frame: 0,
        num_frames: 4,
        was_shot: 0,
        fire_is_on: false,
    }
}

fn hero_touch_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    if specific.was_shot < 2 {
        specific.touching_hero = true;
        general.hurts_hero = true;
    }
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
    actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
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
            8,
        );
        hero_data.score.add(2500);
    } else {
        specific.counter += 1;
        if specific.counter % 2 == 1 {
            specific.current_frame += 1;
            specific.current_frame %= specific.num_frames;
        }

        if specific.counter == 50 {
            specific.counter = 0;
            specific.fire_is_on = !specific.fire_is_on;
            if specific.fire_is_on {
                specific.tile = ANIMATION_FIREWHEEL_ON;
            } else {
                specific.tile = ANIMATION_FIREWHEEL_OFF;
            }
        }

        let direction = match specific.direction {
            HorizontalDirection::Left => -1,
            HorizontalDirection::Right => 1,
            HorizontalDirection::Center => unreachable!(),
        };

        if !level_data.solids.push_rect_standing_on_ground(
            &mut general.position,
            direction * HALFTILE_WIDTH as i16 / 2,
            HALFTILE_HEIGHT as u8,
        ) {
            // push was not successful, so we reverse the direction
            specific.direction = match specific.direction {
                HorizontalDirection::Left => HorizontalDirection::Right,
                HorizontalDirection::Right => HorizontalDirection::Left,
                HorizontalDirection::Center => unreachable!(),
            };
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
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let mut destrect = general.position;
    destrect.x = destrect.x + destrect.w as i16 / 2 - TILE_WIDTH as i16;
    destrect.y -= TILE_HEIGHT as i16;
    destrect.w = TILE_WIDTH as u16 * 2;

    tilecache
        .get_tile(specific.tile + specific.current_frame * 4)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.x += TILE_WIDTH as i16;
    tilecache
        .get_tile(specific.tile + specific.current_frame * 4 + 1)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.x -= TILE_WIDTH as i16;
    destrect.y += TILE_HEIGHT as i16;
    tilecache
        .get_tile(specific.tile + specific.current_frame * 4 + 2)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
    destrect.x += TILE_WIDTH as i16;
    tilecache
        .get_tile(specific.tile + specific.current_frame * 4 + 3)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(destrect));
}

fn shot(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    if !specific.fire_is_on {
        if specific.was_shot == 1 && specific.touching_hero {
            general.hurts_hero = false;
            specific.touching_hero = false;
        }
        if specific.was_shot != 2 {
            specific.was_shot += 1;
        }
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
    pub extern "C" fn fn_level_actor_function_firewheelbot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_firewheelbot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_firewheelbot_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_firewheelbot_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call(super::hero_touch_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_firewheelbot_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_firewheelbot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_firewheelbot_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }
}
