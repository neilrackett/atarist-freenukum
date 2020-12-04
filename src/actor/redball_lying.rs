use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchStartParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_MINE, HALFTILE_HEIGHT, TILE_HEIGHT, TILE_WIDTH,
};

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
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);

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
            p.general.position.x() as u32 / TILE_WIDTH,
            p.general.position.y() as u32 / TILE_HEIGHT + 1,
        ) {
            p.general.position.offset(0, HALFTILE_HEIGHT as i32);
        }

        match self.touching_hero {
            1 => self.touching_hero += 1,
            2 => {
                p.general.hurts_hero = false;
                p.general.is_alive = false;
                p.actor_adder.add_actor(
                    ActorType::BombFire,
                    p.general.position.top_left(),
                );
            }
            _ => {}
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer
            .place_tile(self.tile, p.general.position.top_left())?;
        Ok(())
    }
}
