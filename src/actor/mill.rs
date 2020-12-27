use crate::{
    actor::{
        ActParameters, Actor, ActorMessageType, ActorType, CreateActor,
        RenderParameters, ScoreType, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, Sizes, OBJECT_ROTATINGCYLINDER,
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

impl CreateActor for Specific {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let mut position =
            Rect::new(pos.x, pos.y, sizes.width(), sizes.height());
        while position.y > 0
            && !solids.get(
                position.x() as u32 / sizes.width(),
                position.y() as u32 / sizes.height() - 1,
            )
        {
            position.offset(0, -(sizes.height() as i32));
            position.set_height(position.height() + sizes.height());
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

impl Actor for Specific {
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

        for _ in 0..self.position.height() / p.sizes.width() {
            p.renderer.place_tile(self.tile + self.current_frame, pos)?;
            pos.y += p.sizes.height() as i32;
        }
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.lives -= 1;
        if self.lives > 0 {
            p.actor_adder
                .add_particle_firework(self.position.center(), 4);
        } else {
            // TODO: add removal animation (destroyed body)
            p.actor_message_queue
                .push_back(ActorMessageType::RemoveElectricArc);
            p.hero.score.add(20000);
            p.actor_adder
                .add_particle_firework(self.position.center(), 20);
            p.actor_adder.add_actor(
                ActorType::Score(ScoreType::Score10000),
                self.position.top_left().offset(
                    0,
                    (self.position.height() / 2 - p.sizes.height()) as i32,
                ),
            );
            p.actor_adder.add_actor(
                ActorType::Score(ScoreType::Score10000),
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
