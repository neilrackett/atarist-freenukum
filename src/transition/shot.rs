use super::geometry::Geometry;
use super::hero::HeroData;
use super::level::actor::{ActorAdder, ActorType, ActorsList};
use super::level::solids::LevelSolids;
use super::level::tiles::LevelTiles;
use super::level::LevelData;
use super::tilecache::TileCache;
use super::HorizontalDirection;
use crate::{
    HALFTILE_WIDTH, LEVELWINDOW_WIDTH, OBJECT_SHOT, TILE_HEIGHT,
    TILE_WIDTH,
};

pub type ShotList = Vec<Shot>;

#[derive(Debug)]
pub struct Shot {
    position: Geometry,
    pub is_alive: bool,
    pub direction: HorizontalDirection,
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
        actors: &mut ActorsList,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
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
            self.push(
                hero_data,
                actors,
                solids,
                tiles,
                distance,
                actor_adder,
            );
            self.push(
                hero_data,
                actors,
                solids,
                tiles,
                distance,
                actor_adder,
            );

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

    pub fn push(
        &mut self,
        hero_data: &mut HeroData,
        actors: &mut ActorsList,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        offset: i16,
        actor_adder: &mut dyn ActorAdder,
    ) {
        if self.countdown == 2 {
            self.position.x += offset;
            if actors.process_shot(
                self.position,
                solids,
                tiles,
                actor_adder,
                hero_data,
            ) {
                self.countdown = 1;
            }
        }
        if self.countdown == 2 {
            if solids.collides(self.position) {
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
