use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchStartParameters, RenderParameters,
        ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_BALLOON, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    destroyed: bool,
    current_frame: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT * 2);

        Specific {
            destroyed: false,
            current_frame: 0,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        if !self.destroyed {
            p.general.is_alive = false;
            p.hero_data.score.add(10000);
            p.actor_adder.add_actor(
                ActorType::Score10000,
                p.general.position.top_left(),
            );
        }
    }

    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= 9;

        if self.destroyed {
            p.general.is_alive = false;
        } else {
            p.general.position.y -= 1;
            if p.solids.get(
                p.general.position.x() as u32 / TILE_WIDTH,
                p.general.position.y() as u32 / TILE_WIDTH,
            ) {
                // balloon bumps against wall
                self.destroyed = true;
                p.actor_adder.add_actor(
                    ActorType::Steam,
                    p.general.position.top_left(),
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p.general.position.top_left();

        let tile = if self.destroyed {
            OBJECT_BALLOON + 4
        } else {
            OBJECT_BALLOON
        };
        p.renderer.place_tile(tile, pos)?;

        pos.y += TILE_HEIGHT as i32;
        p.renderer.place_tile(
            OBJECT_BALLOON + 1 + self.current_frame / 3,
            pos,
        )?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.destroyed = true;
        p.actor_adder
            .add_actor(ActorType::Steam, p.general.position.top_left());
        ShotProcessing::Absorb
    }
}
