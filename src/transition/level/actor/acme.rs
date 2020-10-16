use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorInterface, ActorQueue, ActorType};
use crate::{OBJECT_FALLINGBLOCK, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    counter: usize,
    touching_hero: bool,
}

impl ActorInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        Specific {
            tile: OBJECT_FALLINGBLOCK,
            counter: 0,
            touching_hero: false,
        }
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        let hero_geometry = hero_data.position.geometry;

        match self.counter {
            0 => {
                let xl = general.position.x as u16;
                let xr = xl + general.position.w;
                let y = general.position.y;
                let hxl = hero_geometry.x as u16;
                let hxr = hxl + hero_geometry.w;
                let hy = hero_geometry.y;

                if y < hy && xl < hxr && xr > hxl {
                    let mut solid_between = false;
                    for i in (y as usize / TILE_HEIGHT) + 1
                        ..hy as usize / TILE_HEIGHT
                    {
                        let x = xl as usize / TILE_WIDTH;
                        if level_data.solids.get(x, i)
                            || level_data.solids.get(x + 1, i)
                        {
                            solid_between = true;
                            break;
                        }
                    }
                    if !solid_between {
                        self.counter += 1;
                    }
                }
            }
            c if c <= 10 && c % 2 == 0 => {
                general.position.y -= 1;
                self.counter += 1;
            }
            c if c <= 10 && c % 2 == 1 => {
                general.position.y += 1;
                self.counter += 1;
            }
            _ => {
                if level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT + 1,
                ) {
                    actor_queue.push_back(
                        ActorType::Steam,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                    actor_queue.push_particle_firework(
                        general.position.x as u16,
                        general.position.y as u16,
                        4,
                    );
                    general.is_alive = false;
                } else {
                    general.position.y += TILE_HEIGHT as i16;
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
        let mut destrect = general.position;
        tilecache.get_tile(self.tile).unwrap().blit_to_sdl_surface(
            None,
            target,
            Some(destrect),
        );
        destrect.x += TILE_WIDTH as i16;
        tilecache
            .get_tile(self.tile + 1)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        if self.counter > 0 {
            hero_data.score.add(500);
            actor_queue.push_back(
                ActorType::Score500,
                general.position.x as u16,
                general.position.y as u16,
            );
            actor_queue.push_particle_firework(
                general.position.x as u16,
                general.position.y as u16,
                4,
            );

            general.is_alive = false;
        }
    }

    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        if self.counter > 10 && !self.touching_hero {
            self.touching_hero = true;
            general.hurts_hero = true;
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
    pub extern "C" fn fn_level_actor_function_acme_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_acme_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
