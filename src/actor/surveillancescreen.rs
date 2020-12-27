use crate::{
    actor::{
        ActParameters, Actor, CreateActor,
        HeroInteractStartParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, ANIMATION_BADGUYSCREEN, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    position: Rect,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            position: Rect::new(pos.x, pos.y, TILE_WIDTH * 2, TILE_HEIGHT),
        }
    }
}

impl Actor for Specific {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        // TODO: implement functionality.
        p.info_message_queue
            .push_back("Not implemented yet.".to_string());
    }

    fn act(&mut self, _p: ActParameters) {}

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();
        p.renderer.place_tile(ANIMATION_BADGUYSCREEN, pos)?;
        pos = pos.offset(TILE_WIDTH as i32, 0);
        p.renderer.place_tile(ANIMATION_BADGUYSCREEN + 1, pos)?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
