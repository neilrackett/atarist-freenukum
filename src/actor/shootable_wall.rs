use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, RenderParameters, ShotParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    BACKGROUND_LIGHT_GREY, SOLID_SHOOTABLE_WALL_BRICKS, TILE_HEIGHT,
    TILE_WIDTH,
};

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
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
        p.general.is_alive = false;
        p.solids.set(
            p.general.position.x as usize / TILE_WIDTH,
            p.general.position.y as usize / TILE_HEIGHT,
            false,
        );
    }

    fn render(&mut self, p: RenderParameters) {
        p.tilecache
            .get_tile(BACKGROUND_LIGHT_GREY)
            .unwrap()
            .blit_to_sdl_surface(None, p.target, Some(p.general.position));
        p.tilecache
            .get_tile(SOLID_SHOOTABLE_WALL_BRICKS)
            .unwrap()
            .blit_to_sdl_surface(None, p.target, Some(p.general.position));
    }
}
