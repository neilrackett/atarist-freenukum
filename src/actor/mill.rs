use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType, ActorType,
        CreateActor, RenderParameters, ScoreType, ShotParameters,
        ShotProcessing,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Result, Sizes, OBJECT_ROTATINGCYLINDER,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Mill {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    lives: usize,
    position: Rect,
}

impl CreateActor for Mill {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Actor {
        let mut position =
            Rect::new(pos.x, pos.y, sizes.width(), sizes.height());
        while position.y > 0
            && tiles
                .get(
                    position.x() / sizes.width() as i32,
                    position.y() / sizes.height() as i32 - 1,
                )
                .map(|t| !t.solid)
                .unwrap_or(true)
        {
            position.offset(0, -(sizes.height() as i32));
            position.set_height(position.height() + sizes.height());
        }

        Actor::Mill(Self {
            tile: OBJECT_ROTATINGCYLINDER,
            current_frame: 0,
            num_frames: 5,
            lives: 10,
            position,
        })
    }
}

impl ActorExt for Mill {
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

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }
}
