use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchStartParameters, RenderParameters,
        ShotParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_FALLINGBLOCK, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    counter: usize,
    touching_hero: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.resize(TILE_WIDTH * 2, TILE_HEIGHT);
        general.is_in_foreground = true;

        Specific {
            tile: OBJECT_FALLINGBLOCK,
            counter: 0,
            touching_hero: false,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let hero_geometry = p.hero_data.position.geometry;

        match self.counter {
            0 => {
                let xl = p.general.position.left();
                let xr = p.general.position.right();
                let y = p.general.position.y();
                let hxl = hero_geometry.left();
                let hxr = hero_geometry.right();
                let hy = hero_geometry.y();

                if y < hy && xl < hxr && xr > hxl {
                    let mut solid_between = false;
                    for i in (y as u32 / TILE_HEIGHT) + 1
                        ..hy as u32 / TILE_HEIGHT
                    {
                        let x = xl as u32 / TILE_WIDTH;
                        if p.solids.get(x, i) || p.solids.get(x + 1, i) {
                            solid_between = true;
                            break;
                        }
                    }
                    if !solid_between {
                        self.counter += 1;
                    }
                }
            }
            c if c <= 10 && c % 2 == 0 => {
                p.general.position.y -= 1;
                self.counter += 1;
            }
            c if c <= 10 && c % 2 == 1 => {
                p.general.position.y += 1;
                self.counter += 1;
            }
            _ => {
                if p.solids.get(
                    p.general.position.x() as u32 / TILE_WIDTH,
                    p.general.position.y() as u32 / TILE_HEIGHT + 1,
                ) {
                    p.actor_adder.add_actor(
                        ActorType::Steam,
                        p.general.position.top_left(),
                    );
                    p.actor_adder.add_particle_firework(
                        p.general.position.top_left(),
                        4,
                    );
                    p.general.is_alive = false;
                } else {
                    p.general.position.offset(0, TILE_HEIGHT as i32);
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p.general.position.top_left();
        p.renderer.place_tile(self.tile, pos)?;
        pos.x += TILE_WIDTH as i32;
        p.renderer.place_tile(self.tile + 1, pos)?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        if self.counter > 0 {
            p.hero_data.score.add(500);
            p.actor_adder.add_actor(
                ActorType::Score500,
                p.general.position.top_left(),
            );
            p.actor_adder
                .add_particle_firework(p.general.position.top_left(), 4);

            p.general.is_alive = false;
        }
    }

    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        if self.counter > 10 && !self.touching_hero {
            self.touching_hero = true;
            p.general.hurts_hero = true;
        }
    }
}
