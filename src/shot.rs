// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActorMessageQueue, ActorType, ActorsList, SingleAnimationType,
    },
    game::GameCommands,
    hero::Hero,
    level::tiles::LevelTiles,
    rendering::Renderer,
    HorizontalDirection, Result, Sizes, LEVELWINDOW_WIDTH, OBJECT_SHOT,
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
    pub fn new(
        sizes: &dyn Sizes,
        x: i32,
        y: i32,
        direction: HorizontalDirection,
    ) -> Self {
        let w = sizes.width() / 4;
        let h = sizes.height() / 4 * 3;
        Shot {
            position: Rect::new(
                x + sizes.width() as i32 - w as i32 / 2
                    + direction.as_factor_i32() * w as i32,
                y + sizes.height() as i32 - h as i32,
                w,
                h,
            ),
            is_alive: true,
            direction,
            counter: 0,
            countdown: 2,
        }
    }

    /// Returns whether the shot is still alive after acting.
    pub fn act(
        &mut self,
        sizes: &dyn Sizes,
        hero: &mut Hero,
        actors: &mut ActorsList,
        tiles: &mut LevelTiles,
        game_commands: &mut dyn GameCommands,
        actor_message_queue: &mut ActorMessageQueue,
    ) -> bool {
        self.counter += 1;
        self.counter %= 4;

        if self.countdown == 1 {
            self.is_alive = false;
            self.countdown -= 1;
        }

        let x_start = hero.position.geometry.x()
            - sizes.width() as i32 * LEVELWINDOW_WIDTH as i32 / 2;
        let x_end = hero.position.geometry.x()
            + hero.position.geometry.w as i32
            + sizes.width() as i32 * LEVELWINDOW_WIDTH as i32 / 2;

        if self.countdown >= 2 {
            let distance =
                sizes.half_width() as i32 * self.direction.as_factor_i32();

            // we only push half of the distance, but do it twice, so that
            // also the intermediate position gets covered, not just the
            // end position.
            self.push(
                sizes,
                hero,
                actors,
                tiles,
                distance,
                game_commands,
                actor_message_queue,
            );
            self.push(
                sizes,
                hero,
                actors,
                tiles,
                distance,
                game_commands,
                actor_message_queue,
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
        sizes: &dyn Sizes,
        draw_collision_bounds: bool,
    ) -> Result<()> {
        if self.is_alive {
            let mut destrect = self.position;
            destrect.set_x(
                destrect.x() + destrect.width() as i32 / 2
                    - sizes.half_width() as i32,
            );
            destrect.set_width(sizes.width());

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

    #[allow(clippy::too_many_arguments)]
    pub fn push(
        &mut self,
        sizes: &dyn Sizes,
        hero: &mut Hero,
        actors: &mut ActorsList,
        tiles: &mut LevelTiles,
        offset: i32,
        game_commands: &mut dyn GameCommands,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        if self.countdown >= 2 {
            self.position.x += offset;
            if self.countdown == 2 {
                if actors.process_shot(
                    self.position,
                    sizes,
                    tiles,
                    game_commands,
                    hero,
                    actor_message_queue,
                ) {
                    self.countdown = 1;
                }
            } else {
                self.countdown -= 1;
            }
        }
        if self.countdown >= 2 && tiles.collides(sizes, self.position) {
            self.countdown = 1;
            game_commands.add_actor(
                ActorType::SingleAnimation(SingleAnimationType::Explosion),
                self.position.top_left().offset(
                    self.position.width() as i32 / 2
                        - sizes.half_width() as i32,
                    0,
                ),
            );
        }
    }

    pub fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }
}
