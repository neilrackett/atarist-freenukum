use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::super::{HorizontalDirection, VerticalDirection};
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    ANIMATION_WALLCRAWLERBOT_LEFT, ANIMATION_WALLCRAWLERBOT_RIGHT,
    TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    direction: VerticalDirection,
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: bool,
    touching_hero: bool,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = true;

    let (tile, orientation) = match general.actor_type {
        ActorType::WallCrawlerBotLeft => {
            (ANIMATION_WALLCRAWLERBOT_LEFT, HorizontalDirection::Left)
        }
        ActorType::WallCrawlerBotRight => {
            (ANIMATION_WALLCRAWLERBOT_RIGHT, HorizontalDirection::Right)
        }
        _ => unreachable!(),
    };

    Specific {
        direction: VerticalDirection::Up,
        orientation,
        tile,
        current_frame: 0,
        num_frames: 4,
        was_shot: false,
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
    let direction = match specific.direction {
        VerticalDirection::Up => 1,
        VerticalDirection::Down => -1,
        VerticalDirection::Center => unreachable!(),
    };
    let orientation = match specific.orientation {
        HorizontalDirection::Left => -1,
        HorizontalDirection::Right => 1,
        HorizontalDirection::Center => unreachable!(),
    };

    if direction > 0 {
        // going up
        specific.current_frame += 1;
        specific.current_frame %= specific.num_frames;

        if
        // bot collides with solid tile
        level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                (general.position.y as usize - 1) / TILE_WIDTH) ||
            // bot has no more wall to stick upon
            !level_data.solids.get(
                (
                    general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (general.position.y as usize - 1) / TILE_HEIGHT)
        {
            general.position.y += 1;
            specific.direction = VerticalDirection::Down;
        } else {
            general.position.y -= 1;
        }
    } else {
        // going down
        if specific.current_frame == 0 {
            specific.current_frame = specific.num_frames;
        }
        specific.current_frame -= 1;

        if
        // bot collides with solid tile
        level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    (
                        general.position.y as usize + TILE_HEIGHT
                    ) / TILE_HEIGHT) ||
            // bot has no more wall to stick upon
            !level_data.solids.get(
                (
                    general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize) as usize /
                TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT)
        {
            general.position.y -= 1;
            specific.direction = VerticalDirection::Up;
        } else {
            general.position.y += 1;
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
    let tile = tilecache
        .get_tile(specific.tile + specific.current_frame)
        .unwrap();
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
    if !specific.was_shot {
        if specific.touching_hero {
            general.hurts_hero = false;
            specific.touching_hero = false;
            specific.current_frame = 0;
        }
        general.is_alive = false;

        hero_data.score.add(100);
        actor_queue.push_back(
            ActorType::Steam,
            general.position.x as u16,
            general.position.y as u16,
        );
        actor_queue.push_back(
            ActorType::Explosion,
            general.position.x as u16,
            general.position.y as u16,
        );
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
    pub extern "C" fn fn_level_actor_function_wallcrawler_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call(super::hero_touch_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }
}
