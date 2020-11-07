use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchStartParameters, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{ANIMATION_MINE, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    touching_hero: u8,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {
            tile: ANIMATION_MINE,
            touching_hero: 0,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.general.hurts_hero = true;
        self.touching_hero = 1;
    }

    fn act(&mut self, p: ActParameters) {
        if !p.solids.get(
            p.general.position.x as usize / TILE_WIDTH,
            p.general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            p.general.position.y += HALFTILE_HEIGHT as i16;
        }

        match self.touching_hero {
            1 => self.touching_hero += 1,
            2 => {
                p.general.hurts_hero = false;
                p.general.is_alive = false;
                p.actor_adder.add_actor(
                    ActorType::BombFire,
                    p.general.position.x as u16,
                    p.general.position.y as u16,
                );
            }
            _ => {}
        }
    }

    fn render(&mut self, p: RenderParameters) {
        p.renderer.place_tile(self.tile, p.general.position);
    }
}
