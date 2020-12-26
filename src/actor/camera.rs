use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_CAMERA_CENTER, ANIMATION_CAMERA_LEFT,
    ANIMATION_CAMERA_RIGHT, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self {
        let x = pos.x as u32 / TILE_WIDTH;
        let y = pos.y as u32 / TILE_HEIGHT;
        tiles.copy_from_to(x, y + 1, x, y);

        Specific {
            tile: ANIMATION_CAMERA_CENTER,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        let x = p.hero.position.geometry.x;
        self.tile = if x - 1 > self.position.x {
            ANIMATION_CAMERA_RIGHT
        } else if x + 1 < self.position.x {
            ANIMATION_CAMERA_LEFT
        } else {
            ANIMATION_CAMERA_CENTER
        };
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.general.is_alive = false;
        p.hero.score.add(100);
        p.actor_adder
            .add_actor(ActorType::Score100, self.position.top_left());
        p.actor_adder
            .add_actor(ActorType::Explosion, self.position.top_left());
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
