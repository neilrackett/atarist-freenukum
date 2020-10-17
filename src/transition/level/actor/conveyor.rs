use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::super::HorizontalDirection;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{
    HALFTILE_WIDTH, SOLID_BLACK, SOLID_CONVEYORBELT_CENTER,
    SOLID_CONVEYORBELT_LEFTEND, SOLID_CONVEYORBELT_RIGHTEND, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    current_frame: usize,
    num_frames: usize,
    direction: HorizontalDirection,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        level_data: &mut LevelData,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let direction = match general.actor_type {
            ActorType::ConveyorLeftMovingRightEnd => {
                HorizontalDirection::Left
            }
            ActorType::ConveyorRightMovingRightEnd => {
                HorizontalDirection::Right
            }
            _ => unreachable!(),
        };

        // find the beginning of the conveyor belt
        let mut found_begin = false;
        let mut tile;
        while !found_begin {
            general.position.x -= TILE_WIDTH as i16;
            general.position.w += TILE_WIDTH as u16;
            tile = level_data.tiles.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT,
            );
            if tile as usize == SOLID_CONVEYORBELT_LEFTEND
                || general.position.x == 0
                || tile == 0
            {
                found_begin = true;
                level_data.tiles.set(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT,
                    SOLID_BLACK as u16,
                );
            }
        }

        Specific {
            current_frame: 0,
            num_frames: 4,
            direction,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        let hero_push_offset = match self.direction {
            HorizontalDirection::Left => {
                if self.current_frame == 0 {
                    self.current_frame = self.num_frames;
                }
                self.current_frame -= 1;
                -1 * HALFTILE_WIDTH as i16
            }
            HorizontalDirection::Right => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
                HALFTILE_WIDTH as i16
            }
            _ => unreachable!(),
        };

        let hero_geometry = hero_data.position.geometry;

        if hero_geometry.x + hero_geometry.w as i16 > general.position.x
            && hero_geometry.x
                < general.position.x + general.position.w as i16
            && hero_geometry.y + hero_geometry.h as i16
                == general.position.y
        {
            hero_data
                .position
                .push_horizontally(&level_data.solids, hero_push_offset);
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let mut tile = tilecache
            .get_tile(SOLID_CONVEYORBELT_LEFTEND + self.current_frame)
            .unwrap();
        let mut destrect = general.position;

        let num_elements = general.position.w as usize / TILE_WIDTH;
        for i in 0..num_elements {
            if i == num_elements - 1 {
                // right end of the conveyor
                tile = tilecache
                    .get_tile(
                        SOLID_CONVEYORBELT_RIGHTEND + self.current_frame,
                    )
                    .unwrap();
            } else if i == 1 {
                // center parts of the conveyor
                tile = tilecache
                    .get_tile(
                        SOLID_CONVEYORBELT_CENTER + self.current_frame % 2,
                    )
                    .unwrap();
            }
            tile.blit_to_sdl_surface(None, target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }
}
