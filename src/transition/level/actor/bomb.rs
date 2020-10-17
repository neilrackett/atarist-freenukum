use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{ANIMATION_BOMB, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug, PartialEq)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    counter: usize,
    explode_left: bool,
    explode_right: bool,
    explode_threshold: usize,
    num_flames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {
            tile: ANIMATION_BOMB,
            current_frame: 0,
            num_frames: 2,
            counter: 0,
            explode_left: true,
            explode_right: true,
            explode_threshold: 12,
            num_flames: 4,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        self.counter += 1;

        if self.counter < self.explode_threshold {
        } else if self.counter < self.explode_threshold + self.num_flames {
            let distance = self.counter - self.explode_threshold;
            if self.explode_left {
                // explode to the left if possible
                let space_is_free = !level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH - distance,
                    general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH - distance,
                    general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    actor_queue.push_back(
                        ActorType::BombFire,
                        general.position.x as u16
                            - distance as u16 * TILE_WIDTH as u16,
                        general.position.y as u16,
                    );
                } else {
                    self.explode_left = false;
                }
            }
            if self.explode_right {
                // explode to the right if possible
                let space_is_free = !level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH + distance,
                    general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH + distance,
                    general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    actor_queue.push_back(
                        ActorType::BombFire,
                        general.position.x as u16
                            + distance as u16 * TILE_WIDTH as u16,
                        general.position.y as u16,
                    );
                } else {
                    self.explode_right = false;
                }
            }
        } else {
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
        if self.counter < self.explode_threshold {
            tilecache
                .get_tile(self.tile + self.current_frame)
                .unwrap()
                .blit_to_sdl_surface(None, target, Some(general.position));
        }
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_bomb_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
