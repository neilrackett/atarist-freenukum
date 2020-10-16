use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorMessageType, ActorQueue};
use crate::{OBJECT_LASERBEAM, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = false;

    Specific {
        tile: OBJECT_LASERBEAM,
        current_frame: 0,
        num_frames: 4,
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
    tilecache
        .get_tile(specific.tile + specific.current_frame)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(general.position));
}

fn receive_message(
    general: &mut ActorData,
    _specific: &mut Specific,
    message: ActorMessageType,
    _hero_data: &mut HeroData,
    level_data: &mut LevelData,
) {
    if message != ActorMessageType::OpenDoor {
        return;
    }
    let x = general.position.x as usize / TILE_WIDTH;
    let y = general.position.y as usize / TILE_HEIGHT;
    level_data.solids.set(x, y, false);
    general.is_alive = false;
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorReceiveMessageParams,
    };
    use super::Specific;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        p.call(super::receive_message);
    }
}
