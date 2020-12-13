use crate::{
    actor::{ActorAdder, ActorType, ActorsList},
    hero::HeroData,
    level::{solids::LevelSolids, tiles::LevelTiles},
    rendering::Renderer,
    HorizontalDirection, Result, HALFTILE_WIDTH, LEVELWINDOW_WIDTH,
    OBJECT_SHOT, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::Rect;

pub type ShotList = Vec<Shot>;

#[derive(Debug)]
pub struct Shot {
    position: Rect,
    pub is_alive: bool,
    pub direction: HorizontalDirection,
    counter: usize,
    countdown: usize,
}

impl Shot {
    pub fn new(x: i32, y: i32, direction: HorizontalDirection) -> Self {
        let w = 4;
        let h = TILE_HEIGHT - 4;
        Shot {
            position: Rect::new(
                x + HALFTILE_WIDTH as i32 - w as i32 / 2,
                y + TILE_HEIGHT as i32 - h as i32,
                w,
                h,
            ),
            is_alive: true,
            direction,
            counter: 0,
            countdown: 3,
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

        let x_start = hero_data.position.geometry.x()
            - TILE_WIDTH as i32 * LEVELWINDOW_WIDTH as i32 / 2;
        let x_end = hero_data.position.geometry.x()
            + hero_data.position.geometry.w as i32
            + TILE_WIDTH as i32 * LEVELWINDOW_WIDTH as i32 / 2;

        if self.countdown >= 2 {
            let distance = match self.direction {
                HorizontalDirection::Left => -(HALFTILE_WIDTH as i32),
                HorizontalDirection::Right => HALFTILE_WIDTH as i32,
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

    pub fn render(
        &self,
        renderer: &mut dyn Renderer,
        draw_collision_bounds: bool,
    ) -> Result<()> {
        if self.is_alive {
            let mut destrect = self.position;
            destrect.set_x(
                destrect.x() + destrect.width() as i32 / 2
                    - HALFTILE_WIDTH as i32,
            );
            destrect.set_width(TILE_WIDTH);

            renderer.place_tile(
                OBJECT_SHOT + self.counter,
                destrect.top_left(),
            )?;
            if draw_collision_bounds {
                let color = crate::collision_bounds_color();
                renderer.draw_rect(self.position, color)?;
            }
        }
        Ok(())
    }

    pub fn push(
        &mut self,
        hero_data: &mut HeroData,
        actors: &mut ActorsList,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        offset: i32,
        actor_adder: &mut dyn ActorAdder,
    ) {
        if self.countdown >= 2 {
            self.position.x += offset;
            if self.countdown == 2 {
                if actors.process_shot(
                    self.position,
                    solids,
                    tiles,
                    actor_adder,
                    hero_data,
                ) {
                    self.countdown = 1;
                }
            } else {
                self.countdown -= 1;
            }
        }
        if self.countdown >= 2 && solids.collides(self.position) {
            self.countdown = 1;
            actor_adder.add_actor(
                ActorType::Explosion,
                self.position.top_left().offset(
                    self.position.width() as i32 / 2
                        - HALFTILE_WIDTH as i32,
                    0,
                ),
            );
        }
    }

    pub fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }
}
