use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters, ShotParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, BACKGROUND_LIGHT_GREY, SOLID_SHOOTABLE_WALL_BRICKS,
    TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = false;

        Specific {}
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {}

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        p.hero_data.score.add(10);
        p.actor_adder.add_actor(
            ActorType::Explosion,
            p.general.position.top_left(),
        );
        p.general.is_alive = false;
        p.solids.set(
            p.general.position.x() as u32 / TILE_WIDTH,
            p.general.position.y() as u32 / TILE_HEIGHT,
            false,
        );
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            BACKGROUND_LIGHT_GREY,
            p.general.position.top_left(),
        )?;
        p.renderer.place_tile(
            SOLID_SHOOTABLE_WALL_BRICKS,
            p.general.position.top_left(),
        )?;
        Ok(())
    }
}
