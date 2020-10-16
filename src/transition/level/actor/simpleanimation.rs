use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{
    ANIMATION_BROKENWALLBG, ANIMATION_STONEWINDOWBG, ANIMATION_WINDOWBG,
    TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: u16,
    current_frame: u16,
    num_frames: u16,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.is_in_foreground = false;
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    let (tile, num_frames) = match general.actor_type {
        ActorType::TextOnScreenBackground => (0x0004, 4),
        ActorType::HighVoltageFlashBackground => (0x0008, 4),
        ActorType::RedFlashlightBackground => (0x000C, 4),
        ActorType::BlueFlashlightBackground => (0x0010, 4),
        ActorType::KeypanelBackground => (0x0014, 4),
        ActorType::RedRotationLightBackground => (0x0018, 4),
        ActorType::UpArrowBackground => (0x001C, 4),
        ActorType::BlueLightBackground1 => (0x0020, 4),
        ActorType::BlueLightBackground2 => (0x0021, 4),
        ActorType::BlueLightBackground3 => (0x0022, 4),
        ActorType::BlueLightBackground4 => (0x0023, 4),
        ActorType::GreenPoisonBackground => (0x0028, 4),
        ActorType::LavaBackground => (0x002C, 4),
        ActorType::WindowLeftBackground => (ANIMATION_WINDOWBG as u16, 1),
        ActorType::WindowRightBackground => {
            (ANIMATION_WINDOWBG as u16 + 1, 1)
        }
        ActorType::StoneWindowBackground => {
            (ANIMATION_STONEWINDOWBG as u16, 1)
        }
        ActorType::BrokenWallBackground => {
            (ANIMATION_BROKENWALLBG as u16, 1)
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
    }
}

fn act(
    _general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
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
        .get_tile((specific.tile + specific.current_frame) as usize)
        .unwrap();
    let destrect = general.position;
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_simpleanimation_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
