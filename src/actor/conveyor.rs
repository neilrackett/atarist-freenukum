use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::HorizontalDirection;
use crate::{
    HALFTILE_WIDTH, SOLID_BLACK, SOLID_CONVEYORBELT_CENTER,
    SOLID_CONVEYORBELT_LEFTEND, SOLID_CONVEYORBELT_RIGHTEND, TILE_HEIGHT,
    TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    current_frame: usize,
    num_frames: usize,
    direction: HorizontalDirection,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
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
            tile = tiles.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT,
            );
            if tile as usize == SOLID_CONVEYORBELT_LEFTEND
                || general.position.x == 0
                || tile == 0
            {
                found_begin = true;
                tiles.set(
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
    fn act(&mut self, p: ActParameters) {
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

        let hero_geometry = p.hero_data.position.geometry;

        if hero_geometry.x + hero_geometry.w as i16 > p.general.position.x
            && hero_geometry.x
                < p.general.position.x + p.general.position.w as i16
            && hero_geometry.y + hero_geometry.h as i16
                == p.general.position.y
        {
            p.hero_data
                .position
                .push_horizontally(p.solids, hero_push_offset);
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let mut tile = p
            .tilecache
            .get_tile(SOLID_CONVEYORBELT_LEFTEND + self.current_frame)
            .unwrap();
        let mut destrect = p.general.position;

        let num_elements = p.general.position.w as usize / TILE_WIDTH;
        for i in 0..num_elements {
            if i == num_elements - 1 {
                // right end of the conveyor
                tile = p
                    .tilecache
                    .get_tile(
                        SOLID_CONVEYORBELT_RIGHTEND + self.current_frame,
                    )
                    .unwrap();
            } else if i == 1 {
                // center parts of the conveyor
                tile = p
                    .tilecache
                    .get_tile(
                        SOLID_CONVEYORBELT_CENTER + self.current_frame % 2,
                    )
                    .unwrap();
            }
            tile.blit_to_sdl_surface(None, p.target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }
}
