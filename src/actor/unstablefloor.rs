use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{SOLID_START, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    touch_count: usize,
    touching_hero: bool,
    floor_length: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let mut floor_length = 0;
        while !solids.get(
            general.position.x as usize / TILE_WIDTH + floor_length,
            general.position.y as usize / TILE_HEIGHT,
        ) {
            solids.set(
                general.position.x as usize / TILE_WIDTH + floor_length,
                general.position.y as usize / TILE_HEIGHT,
                true,
            );
            floor_length += 1;
        }

        general.position.w = (TILE_WIDTH * floor_length) as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {
            tile: SOLID_START + 77,
            touch_count: 0,
            touching_hero: false,
            floor_length,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        // Detect whether the hero is standing upon the floor.
        // We can't use the hero_touch_start functionality here
        // because it only gets triggered when the hero geometry
        // overlaps with the part, which is not the case here.
        let hero_geometry = hero_data.position.geometry;
        let hero_center = hero_geometry.x + (hero_geometry.w as i16) / 2;
        let stands_upon = hero_center >= general.position.x
            && hero_center
                <= general.position.x + general.position.w as i16
            && hero_geometry.y + hero_geometry.h as i16
                == general.position.y;

        if stands_upon {
            if !self.touching_hero {
                self.touching_hero = true;
                self.touch_count += 1;
            }
        } else {
            self.touching_hero = false;
        }

        if self.touch_count >= 2 {
            let mut r = general.position;
            for _ in 0..self.floor_length {
                solids.set(
                    r.x as usize / TILE_WIDTH,
                    r.y as usize / TILE_HEIGHT,
                    false,
                );
                actor_adder.add_actor(
                    ActorType::Explosion,
                    r.x as u16,
                    r.y as u16,
                );
                actor_adder
                    .add_particle_firework(r.x as u16, r.y as u16, 4);
                r.x += TILE_WIDTH as i16;
            }
            general.is_alive = false;
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = tilecache.get_tile(self.tile as usize).unwrap();
        let mut destrect = general.position;
        for _ in 0..self.floor_length {
            tile.blit_to_sdl_surface(None, target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }
}
