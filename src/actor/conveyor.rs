use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    HorizontalDirection, Result, HALFTILE_WIDTH, SOLID_BLACK,
    SOLID_CONVEYORBELT_CENTER, SOLID_CONVEYORBELT_LEFTEND,
    SOLID_CONVEYORBELT_RIGHTEND, TILE_HEIGHT, TILE_WIDTH,
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
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);

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
            general.position.offset(-(TILE_WIDTH as i32), 0);
            general
                .position
                .set_width(general.position.width() + TILE_WIDTH);
            tile = tiles.get(
                general.position.x() as u32 / TILE_WIDTH,
                general.position.y() as u32 / TILE_HEIGHT,
            );
            if tile as usize == SOLID_CONVEYORBELT_LEFTEND
                || general.position.x() <= 0
                || tile == 0
            {
                found_begin = true;
                tiles.set(
                    general.position.x() as u32 / TILE_WIDTH,
                    general.position.y() as u32 / TILE_HEIGHT,
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
                -(HALFTILE_WIDTH as i32)
            }
            HorizontalDirection::Right => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
                HALFTILE_WIDTH as i32
            }
        };

        let hero_geometry = p.hero_data.position.geometry;

        if hero_geometry.right() > p.general.position.left()
            && hero_geometry.left() < p.general.position.right()
            && hero_geometry.bottom() == p.general.position.top()
        {
            p.hero_data
                .position
                .push_horizontally(p.solids, hero_push_offset);
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut tile = SOLID_CONVEYORBELT_LEFTEND + self.current_frame;
        let mut pos = p.general.position.top_left();

        let num_elements = p.general.position.width() / TILE_WIDTH;
        for i in 0..num_elements {
            if i == num_elements - 1 {
                // right end of the conveyor
                tile = SOLID_CONVEYORBELT_RIGHTEND + self.current_frame;
            } else if i == 1 {
                // center parts of the conveyor
                tile = SOLID_CONVEYORBELT_CENTER + self.current_frame % 2;
            }
            p.renderer.place_tile(tile, pos)?;
            pos.x += TILE_WIDTH as i32;
        }
        Ok(())
    }
}
