use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchStartParameters, RenderParameters,
        ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_FALLINGBLOCK, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    counter: usize,
    touching_hero: bool,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.is_in_foreground = true;

        Specific {
            tile: OBJECT_FALLINGBLOCK,
            counter: 0,
            touching_hero: false,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH * 2, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let hero_geometry = p.hero.position.geometry;

        match self.counter {
            0 => {
                let xl = self.position.left();
                let xr = self.position.right();
                let y = self.position.y();
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
                self.position.y -= 1;
                self.counter += 1;
            }
            c if c <= 10 && c % 2 == 1 => {
                self.position.y += 1;
                self.counter += 1;
            }
            _ => {
                if p.solids.get(
                    self.position.x() as u32 / TILE_WIDTH,
                    self.position.y() as u32 / TILE_HEIGHT + 1,
                ) {
                    p.actor_adder.add_actor(
                        ActorType::Steam,
                        self.position.top_left(),
                    );
                    p.actor_adder.add_particle_firework(
                        self.position.top_left(),
                        4,
                    );
                    p.general.is_alive = false;
                } else {
                    self.position.offset(0, TILE_HEIGHT as i32);
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();
        p.renderer.place_tile(self.tile, pos)?;
        pos.x += TILE_WIDTH as i32;
        p.renderer.place_tile(self.tile + 1, pos)?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        if self.counter > 0 {
            p.hero.score.add(500);
            p.actor_adder
                .add_actor(ActorType::Score500, self.position.top_left());
            p.actor_adder
                .add_particle_firework(self.position.top_left(), 4);

            p.general.is_alive = false;
        }
        ShotProcessing::Absorb
    }

    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        if self.counter > 10 && !self.touching_hero {
            self.touching_hero = true;
            p.general.hurts_hero = true;
        }
    }

    fn position(&self) -> Rect {
        self.position
    }
}
