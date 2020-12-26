use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, OBJECT_HOSTILESHOT, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
    is_alive: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let tile = match general.actor_type {
            ActorType::HostileShotLeft => OBJECT_HOSTILESHOT,
            ActorType::HostileShotRight => OBJECT_HOSTILESHOT + 2,
            _ => unreachable!(
                "Passed actor type {:?} to hostile shot actor \
                which is not a shot actor id",
                general.actor_type
            ),
        };

        Specific {
            tile,
            current_frame: 0,
            num_frames: 2,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            is_alive: true,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let offset = match p.general.actor_type {
            ActorType::HostileShotLeft => -(TILE_WIDTH as i32),
            ActorType::HostileShotRight => TILE_WIDTH as i32,
            _ => unreachable!(),
        };
        self.position.offset(offset, 0);

        if p.solids.get(
            self.position.x() as u32 / TILE_WIDTH,
            self.position.y() as u32 / TILE_HEIGHT,
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
