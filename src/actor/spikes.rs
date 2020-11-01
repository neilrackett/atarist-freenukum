use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{
    OBJECT_SPIKE, OBJECT_SPIKES_DOWN, OBJECT_SPIKES_UP, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

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
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        Specific {
            touching_hero: false,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        _general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
    }

    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = true;
        self.touching_hero = true;
    }

    fn hero_touch_end(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = false;
        self.touching_hero = false;
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = match general.actor_type {
            ActorType::SpikesUp => OBJECT_SPIKES_UP,
            ActorType::SpikesDown => OBJECT_SPIKES_DOWN,
            ActorType::Spike if self.touching_hero => OBJECT_SPIKE + 1,
            ActorType::Spike => OBJECT_SPIKE,
            _ => unreachable!(),
        };

        let tile = tilecache.get_tile(tile).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}
