use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::super::HorizontalDirection;
use super::super::LevelData;
use super::{ActorData, ActorInterface, ActorQueue, ActorType};
use crate::{
    ANIMATION_ROBOT, HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    touching_hero: bool,
}

impl ActorInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_ROBOT,
            current_frame: 0,
            num_frames: 3,
            touching_hero: false,
        }
    }

    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = true;
        self.touching_hero = true;
    }

    fn hero_touch_end(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = false;
        self.touching_hero = false;
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if !level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            // In the air, falling down.
            general.position.y += HALFTILE_HEIGHT as i16;
        } else {
            // On the floor, walking.
            if self.current_frame == 0 {
                let mut direction = match self.direction {
                    HorizontalDirection::Left => -1,
                    HorizontalDirection::Right => 2,
                    HorizontalDirection::Center => unreachable!(),
                };
                // Check if the place next to the bot is free
                if !level_data.solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize/ TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT
            ) &&
            // Check if the tile below this free place is solid
            level_data.solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT
            ) {
                    if direction == 2 {
                        direction = 1;
                    }
                    general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                } else {
                    self.direction =
                        if self.direction == HorizontalDirection::Left {
                            HorizontalDirection::Right
                        } else {
                            HorizontalDirection::Left
                        };
                    if direction == 2 {
                        direction = 1
                    };
                    direction *= -1;
                    general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                }
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
        let tile = tilecache.get_tile(self.tile as usize).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        hero_data.score.add(100);
        if self.touching_hero {
            general.hurts_hero = false;
            self.touching_hero = false;
        }
        actor_queue.push_back(
            ActorType::RobotDisappearing,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
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
    pub extern "C" fn fn_level_actor_function_robot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_robot_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
