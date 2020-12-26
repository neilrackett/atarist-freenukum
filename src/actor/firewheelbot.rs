use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
        RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    HorizontalDirection, Result, ANIMATION_FIREWHEEL_OFF,
    ANIMATION_FIREWHEEL_ON, HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
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
    touching_hero: bool,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_FIREWHEEL_OFF,
            counter: 0,
            touching_hero: false,
            current_frame: 0,
            num_frames: 4,
            was_shot: 0,
            fire_is_on: false,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        if self.was_shot < 2 {
            self.touching_hero = true;
            p.general.hurts_hero = true;
        }
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        self.touching_hero = false;
        p.general.hurts_hero = false;
    }

    fn act(&mut self, p: ActParameters) {
        if self.was_shot == 2 {
            p.general.is_alive = false;
            p.actor_adder.add_actor(
                ActorType::Explosion,
                self.position.top_left().offset(HALFTILE_WIDTH as i32, 0),
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
                &mut self.position,
                direction * HALFTILE_WIDTH as i32 / 2,
                HALFTILE_HEIGHT as u8,
            ) {
                // push was not successful, so we reverse the direction
                self.direction.reverse();
            }

            if self.was_shot == 1 {
                // create steam clouds
                if self.current_frame == 0 {
                    p.actor_adder.add_actor(
                        ActorType::Steam,
                        self.position.top_left().offset(
                            HALFTILE_WIDTH as i32,
                            -(TILE_HEIGHT as i32),
                        ),
                    );
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let r = self.position;
        let mut pos = r.top_left();
        pos.x = pos.x + r.width() as i32 / 2 - TILE_WIDTH as i32;
        pos.y -= TILE_HEIGHT as i32;

        p.renderer
            .place_tile(self.tile + self.current_frame * 4, pos)?;
        pos.x += TILE_WIDTH as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 4 + 1, pos)?;
        pos.x -= TILE_WIDTH as i32;
        pos.y += TILE_HEIGHT as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 4 + 2, pos)?;
        pos.x += TILE_WIDTH as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 4 + 3, pos)?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        if !self.fire_is_on {
            if self.was_shot == 1 && self.touching_hero {
                p.general.hurts_hero = false;
                self.touching_hero = false;
            }
            if self.was_shot != 2 {
                self.was_shot += 1;
            }
        }
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }
}
