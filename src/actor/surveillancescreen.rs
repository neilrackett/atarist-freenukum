use crate::{
    actor::{
        ActParameters, Actor, CreateActor, HeroInteractStartParameters,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, Sizes, ANIMATION_BADGUYSCREEN,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct SurveillanceScreen {
    position: Rect,
}

impl CreateActor for SurveillanceScreen {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        Self {
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width() * 2,
                sizes.height(),
            ),
        }
    }
}

impl Actor for SurveillanceScreen {
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
        pos = pos.offset(p.sizes.width() as i32, 0);
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
