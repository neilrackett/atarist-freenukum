use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{SOLID_START, TILE_HEIGHT, TILE_WIDTH};

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
    fn act(&mut self, p: ActParameters) {
        // Detect whether the hero is standing upon the floor.
        // We can't use the hero_touch_start functionality here
        // because it only gets triggered when the hero geometry
        // overlaps with the part, which is not the case here.
        let hero_geometry = p.hero_data.position.geometry;
        let hero_center = hero_geometry.x + (hero_geometry.w as i16) / 2;
        let stands_upon = hero_center >= p.general.position.x
            && hero_center
                <= p.general.position.x + p.general.position.w as i16
            && hero_geometry.y + hero_geometry.h as i16
                == p.general.position.y;

        if stands_upon {
            if !self.touching_hero {
                self.touching_hero = true;
                self.touch_count += 1;
            }
        } else {
            self.touching_hero = false;
        }

        if self.touch_count >= 2 {
            let mut r = p.general.position;
            for _ in 0..self.floor_length {
                p.solids.set(
                    r.x as usize / TILE_WIDTH,
                    r.y as usize / TILE_HEIGHT,
                    false,
                );
                p.actor_adder.add_actor(
                    ActorType::Explosion,
                    r.x as u16,
                    r.y as u16,
                );
                p.actor_adder
                    .add_particle_firework(r.x as u16, r.y as u16, 4);
                r.x += TILE_WIDTH as i16;
            }
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let mut destrect = p.general.position;
        for _ in 0..self.floor_length {
            p.renderer.place_tile(self.tile, destrect);
            destrect.x += TILE_WIDTH as i16;
        }
    }
}
