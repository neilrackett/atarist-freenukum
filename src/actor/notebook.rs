use crate::{
    actor::{
        ActParameters, Actor, ActorExt, CreateActor,
        HeroInteractStartParameters, RenderParameters,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Hero, Result, Sizes, OBJECT_NOTEBOOK,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct NoteBook {
    position: Rect,
}

impl CreateActor for NoteBook {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::NoteBook(Self {
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for NoteBook {
    fn act(&mut self, _p: ActParameters) {}

    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        // TODO: implement functionality.
        p.info_message_queue
            .push_back("Not implemented yet.".to_string());
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer
            .place_tile(OBJECT_NOTEBOOK, self.position.top_left())?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }
}
