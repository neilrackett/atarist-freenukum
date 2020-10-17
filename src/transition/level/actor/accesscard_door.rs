use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorMessageType,
    ActorQueue,
};
use crate::{OBJECT_LASERBEAM, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        Specific {
            tile: OBJECT_LASERBEAM,
            current_frame: 0,
            num_frames: 4,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        tilecache
            .get_tile(self.tile + self.current_frame)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }

    fn receive_message(
        &mut self,
        general: &mut ActorData,
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
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_door_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
