use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{ANIMATION_BOMB, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug, PartialEq)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    counter: usize,
    explode_left: bool,
    explode_right: bool,
    explode_threshold: usize,
    num_flames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

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
                    p.general.position.x as usize / TILE_WIDTH - distance,
                    p.general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = p.solids.get(
                    p.general.position.x as usize / TILE_WIDTH - distance,
                    p.general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    p.actor_adder.add_actor(
                        ActorType::BombFire,
                        p.general.position.x as u16
                            - distance as u16 * TILE_WIDTH as u16,
                        p.general.position.y as u16,
                    );
                } else {
                    self.explode_left = false;
                }
            }
            if self.explode_right {
                // explode to the right if possible
                let space_is_free = !p.solids.get(
                    p.general.position.x as usize / TILE_WIDTH + distance,
                    p.general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = p.solids.get(
                    p.general.position.x as usize / TILE_WIDTH + distance,
                    p.general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    p.actor_adder.add_actor(
                        ActorType::BombFire,
                        p.general.position.x as u16
                            + distance as u16 * TILE_WIDTH as u16,
                        p.general.position.y as u16,
                    );
                } else {
                    self.explode_right = false;
                }
            }
        } else {
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) {
        if self.counter < self.explode_threshold {
            p.tilecache
                .get_tile(self.tile + self.current_frame)
                .unwrap()
                .blit_to_sdl_surface(
                    None,
                    p.target,
                    Some(p.general.position),
                );
        }
    }
}
