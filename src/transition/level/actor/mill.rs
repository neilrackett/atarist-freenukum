use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{OBJECT_ROTATINGCYLINDER, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    lives: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        while general.position.y > 0
            && !level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT - 1,
            )
        {
            general.position.y -= TILE_HEIGHT as i16;
            general.position.h += TILE_HEIGHT as u16;
        }

        Specific {
            tile: OBJECT_ROTATINGCYLINDER,
            current_frame: 0,
            num_frames: 5,
            lives: 10,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        _general: &mut ActorData,
        _actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        hero_data.health.kill();
    }

    fn act(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        if self.lives > 0 {
            self.current_frame += 1;
            self.current_frame %= self.num_frames;
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
        let tile =
            tilecache.get_tile(self.tile + self.current_frame).unwrap();

        for _ in 0..general.position.h as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, target, Some(destrect));
            destrect.y += TILE_HEIGHT as i16;
        }
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        self.lives -= 1;
        if self.lives > 0 {
            actor_queue.push_particle_firework(
                general.position.x as u16 + general.position.w / 2,
                general.position.y as u16 + general.position.h / 2,
                4,
            );
        } else {
            // TODO: add removal animation (destroyed body)
            general.is_alive = false;
            hero_data.score.add(20000);
            actor_queue.push_particle_firework(
                general.position.x as u16 + general.position.w / 2,
                general.position.y as u16 + general.position.h / 2,
                20,
            );
            actor_queue.push_back(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16 + general.position.h / 2
                    - TILE_HEIGHT as u16,
            );
            actor_queue.push_back(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16 + general.position.h / 2,
            );
        }
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_mill_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
