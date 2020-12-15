use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ActorType, HeroTouchStartParameters,
        RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_ROTATINGCYLINDER, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    lives: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = false;

        while general.position.y > 0
            && !solids.get(
                general.position.x() as u32 / TILE_WIDTH,
                general.position.y() as u32 / TILE_HEIGHT - 1,
            )
        {
            general.position.offset(0, -(TILE_HEIGHT as i32));
            general
                .position
                .set_height(general.position.height() + TILE_HEIGHT);
        }

        Specific {
            tile: OBJECT_ROTATINGCYLINDER,
            current_frame: 0,
            num_frames: 5,
            lives: 10,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.hero_data.health.kill();
    }

    fn act(&mut self, _p: ActParameters) {
        if self.lives > 0 {
            self.current_frame += 1;
            self.current_frame %= self.num_frames;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p.general.position.top_left();

        for _ in 0..p.general.position.height() / TILE_WIDTH {
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
                .add_particle_firework(p.general.position.center(), 4);
        } else {
            // TODO: add removal animation (destroyed body)
            p.general.is_alive = false;
            p.actor_message_queue.push_back(
                ActorType::ElectricArc,
                ActorMessageType::Remove,
            );
            p.hero_data.score.add(20000);
            p.actor_adder
                .add_particle_firework(p.general.position.center(), 20);
            p.actor_adder.add_actor(
                ActorType::Score10000,
                p.general.position.top_left().offset(
                    0,
                    (p.general.position.height() / 2 - TILE_HEIGHT) as i32,
                ),
            );
            p.actor_adder.add_actor(
                ActorType::Score10000,
                p.general
                    .position
                    .top_left()
                    .offset(0, p.general.position.height() as i32 / 2),
            );
        }
        ShotProcessing::Absorb
    }
}
