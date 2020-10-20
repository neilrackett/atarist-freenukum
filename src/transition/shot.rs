use super::geometry::Geometry;
use super::hero::HeroData;
use super::level::actor::{ActorQueue, ActorType};
use super::level::LevelData;
use super::tilecache::TileCache;
use super::HorizontalDirection;
use crate::{HALFTILE_WIDTH, OBJECT_SHOT, TILE_HEIGHT, TILE_WIDTH};

pub struct Shot {
    position: Geometry,
    is_alive: bool,
    direction: HorizontalDirection,
    counter: usize,
    countdown: usize,
}

impl Shot {
    pub fn new(x: i16, y: i16, direction: HorizontalDirection) -> Self {
        let w = 4;
        let h = TILE_HEIGHT as u16 - 4;
        Shot {
            position: Geometry {
                x: x + HALFTILE_WIDTH as i16 - w as i16 / 2,
                y: y + TILE_HEIGHT as i16 - h as i16,
                w,
                h,
            },
            is_alive: true,
            direction,
            counter: 0,
            countdown: 2,
        }
    }

    /// Returns whether the shot is still alive after acting.
    pub fn act(
        &mut self,
        hero_data: &mut HeroData,
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
    ) -> bool {
        self.counter += 1;
        self.counter %= 4;

        if self.countdown == 1 {
            self.is_alive = false;
            self.countdown -= 1;
        }

        if self.countdown == 2 {
            let distance = match self.direction {
                HorizontalDirection::Left => -(HALFTILE_WIDTH as i16),
                HorizontalDirection::Right => HALFTILE_WIDTH as i16,
                _ => unreachable!(),
            };

            // we only push half of the distance, but do it twice, so that
            // also the intermediate position gets covered, not just the
            // end position.
            self.push(hero_data, level_data, distance, actor_queue);
            self.push(hero_data, level_data, distance, actor_queue);
        }
        self.is_alive
    }

    pub fn blit(
        &self,
        target: &mut transdl::video::Surface,
        tilecache: &TileCache,
        draw_collision_bounds: bool,
    ) {
        if self.is_alive {
            let mut destrect = self.position;
            destrect.x += destrect.w as i16 / 2 - HALFTILE_WIDTH as i16;
            destrect.w = TILE_WIDTH as u16;

            tilecache
                .get_tile(OBJECT_SHOT + self.counter)
                .unwrap()
                .blit_to_sdl_surface(None, target, Some(destrect));
            if draw_collision_bounds {
                let color =
                    crate::collision_bounds_color(&target.format());
                self.position.draw_outline(target, color);
            }
        }
    }

    fn push(
        &mut self,
        hero_data: &mut HeroData,
        level_data: &mut LevelData,
        offset: i16,
        actor_queue: &mut ActorQueue,
    ) {
        if self.countdown == 2 {
            self.position.x += offset;
            for actor in level_data.actors.iter_mut() {
                if actor.can_get_shot()
                    && self.position.touches(actor.position())
                {
                    actor.shot(
                        &mut level_data.solids,
                        &mut level_data.tiles,
                        actor_queue,
                        hero_data,
                    );
                    if !actor.is_alive() {
                        self.countdown = 1;
                    }
                }
            }
        }
        if self.countdown == 2 {
            if level_data.solids.collides(self.position) {
                self.countdown = 1;
                actor_queue.push_back(
                    ActorType::Explosion,
                    self.position.x as u16 + self.position.w / 2
                        - HALFTILE_WIDTH as u16,
                    self.position.y as u16,
                );
            }
        }
    }

    pub fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }
}

pub mod ffi {
    pub type FnShot = super::Shot;

    use super::super::geometry::ffi::FnGeometry;
    use super::super::hero::ffi::FnHeroData;
    use super::super::level::actor::ffi::FnLevelActorQueue;
    use super::super::level::ffi::FnLevelData;
    use super::super::tilecache::ffi::FnTileCache;
    use super::HorizontalDirection;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_shot_create(
        x: i16,
        y: i16,
        direction: HorizontalDirection,
    ) -> *mut FnShot {
        Box::into_raw(Box::new(FnShot::new(x, y, direction)))
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_free(ptr: *mut FnShot) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_get_position(
        shot: *const FnShot,
    ) -> FnGeometry {
        assert!(!shot.is_null());
        let shot: &FnShot = unsafe { &(*shot) };
        shot.position
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_blit(
        ptr: *const FnShot,
        target: *mut SDL_Surface,
        tilecache: *const FnTileCache,
        draw_collision_bounds: bool,
    ) {
        assert!(!ptr.is_null());
        let d: &FnShot = unsafe { &(*ptr) };

        assert!(!target.is_null());
        let mut target = Surface { raw: target };

        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        d.blit(&mut target, tilecache, draw_collision_bounds);
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_push(
        ptr: *mut FnShot,
        hero_data: *mut FnHeroData,
        level_data: *mut FnLevelData,
        offset: i16,
        actor_queue: *mut FnLevelActorQueue,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnShot = unsafe { &mut (*ptr) };

        assert!(!hero_data.is_null());
        let hero_data: &mut FnHeroData = unsafe { &mut (*hero_data) };

        assert!(!level_data.is_null());
        let level_data: &mut FnLevelData = unsafe { &mut (*level_data) };

        assert!(!actor_queue.is_null());
        let actor_queue: &mut FnLevelActorQueue =
            unsafe { &mut (*actor_queue) };

        d.push(hero_data, level_data, offset, actor_queue);
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_act(
        ptr: *mut FnShot,
        hero_data: *mut FnHeroData,
        level_data: *mut FnLevelData,
        actor_queue: *mut FnLevelActorQueue,
    ) -> bool {
        assert!(!ptr.is_null());
        let d: &mut FnShot = unsafe { &mut (*ptr) };

        assert!(!hero_data.is_null());
        let hero_data: &mut FnHeroData = unsafe { &mut (*hero_data) };

        assert!(!level_data.is_null());
        let level_data: &mut FnLevelData = unsafe { &mut (*level_data) };

        assert!(!actor_queue.is_null());
        let actor_queue: &mut FnLevelActorQueue =
            unsafe { &mut (*actor_queue) };

        d.act(hero_data, level_data, actor_queue)
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_set_is_alive(
        ptr: *mut FnShot,
        is_alive: bool,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnShot = unsafe { &mut (*ptr) };

        d.set_is_alive(is_alive);
    }
}
