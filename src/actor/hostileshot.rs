use crate::{
    actor::{
        ActParameters, Actor, CreateActorWithDetails, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, HorizontalDirection, Result, Sizes, OBJECT_HOSTILESHOT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct HostileShot {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
    is_alive: bool,
    direction: HorizontalDirection,
}

impl CreateActorWithDetails for HostileShot {
    type Details = HorizontalDirection;

    fn create_with_details(
        direction: HorizontalDirection,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        let tile = match direction {
            HorizontalDirection::Left => OBJECT_HOSTILESHOT,
            HorizontalDirection::Right => OBJECT_HOSTILESHOT + 2,
        };

        Self {
            tile,
            current_frame: 0,
            num_frames: 2,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
            direction,
        }
    }
}

impl Actor for HostileShot {
    fn act(&mut self, p: ActParameters) {
        let offset =
            (p.sizes.width() as i32) * self.direction.as_factor_i32();
        self.position.offset(offset, 0);

        if p.solids.get(
            self.position.x() as u32 / p.sizes.width(),
            self.position.y() as u32 / p.sizes.height(),
        ) {
            self.is_alive = false;
        }

        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.position.has_intersection(hero.position.geometry)
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }
}
