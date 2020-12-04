use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, SOLID_START, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    touch_count: usize,
    touching_hero: bool,
    floor_length: u32,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let mut floor_length = 0;
        while !solids.get(
            general.position.x() as u32 / TILE_WIDTH + floor_length,
            general.position.y() as u32 / TILE_HEIGHT,
        ) {
            solids.set(
                general.position.x() as u32 / TILE_WIDTH + floor_length,
                general.position.y() as u32 / TILE_HEIGHT,
                true,
            );
            floor_length += 1;
        }

        general
            .position
            .resize(TILE_WIDTH * floor_length, TILE_HEIGHT);

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
        let hero_center = hero_geometry.x() + (hero_geometry.w as i32) / 2;
        let stands_upon = hero_center >= p.general.position.left()
            && hero_center <= p.general.position.right()
            && hero_geometry.bottom() == p.general.position.top();

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
                    r.x() as u32 / TILE_WIDTH,
                    r.y() as u32 / TILE_HEIGHT,
                    false,
                );
                p.actor_adder
                    .add_actor(ActorType::Explosion, r.top_left());
                p.actor_adder.add_particle_firework(r.center(), 4);
                r.offset(TILE_WIDTH as i32, 0);
            }
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p.general.position.top_left();
        for _ in 0..self.floor_length {
            p.renderer.place_tile(self.tile, pos)?;
            pos.x += TILE_WIDTH as i32;
        }
        Ok(())
    }
}
