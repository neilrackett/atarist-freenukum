use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{OBJECT_HOSTILESHOT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    touching_hero: bool,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let tile = match general.actor_type {
            ActorType::HostileShotLeft => OBJECT_HOSTILESHOT,
            ActorType::HostileShotRight => OBJECT_HOSTILESHOT + 2,
            _ => unreachable!(
                "Passed actor type {:?} to hostile shot actor \
                which is not a shot actor id",
                general.actor_type
            ),
        };

        Specific {
            tile,
            touching_hero: false,
            current_frame: 0,
            num_frames: 2,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        self.touching_hero = true;
        general.hurts_hero = true;
    }

    fn hero_touch_end(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
    ) {
        self.touching_hero = false;
        general.hurts_hero = false;
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        let offset = match general.actor_type {
            ActorType::HostileShotLeft => -(TILE_WIDTH as i16),
            ActorType::HostileShotRight => TILE_WIDTH as i16,
            _ => unreachable!(),
        };
        general.position.x += offset;

        if solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT,
        ) {
            general.is_alive = false;
            if self.touching_hero {
                general.hurts_hero = false;
            }
        }

        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile =
            tilecache.get_tile(self.tile + self.current_frame).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}
