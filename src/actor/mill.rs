use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ActorType, RenderParameters, ShotParameters,
        ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_ROTATINGCYLINDER, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    lives: usize,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let mut position =
            Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT);
        while position.y > 0
            && !solids.get(
                position.x() as u32 / TILE_WIDTH,
                position.y() as u32 / TILE_HEIGHT - 1,
            )
        {
            position.offset(0, -(TILE_HEIGHT as i32));
            position.set_height(position.height() + TILE_HEIGHT);
        }

        Specific {
            tile: OBJECT_ROTATINGCYLINDER,
            current_frame: 0,
            num_frames: 5,
            lives: 10,
            position,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        if self.lives > 0 {
            self.current_frame += 1;
            self.current_frame %= self.num_frames;

            if self.position.has_intersection(p.hero.position.geometry) {
                p.hero.health.kill();
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();

        for _ in 0..self.position.height() / TILE_WIDTH {
            p.renderer.place_tile(self.tile + self.current_frame, pos)?;
            pos.y += TILE_HEIGHT as i32;
        }
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.lives -= 1;
        if self.lives > 0 {
            p.actor_adder
                .add_particle_firework(self.position.center(), 4);
        } else {
            // TODO: add removal animation (destroyed body)
            p.actor_message_queue.push_back(
                ActorType::ElectricArc,
                ActorMessageType::Remove,
            );
            p.hero.score.add(20000);
            p.actor_adder
                .add_particle_firework(self.position.center(), 20);
            p.actor_adder.add_actor(
                ActorType::Score10000,
                self.position.top_left().offset(
                    0,
                    (self.position.height() / 2 - TILE_HEIGHT) as i32,
                ),
            );
            p.actor_adder.add_actor(
                ActorType::Score10000,
                self.position
                    .top_left()
                    .offset(0, self.position.height() as i32 / 2),
            );
        }
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        self.lives > 0
    }
}
