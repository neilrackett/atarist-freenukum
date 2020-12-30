use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActor, RenderParameters,
        SingleAnimationType,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Result, Sizes, SOLID_START,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct UnstableFloor {
    tile: usize,
    touch_count: usize,
    touching_hero: bool,
    floor_length: u32,
    position: Rect,
}

impl CreateActor for UnstableFloor {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Self {
        let mut floor_length = 0u32;
        let mut position =
            Rect::new(pos.x, pos.y, sizes.width(), sizes.height());
        while tiles
            .get(
                position.x() / sizes.width() as i32 + floor_length as i32,
                position.y() / sizes.height() as i32,
            )
            .map(|t| !t.solid)
            .unwrap_or(false)
        {
            if let Ok(ref mut t) = tiles.get_mut(
                position.x() / sizes.width() as i32 + floor_length as i32,
                position.y() / sizes.height() as i32,
            ) {
                t.solid = true;
            }
            floor_length += 1;
        }

        position.resize(sizes.width() * floor_length, sizes.height());

        Self {
            tile: SOLID_START + 77,
            touch_count: 0,
            touching_hero: false,
            floor_length,
            position,
        }
    }
}

impl Actor for UnstableFloor {
    fn act(&mut self, p: ActParameters) {
        // Detect whether the hero is standing upon the floor.
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

        if !self.is_alive() {
            let mut r = self.position;
            for _ in 0..self.floor_length {
                if let Ok(ref mut t) = p.tiles.get_mut(
                    r.x() / p.sizes.width() as i32,
                    r.y() / p.sizes.height() as i32,
                ) {
                    t.solid = false;
                }
                p.actor_adder.add_actor(
                    ActorType::SingleAnimation(
                        SingleAnimationType::Explosion,
                    ),
                    r.top_left(),
                );
                p.actor_adder.add_particle_firework(r.center(), 4);
                r.offset(p.sizes.width() as i32, 0);
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();
        for _ in 0..self.floor_length {
            p.renderer.place_tile(self.tile, pos)?;
            pos.x += p.sizes.width() as i32;
        }
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        self.touch_count < 2
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromAbove
    }
}
