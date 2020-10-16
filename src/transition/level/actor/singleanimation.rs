use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    ANIMATION_BOMBFIRE, ANIMATION_EXPLOSION, ANIMATION_ROBOT,
    OBJECT_DUSTCLOUD, OBJECT_STEAM, TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    can_hurt_hero: bool,
    replaced_by: Option<ActorType>,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.is_in_foreground = false;
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    let (tile, num_frames, can_hurt_hero, replaced_by) =
        match general.actor_type {
            ActorType::BombFire => (ANIMATION_BOMBFIRE, 6, true, None),
            ActorType::Explosion => (ANIMATION_EXPLOSION, 6, false, None),
            ActorType::DustCloud => (OBJECT_DUSTCLOUD, 5, false, None),
            ActorType::Steam => (OBJECT_STEAM, 5, false, None),
            ActorType::RobotDisappearing => {
                (ANIMATION_ROBOT + 3, 7, false, Some(ActorType::Explosion))
            }
            _ => {
                unreachable!(
                    "Actor type {:?} added as an animation \
                    which is not an animation id",
                    general.actor_type
                );
            }
        };

    Specific {
        tile,
        current_frame: 0,
        num_frames,
        can_hurt_hero,
        replaced_by,
    }
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    specific.current_frame += 1;
    if specific.current_frame == specific.num_frames {
        general.is_alive = false;
        if let Some(successor) = specific.replaced_by {
            actor_queue.push_back(
                successor,
                general.position.x as u16,
                general.position.y as u16,
            );
        }
    }
}

fn hero_touch_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    if specific.can_hurt_hero {
        general.hurts_hero = true;
    }
}

fn hero_touch_end(
    general: &mut ActorData,
    _specific: &mut Specific,
    _hero_data: &mut HeroData,
) {
    general.hurts_hero = false;
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = tilecache
        .get_tile((specific.tile + specific.current_frame) as usize)
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
    pub extern "C" fn fn_level_actor_function_singleanimation_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call(super::hero_touch_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call(super::hero_touch_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_singleanimation_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
