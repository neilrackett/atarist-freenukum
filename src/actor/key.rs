use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActorWithDetails,
        RenderParameters, ScoreType,
    },
    hero::InventoryItem,
    level::{solids::LevelSolids, tiles::LevelTiles},
    KeyColor, Result, OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK,
    OBJECT_KEY_RED, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct Specific {
    position: Rect,
    is_alive: bool,
    color: KeyColor,
}

impl CreateActorWithDetails for Specific {
    type Details = KeyColor;

    fn create_with_details(
        color: KeyColor,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            is_alive: true,
            color,
        }
    }
}

impl Actor for Specific {
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
}
