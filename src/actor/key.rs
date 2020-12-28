use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActorWithDetails,
        RenderParameters, ScoreType,
    },
    hero::InventoryItem,
    level::{
        solids::LevelSolids, tiles::LevelTiles, BackgroundTileStrategy,
    },
    KeyColor, Result, Sizes, OBJECT_KEY_BLUE, OBJECT_KEY_GREEN,
    OBJECT_KEY_PINK, OBJECT_KEY_RED,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct Key {
    position: Rect,
    is_alive: bool,
    color: KeyColor,
}

impl CreateActorWithDetails for Key {
    type Details = KeyColor;

    fn create_with_details(
        color: KeyColor,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        Self {
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
            color,
        }
    }
}

impl Actor for Key {
    fn act(&mut self, p: ActParameters) {
        if p.hero.position.geometry.has_intersection(self.position) {
            let item = InventoryItem::Key(self.color);

            p.hero.inventory.set(item);
            p.hero.score.add(1000);
            p.actor_adder.add_actor(
                ActorType::Score(ScoreType::Score1000),
                self.position.top_left(),
            );
            self.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = match self.color {
            KeyColor::Red => OBJECT_KEY_RED,
            KeyColor::Blue => OBJECT_KEY_BLUE,
            KeyColor::Pink => OBJECT_KEY_PINK,
            KeyColor::Green => OBJECT_KEY_GREEN,
        };
        p.renderer.place_tile(tile, self.position.top_left())?;
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
