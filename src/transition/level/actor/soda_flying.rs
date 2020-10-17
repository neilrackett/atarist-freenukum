use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorInterface, ActorQueue, ActorType};
use crate::{ANIMATION_SODAFLY, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {}

impl ActorInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {}
    }

    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        hero_data.score.add(1000);
        actor_queue.push_back(
            ActorType::Score1000,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        general.position.y -= HALFTILE_HEIGHT as i16;
        if level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT,
        ) {
            actor_queue.push_back(
                ActorType::Explosion,
                general.position.x as u16,
                general.position.y as u16,
            );
            general.is_alive = false;
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = tilecache
            .get_tile(
                ANIMATION_SODAFLY
                    + ((general.position.y as usize / HALFTILE_HEIGHT)
                        % 4),
            )
            .unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorHeroTouchStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_soda_flying_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
