use super::super::super::hero::HeroData;
use super::super::super::level::solids::LevelSolids;
use super::super::super::level::tiles::LevelTiles;
use super::super::super::tilecache::TileCache;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::{ANIMATION_FAN, HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    running: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.y -= TILE_HEIGHT as i16;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16 * 2;

        Specific {
            tile: ANIMATION_FAN,
            current_frame: 0,
            num_frames: 4,
            running: 10,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        match self.running {
            0 => {}
            1 => {
                self.current_frame += 1;
            }
            2 => {}
            3 => {}
            4 => {}
            5 => {
                self.current_frame += 1;
            }
            6 => {}
            7 => {}
            8 => {
                self.current_frame += 1;
            }
            9 => {}
            10 => {
                self.current_frame += 1;
            }
            _ => unreachable!(),
        }
        self.current_frame %= self.num_frames;
        if self.running < 10 && self.running > 0 {
            self.running -= 1;
        } else if self.running == 10 {
            if hero_data
                .position
                .geometry
                .overlaps_vertically(general.position)
            {
                let mut hdistance = hero_data
                    .position
                    .geometry
                    .horizontal_distance(general.position);

                let fan_direction = match general.actor_type {
                    ActorType::FanLeft => -1,
                    ActorType::FanRight => 1,
                    _ => unreachable!(),
                };
                if (fan_direction > 0 && hdistance > 0)
                    || (fan_direction < 0 && hdistance < 0)
                {
                    return;
                }

                if hdistance == 0 {
                    hdistance = HALFTILE_WIDTH as i32 * fan_direction;
                }

                if hdistance.abs() < 8 * HALFTILE_WIDTH as i32 {
                    hero_data.position.push_horizontally(
                        &solids,
                        fan_direction as i16 * TILE_WIDTH as i16,
                    );
                }
            }
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let mut destrect = general.position;
        tilecache
            .get_tile(self.tile + self.current_frame * 2)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
        destrect.y += TILE_HEIGHT as i16;
        tilecache
            .get_tile(self.tile + self.current_frame * 2 + 1)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(destrect));
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_solids: &mut LevelSolids,
        _level_tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        self.running = 9;
        actor_adder.add_actor(
            ActorType::Steam,
            general.position.x as u16,
            general.position.y as u16,
        );
    }
}
