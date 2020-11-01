use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{ANIMATION_BOMB, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

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
    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        self.counter += 1;

        if self.counter < self.explode_threshold {
        } else if self.counter < self.explode_threshold + self.num_flames {
            let distance = self.counter - self.explode_threshold;
            if self.explode_left {
                // explode to the left if possible
                let space_is_free = !solids.get(
                    general.position.x as usize / TILE_WIDTH - distance,
                    general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = solids.get(
                    general.position.x as usize / TILE_WIDTH - distance,
                    general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    actor_adder.add_actor(
                        ActorType::BombFire,
                        general.position.x as u16
                            - distance as u16 * TILE_WIDTH as u16,
                        general.position.y as u16,
                    );
                } else {
                    self.explode_left = false;
                }
            }
            if self.explode_right {
                // explode to the right if possible
                let space_is_free = !solids.get(
                    general.position.x as usize / TILE_WIDTH + distance,
                    general.position.y as usize / TILE_HEIGHT,
                );
                let space_has_solid_below = solids.get(
                    general.position.x as usize / TILE_WIDTH + distance,
                    general.position.y as usize / TILE_HEIGHT + 1,
                );
                if space_is_free && space_has_solid_below {
                    actor_adder.add_actor(
                        ActorType::BombFire,
                        general.position.x as u16
                            + distance as u16 * TILE_WIDTH as u16,
                        general.position.y as u16,
                    );
                } else {
                    self.explode_right = false;
                }
            }
        } else {
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
        if self.counter < self.explode_threshold {
            tilecache
                .get_tile(self.tile + self.current_frame)
                .unwrap()
                .blit_to_sdl_surface(None, target, Some(general.position));
        }
    }
}
