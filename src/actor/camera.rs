use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, RenderParameters, ShotParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    ANIMATION_CAMERA_CENTER, ANIMATION_CAMERA_LEFT,
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
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let x = general.position.x as usize / TILE_WIDTH;
        let y = general.position.y as usize / TILE_HEIGHT;
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

    fn render(&mut self, p: RenderParameters) {
        p.tilecache
            .get_tile(self.tile)
            .unwrap()
            .blit_to_sdl_surface(None, p.target, Some(p.general.position));
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        p.general.is_alive = false;
        p.hero_data.score.add(100);
        p.actor_adder.add_actor(
            ActorType::Score100,
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
        p.actor_adder.add_actor(
            ActorType::Explosion,
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
    }
}
