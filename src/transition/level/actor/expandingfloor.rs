use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::ActorMessageType;
use super::{ActorData, ActorInterface, ActorQueue};
use crate::{SOLID_EXPANDINGFLOOR, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    expanding: bool,
    finished: bool,
}

impl ActorInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {
            expanding: false,
            finished: false,
        }
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        if self.expanding {
            let x = (general.position.x as usize
                + general.position.w as usize)
                / TILE_WIDTH;
            let y = general.position.y as usize / TILE_HEIGHT;
            let can_expand = !level_data.solids.get(x, y);
            if can_expand {
                level_data.solids.set(x, y, true);
                general.position.w += TILE_WIDTH as u16;
            } else {
                self.expanding = false;
                self.finished = true;
            }
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile =
            tilecache.get_tile(SOLID_EXPANDINGFLOOR as usize).unwrap();
        let mut destrect = general.position;
        for _ in 0..general.position.w as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }

    fn receive_message(
        &mut self,
        _general: &mut ActorData,
        message: ActorMessageType,
        _hero_data: &mut HeroData,
        _level_data: &mut LevelData,
    ) {
        if message != ActorMessageType::Expand {
            return;
        }
        if !self.expanding && !self.finished {
            self.expanding = true;
        }
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorReceiveMessageParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_expandingfloor_receive_message(
        p: FnLevelActorReceiveMessageParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
