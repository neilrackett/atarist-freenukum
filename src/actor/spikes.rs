use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_SPIKE, OBJECT_SPIKES_DOWN, OBJECT_SPIKES_UP,
    TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    touching_hero: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = true;

        Specific {
            touching_hero: false,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {}

    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.general.hurts_hero = true;
        self.touching_hero = true;
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        p.general.hurts_hero = false;
        self.touching_hero = false;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = match p.general.actor_type {
            ActorType::SpikesUp => OBJECT_SPIKES_UP,
            ActorType::SpikesDown => OBJECT_SPIKES_DOWN,
            ActorType::Spike if self.touching_hero => OBJECT_SPIKE + 1,
            ActorType::Spike => OBJECT_SPIKE,
            _ => unreachable!(),
        };

        p.renderer.place_tile(tile, p.general.position.top_left())?;
        Ok(())
    }
}
