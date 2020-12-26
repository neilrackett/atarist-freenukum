use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, OBJECT_ELECTRIC_ARC, OBJECT_ELECTRIC_ARC_HURTING,
    TILE_HEIGHT, TILE_WIDTH,
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
        _general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            tile: OBJECT_ELECTRIC_ARC,
            current_frame: 0,
            num_frames: 4,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            is_alive: true,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
        self.tile =
            if p.hero.position.geometry.has_intersection(self.position) {
                OBJECT_ELECTRIC_ARC
            } else {
                OBJECT_ELECTRIC_ARC_HURTING
            }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::Remove {
            return;
        }
        self.is_alive = false;
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
}
