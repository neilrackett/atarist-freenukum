use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{OBJECT_BALLOON, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    destroyed: bool,
    current_frame: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16 * 2;

        Specific {
            destroyed: false,
            current_frame: 0,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        if !self.destroyed {
            general.is_alive = false;
            hero_data.score.add(10000);
            actor_queue.push_back(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16,
            );
        }
    }
    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        self.current_frame %= 9;

        if self.destroyed {
            general.is_alive = false;
        } else {
            general.position.y -= 1;
            if level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_WIDTH,
            ) {
                // balloon bumps against wall
                self.destroyed = true;
                actor_queue.push_back(
                    ActorType::Steam,
                    general.position.x as u16,
                    general.position.y as u16,
                );
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
        let mut destrect = general.position;

        let tile = if self.destroyed {
            OBJECT_BALLOON + 4
        } else {
            OBJECT_BALLOON
        };
        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            target,
            Some(destrect),
        );

        destrect.y += TILE_HEIGHT as i16;
        tilecache
            .get_tile(OBJECT_BALLOON + 1 + self.current_frame / 3)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        self.destroyed = true;
        actor_queue.push_back(
            ActorType::Steam,
            general.position.x as u16,
            general.position.y as u16,
        );
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_balloon_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
