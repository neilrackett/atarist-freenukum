use crate::{
    actor::{
        ActParameters, Actor, ActorMessageType, CreateActorWithDetails,
        HeroInteractStartParameters, RenderParameters,
    },
    hero::InventoryItem,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, KeyColor, Result, OBJECT_KEYHOLE_BLACK, OBJECT_KEYHOLE_BLUE,
    OBJECT_KEYHOLE_GREEN, OBJECT_KEYHOLE_PINK, OBJECT_KEYHOLE_RED,
    TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    counter: usize,
    position: Rect,
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
            tile: OBJECT_KEYHOLE_BLACK,
            counter: 0,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            color,
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, _p: ActParameters) {
        if self.counter < 5 {
            self.counter += 1;
            self.counter %= 4;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = match (self.counter, self.color) {
            (0, _) => self.tile,
            (_, KeyColor::Red) => OBJECT_KEYHOLE_RED,
            (_, KeyColor::Blue) => OBJECT_KEYHOLE_BLUE,
            (_, KeyColor::Pink) => OBJECT_KEYHOLE_PINK,
            (_, KeyColor::Green) => OBJECT_KEYHOLE_GREEN,
        };

        p.renderer.place_tile(tile, self.position.top_left())?;
        Ok(())
    }

    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        let required_item = InventoryItem::Key(self.color);

        if p.hero.inventory.is_set(required_item) {
            p.actor_message_queue
                .push_back(ActorMessageType::OpenDoor(self.color));
            self.counter = 5;
            p.hero.inventory.unset(required_item);
        } else if self.counter < 5 {
            p.info_message_queue.push_back(format!(
                "You don't have the {} key.",
                self.color.to_string()
            ));
        }
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
