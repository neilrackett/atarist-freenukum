use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchStartParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_SODAFLY, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH,
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
        Specific {}
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.hero_data.score.add(1000);
        p.actor_adder.add_actor(
            ActorType::Score1000,
            p.general.position.top_left(),
        );
        p.general.is_alive = false;
    }

    fn act(&mut self, p: ActParameters) {
        p.general.position.offset(0, -(HALFTILE_HEIGHT as i32));
        if p.solids.get(
            p.general.position.x() as u32 / TILE_WIDTH,
            p.general.position.y() as u32 / TILE_HEIGHT,
        ) {
            p.actor_adder.add_actor(
                ActorType::Explosion,
                p.general.position.top_left(),
            );
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = ANIMATION_SODAFLY
            + ((p.general.position.y() as usize
                / HALFTILE_HEIGHT as usize)
                % 4);
        p.renderer.place_tile(tile, p.general.position.top_left())?;
        Ok(())
    }
}
