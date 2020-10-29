use super::geometry::Geometry;
use super::hero::HeroData;
use super::level::actor::{ActorAdder, ActorType};
use super::level::LevelData;
use super::tilecache::TileCache;
use super::HorizontalDirection;
use crate::{
    HALFTILE_WIDTH, LEVELWINDOW_WIDTH, OBJECT_SHOT, TILE_HEIGHT,
    TILE_WIDTH,
};

pub type ShotList = Vec<Shot>;

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
        actor_adder: &mut dyn ActorAdder,
    ) -> bool {
        self.counter += 1;
        self.counter %= 4;

        if self.countdown == 1 {
            self.is_alive = false;
            self.countdown -= 1;
        }

        let x_start = hero_data.position.geometry.x
            - TILE_WIDTH as i16 * LEVELWINDOW_WIDTH as i16 / 2;
        let x_end = hero_data.position.geometry.x
            + hero_data.position.geometry.w as i16
            + TILE_WIDTH as i16 * LEVELWINDOW_WIDTH as i16 / 2;

        if self.countdown == 2 {
            let distance = match self.direction {
                HorizontalDirection::Left => -(HALFTILE_WIDTH as i16),
                HorizontalDirection::Right => HALFTILE_WIDTH as i16,
                _ => unreachable!(),
            };

            // we only push half of the distance, but do it twice, so that
            // also the intermediate position gets covered, not just the
            // end position.
            self.push(hero_data, level_data, distance, actor_adder);
            self.push(hero_data, level_data, distance, actor_adder);

            let x = self.position.x;

            if x < x_start || x > x_end {
                self.countdown = 1;
            }
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
        actor_adder: &mut dyn ActorAdder,
    ) {
        if self.countdown == 2 {
            self.position.x += offset;
            if level_data.actors.process_shot(
                self.position,
                &mut level_data.solids,
                &mut level_data.tiles,
                actor_adder,
                hero_data,
            ) {
                self.countdown = 1;
            }
        }
        if self.countdown == 2 {
            if level_data.solids.collides(self.position) {
                self.countdown = 1;
                actor_adder.add_actor(
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
    pub type FnShotList = super::ShotList;

    use super::super::hero::ffi::FnHeroData;
    use super::super::level::actor::ffi::FnLevelActorQueue;
    use super::super::level::ffi::FnLevelData;
    use super::super::tilecache::ffi::FnTileCache;
    use super::HorizontalDirection;
    use crate::HALFTILE_WIDTH;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_shot_list_create() -> *mut FnShotList {
        Box::into_raw(Box::new(FnShotList::new()))
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_list_free(ptr: *mut FnShotList) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_list_count(l: *const FnShotList) -> usize {
        assert!(!l.is_null());
        let l: &FnShotList = unsafe { &(*l) };
        l.len()
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_list_add(
        l: *mut FnShotList,
        hero_data: *mut FnHeroData,
        level_data: *mut FnLevelData,
        actor_queue: *mut FnLevelActorQueue,
        x: i16,
        y: i16,
        direction: HorizontalDirection,
    ) {
        assert!(!l.is_null());
        let l: &mut FnShotList = unsafe { &mut (*l) };

        assert!(!hero_data.is_null());
        let hero_data: &mut FnHeroData = unsafe { &mut (*hero_data) };

        assert!(!level_data.is_null());
        let level_data: &mut FnLevelData = unsafe { &mut (*level_data) };

        assert!(!actor_queue.is_null());
        let actor_queue: &mut FnLevelActorQueue =
            unsafe { &mut (*actor_queue) };

        let mut shot = FnShot::new(x, y, direction);

        let distance = match shot.direction {
            HorizontalDirection::Left => -(HALFTILE_WIDTH as i16),
            HorizontalDirection::Right => HALFTILE_WIDTH as i16,
            _ => unreachable!(),
        };

        // we only push half of the distance, but do it twice, so that
        // also the intermediate position gets covered, not just the
        // end position.
        shot.push(hero_data, level_data, distance, actor_queue);

        l.push(shot);
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_list_act(
        l: *mut FnShotList,
        hero_data: *mut FnHeroData,
        level_data: *mut FnLevelData,
        actor_queue: *mut FnLevelActorQueue,
    ) {
        assert!(!l.is_null());
        let l: &mut FnShotList = unsafe { &mut (*l) };

        assert!(!hero_data.is_null());
        let hero_data: &mut FnHeroData = unsafe { &mut (*hero_data) };

        assert!(!level_data.is_null());
        let level_data: &mut FnLevelData = unsafe { &mut (*level_data) };

        assert!(!actor_queue.is_null());
        let actor_queue: &mut FnLevelActorQueue =
            unsafe { &mut (*actor_queue) };

        for shot in l.iter_mut() {
            shot.act(hero_data, level_data, actor_queue);
        }
        l.retain(|s| s.is_alive);
    }

    #[no_mangle]
    pub extern "C" fn fn_shot_list_blit(
        ptr: *const FnShotList,
        target: *mut SDL_Surface,
        tilecache: *const FnTileCache,
        draw_collision_bounds: bool,
    ) {
        assert!(!ptr.is_null());
        let d: &FnShotList = unsafe { &(*ptr) };

        assert!(!target.is_null());
        let mut target = Surface { raw: target };

        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        for shot in d.iter() {
            shot.blit(&mut target, tilecache, draw_collision_bounds);
        }
    }
}
