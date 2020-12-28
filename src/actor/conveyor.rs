use crate::{
    actor::{
        ActParameters, Actor, CreateActorWithDetails, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    HorizontalDirection, Result, Sizes, SOLID_BLACK,
    SOLID_CONVEYORBELT_CENTER, SOLID_CONVEYORBELT_LEFTEND,
    SOLID_CONVEYORBELT_RIGHTEND,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Conveyor {
    current_frame: usize,
    num_frames: usize,
    direction: HorizontalDirection,
    position: Rect,
}

impl CreateActorWithDetails for Conveyor {
    type Details = HorizontalDirection;

    fn create_with_details(
        direction: HorizontalDirection,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self {
        // find the beginning of the conveyor belt
        let mut found_begin = false;
        let mut tile;
        let mut position =
            Rect::new(pos.x, pos.y, sizes.width(), sizes.height());
        while !found_begin {
            position.offset(-(sizes.width() as i32), 0);
            position.set_width(position.width() + sizes.width());
            tile = tiles.get(
                position.x() / sizes.width() as i32,
                position.y() / sizes.height() as i32,
            );
            if tile as usize == SOLID_CONVEYORBELT_LEFTEND
                || position.x() <= 0
                || tile == 0
            {
                found_begin = true;
                tiles.set(
                    position.x() / sizes.width() as i32,
                    position.y() / sizes.height() as i32,
                    SOLID_BLACK as u16,
                );
            }
        }

        Self {
            current_frame: 0,
            num_frames: 4,
            direction,
            position,
        }
    }
}

impl Actor for Conveyor {
    fn act(&mut self, p: ActParameters) {
        let hero_push_offset = match self.direction {
            HorizontalDirection::Left => {
                if self.current_frame == 0 {
                    self.current_frame = self.num_frames;
                }
                self.current_frame -= 1;
                -(p.sizes.half_width() as i32)
            }
            HorizontalDirection::Right => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
                p.sizes.half_width() as i32
            }
        };

        let hero_geometry = p.hero.position.geometry;

        if hero_geometry.right() > self.position.left()
            && hero_geometry.left() < self.position.right()
            && hero_geometry.bottom() == self.position.top()
        {
            p.hero.position.push_horizontally(
                p.sizes,
                p.solids,
                hero_push_offset,
            );
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut tile = SOLID_CONVEYORBELT_LEFTEND + self.current_frame;
        let mut pos = self.position.top_left();

        let num_elements = self.position.width() / p.sizes.width();
        for i in 0..num_elements {
            if i == num_elements - 1 {
                // right end of the conveyor
                tile = SOLID_CONVEYORBELT_RIGHTEND + self.current_frame;
            } else if i == 1 {
                // center parts of the conveyor
                tile = SOLID_CONVEYORBELT_CENTER + self.current_frame % 2;
            }
            p.renderer.place_tile(tile, pos)?;
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
}
