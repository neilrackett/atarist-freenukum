use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActorWithDetails,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, Sizes,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct PlaceHolder {
    position: Rect,
}

impl CreateActorWithDetails for PlaceHolder {
    type Details = ActorType;

    fn create_with_details(
        actor_type: ActorType,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        println!(
            "Warning: creating placeholder for unimplemented \
                actor type {:?} at {:?}",
            actor_type, pos
        );
        Self {
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        }
    }
}

impl Actor for PlaceHolder {
    fn act(&mut self, _p: ActParameters) {}

    fn render(&mut self, _p: RenderParameters) -> Result<()> {
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        false
    }
}
