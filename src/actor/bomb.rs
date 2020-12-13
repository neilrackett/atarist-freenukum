use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_BOMB, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug, PartialEq)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    counter: u32,
    explode_left: bool,
    explode_right: bool,
    explode_threshold: u32,
    num_flames: u32,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.acts_while_invisible = true;

        Specific {
            tile: ANIMATION_BOMB,
            current_frame: 0,
            num_frames: 2,
            counter: 0,
            explode_left: true,
            explode_right: true,
            explode_threshold: 12,
            num_flames: 4,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        self.counter += 1;

        if self.counter < self.explode_threshold {
        } else if self.counter < self.explode_threshold + self.num_flames {
            let distance = self.counter - self.explode_threshold;
            if self.explode_left {
                // explode to the left if possible
                let space_is_free = !p.solids.get(
                    p.general.position.x() as u32 / TILE_WIDTH - distance,
                    p.general.position.y() as u32 / TILE_HEIGHT,
                );
                let space_has_solid_below = p.solids.get(
                    p.general.position.x() as u32 / TILE_WIDTH - distance,
                    p.general.position.y() as u32 / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    p.actor_adder.add_actor(
                        ActorType::BombFire,
                        p.general
                            .position
                            .top_left()
                            .offset(-((distance * TILE_WIDTH) as i32), 0),
                    );
                } else {
                    self.explode_left = false;
                }
            }
            if self.explode_right {
                // explode to the right if possible
                let space_is_free = !p.solids.get(
                    p.general.position.x() as u32 / TILE_WIDTH + distance,
                    p.general.position.y() as u32 / TILE_HEIGHT,
                );
                let space_has_solid_below = p.solids.get(
                    p.general.position.x() as u32 / TILE_WIDTH + distance,
                    p.general.position.y() as u32 / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    p.actor_adder.add_actor(
                        ActorType::BombFire,
                        p.general
                            .position
                            .top_left()
                            .offset((distance * TILE_WIDTH) as i32, 0),
                    );
                } else {
                    self.explode_right = false;
                }
            }
        } else {
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        if self.counter < self.explode_threshold {
            p.renderer.place_tile(
                self.tile + self.current_frame,
                p.general.position.top_left(),
            )?;
        }
        Ok(())
    }
}
