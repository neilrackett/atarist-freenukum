use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchStartParameters, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{ANIMATION_SODAFLY, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {}
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.hero_data.score.add(1000);
        p.actor_adder.add_actor(
            ActorType::Score1000,
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
        p.general.is_alive = false;
    }

    fn act(&mut self, p: ActParameters) {
        p.general.position.y -= HALFTILE_HEIGHT as i16;
        if p.solids.get(
            p.general.position.x as usize / TILE_WIDTH,
            p.general.position.y as usize / TILE_HEIGHT,
        ) {
            p.actor_adder.add_actor(
                ActorType::Explosion,
                p.general.position.x as u16,
                p.general.position.y as u16,
            );
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let tile = p
            .tilecache
            .get_tile(
                ANIMATION_SODAFLY
                    + ((p.general.position.y as usize / HALFTILE_HEIGHT)
                        % 4),
            )
            .unwrap();
        let destrect = p.general.position;
        tile.blit_to_sdl_surface(None, p.target, Some(destrect));
    }
}
