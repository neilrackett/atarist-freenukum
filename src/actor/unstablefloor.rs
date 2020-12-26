use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, SOLID_START, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    touch_count: usize,
    touching_hero: bool,
    floor_length: u32,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let mut floor_length = 0;
        let mut position =
            Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT);
        while !solids.get(
            position.x() as u32 / TILE_WIDTH + floor_length,
            position.y() as u32 / TILE_HEIGHT,
        ) {
            solids.set(
                position.x() as u32 / TILE_WIDTH + floor_length,
                position.y() as u32 / TILE_HEIGHT,
                true,
            );
            floor_length += 1;
        }

        position.resize(TILE_WIDTH * floor_length, TILE_HEIGHT);

        Specific {
            tile: SOLID_START + 77,
            touch_count: 0,
            touching_hero: false,
            floor_length,
            position,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        // Detect whether the hero is standing upon the floor.
        // We can't use the hero_touch_start functionality here
        // because it only gets triggered when the hero geometry
        // overlaps with the part, which is not the case here.
        let hero_geometry = p.hero.position.geometry;
        let hero_center = hero_geometry.x() + (hero_geometry.w as i32) / 2;
        let stands_upon = hero_center >= self.position.left()
            && hero_center <= self.position.right()
            && hero_geometry.bottom() == self.position.top();

        if stands_upon {
            if !self.touching_hero {
                self.touching_hero = true;
                self.touch_count += 1;
            }
        } else {
            self.touching_hero = false;
        }

        if self.touch_count >= 2 {
            let mut r = self.position;
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
        let mut pos = self.position.top_left();
        for _ in 0..self.floor_length {
            p.renderer.place_tile(self.tile, pos)?;
            pos.x += TILE_WIDTH as i32;
        }
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }
}
