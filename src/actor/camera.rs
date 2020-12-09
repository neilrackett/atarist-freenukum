use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_CAMERA_CENTER, ANIMATION_CAMERA_LEFT,
    ANIMATION_CAMERA_RIGHT, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self {
        general.is_in_foreground = false;
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);

        let x = general.position.x() as u32 / TILE_WIDTH;
        let y = general.position.y() as u32 / TILE_HEIGHT;
        tiles.copy_from_to(x, y + 1, x, y);

        Specific {
            tile: ANIMATION_CAMERA_CENTER,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let x = p.hero_data.position.geometry.x;
        self.tile = if x - 1 > p.general.position.x {
            ANIMATION_CAMERA_RIGHT
        } else if x + 1 < p.general.position.x {
            ANIMATION_CAMERA_LEFT
        } else {
            ANIMATION_CAMERA_CENTER
        };
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer
            .place_tile(self.tile, p.general.position.top_left())?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.general.is_alive = false;
        p.hero_data.score.add(100);
        p.actor_adder
            .add_actor(ActorType::Score100, p.general.position.top_left());
        p.actor_adder.add_actor(
            ActorType::Explosion,
            p.general.position.top_left(),
        );
        ShotProcessing::Absorb
    }
}
