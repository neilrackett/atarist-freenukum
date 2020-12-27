use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActor, RenderParameters,
        ShotParameters, ShotProcessing, SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, HorizontalDirection, Result, Sizes, ANIMATION_FIREWHEEL_OFF,
    ANIMATION_FIREWHEEL_ON,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: usize,
    fire_is_on: bool,
    counter: usize,
    position: Rect,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_FIREWHEEL_OFF,
            counter: 0,
            current_frame: 0,
            num_frames: 4,
            was_shot: 0,
            fire_is_on: false,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, p: ActParameters) {
        if self.was_shot == 2 {
            p.actor_adder.add_actor(
                ActorType::SingleAnimation(SingleAnimationType::Explosion),
                self.position
                    .top_left()
                    .offset(p.sizes.half_width() as i32, 0),
            );
            p.actor_adder
                .add_particle_firework(self.position.top_left(), 8);
            p.hero.score.add(2500);
        } else {
            self.counter += 1;
            if self.counter % 2 == 1 {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
            }

            if self.counter == 50 {
                self.counter = 0;
                self.fire_is_on = !self.fire_is_on;
                if self.fire_is_on {
                    self.tile = ANIMATION_FIREWHEEL_ON;
                } else {
                    self.tile = ANIMATION_FIREWHEEL_OFF;
                }
            }

            let direction = self.direction.as_factor_i32();

            if !p.solids.push_rect_standing_on_ground(
                p.sizes,
                &mut self.position,
                direction * p.sizes.half_width() as i32 / 2,
                p.sizes.half_height() as u8,
            ) {
                // push was not successful, so we reverse the direction
                self.direction.reverse();
            }

            if self.was_shot == 1 {
                // create steam clouds
                if self.current_frame == 0 {
                    p.actor_adder.add_actor(
                        ActorType::SingleAnimation(
                            SingleAnimationType::Steam,
                        ),
                        self.position.top_left().offset(
                            p.sizes.half_width() as i32,
                            -(p.sizes.height() as i32),
                        ),
                    );
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let r = self.position;
        let mut pos = r.top_left();
        pos.x = pos.x + r.width() as i32 / 2 - p.sizes.width() as i32;
        pos.y -= p.sizes.height() as i32;

        p.renderer
            .place_tile(self.tile + self.current_frame * 4, pos)?;
        pos.x += p.sizes.width() as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 4 + 1, pos)?;
        pos.x -= p.sizes.width() as i32;
        pos.y += p.sizes.height() as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 4 + 2, pos)?;
        pos.x += p.sizes.width() as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 4 + 3, pos)?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, _p: ShotParameters) -> ShotProcessing {
        if !self.fire_is_on && self.was_shot < 2 {
            self.was_shot += 1;
        }
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.was_shot < 2
            && self.position.has_intersection(hero.position.geometry)
    }

    fn is_alive(&self) -> bool {
        self.was_shot < 2
    }
}
