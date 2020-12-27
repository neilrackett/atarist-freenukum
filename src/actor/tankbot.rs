use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActor,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, HorizontalDirection, Result, ANIMATION_CARBOT, HALFTILE_HEIGHT,
    HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: usize,
    position: Rect,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            orientation: HorizontalDirection::Left,
            tile: ANIMATION_CARBOT,
            current_frame: 0,
            num_frames: 4,
            was_shot: 0,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH * 2, TILE_HEIGHT),
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if !self.is_alive() {
            p.actor_adder.add_actor(
                ActorType::SingleAnimation(SingleAnimationType::Explosion),
                self.position.top_left().offset(HALFTILE_WIDTH as i32, 0),
            );
            p.actor_adder
                .add_particle_firework(self.position.top_left(), 4);
            p.hero.score.add(2500);
        } else if p.solids.get(
            self.position.x() as u32 / TILE_WIDTH,
            self.position.y() as u32 / TILE_HEIGHT + 1,
        ) && !p.solids.get(
            self.position.x() as u32 / TILE_WIDTH + 1,
            self.position.y() as u32 / TILE_HEIGHT + 1,
        ) {
            // still in the air, falling down
            self.position.offset(0, HALFTILE_HEIGHT as i32);
        } else {
            // on the floor, walking
            let mut direction = match self.orientation {
                HorizontalDirection::Left => -1,
                HorizontalDirection::Right => 4,
            };

            if !p.solids.get(
                // check if the place next ot the bot is free
                (self.position.x() + direction * HALFTILE_WIDTH as i32)
                    as u32
                    / TILE_WIDTH,
                self.position.y() as u32 / TILE_HEIGHT,
            ) && p.solids.get(
                // check if the tile below is solid
                (self.position.x() as i32
                    + direction * HALFTILE_WIDTH as i32)
                    as u32
                    / TILE_WIDTH,
                (self.position.y() as u32 + TILE_HEIGHT) / TILE_HEIGHT,
            ) {
                if direction > 0 {
                    direction = 1;
                }
                self.position.offset(
                    (direction as f64 * HALFTILE_WIDTH as f64 * 0.7)
                        as i32,
                    0,
                );
            } else {
                // reached the end, turning around
                self.orientation.reverse();
                if direction > 0 {
                    direction = 1;
                }
                direction *= -1;
                self.position.offset(direction * HALFTILE_WIDTH as i32, 0);
                self.tile = (self.tile as i32 + 4 * direction) as usize;

                p.actor_adder.add_actor(
                    ActorType::HostileShot(self.orientation),
                    self.position.top_left().offset(0, -6),
                );
            }
        }
        if self.was_shot == 1 {
            // create steam clouds
            if self.current_frame == 0 {
                p.actor_adder.add_actor(
                    ActorType::SingleAnimation(SingleAnimationType::Steam),
                    self.position.top_left().offset(
                        HALFTILE_WIDTH as i32,
                        -(TILE_HEIGHT as i32),
                    ),
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();
        let tile = self.tile + (self.current_frame / 2) * 2;
        p.renderer.place_tile(tile, pos)?;

        let tile = self.tile + (self.current_frame / 2) * 2 + 1;
        pos = pos.offset(TILE_WIDTH as i32, 0);
        p.renderer.place_tile(tile, pos)?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, _p: ShotParameters) -> ShotProcessing {
        if self.was_shot != 2 {
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
        self.position.has_intersection(hero.position.geometry)
    }

    fn is_alive(&self) -> bool {
        self.was_shot < 2
    }
}
