use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    hero::InventoryItem,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK,
    OBJECT_KEY_RED, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct Specific {
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        if p.hero.position.geometry.has_intersection(self.position) {
            let item = match p.general.actor_type {
                ActorType::KeyRed => InventoryItem::KeyRed,
                ActorType::KeyBlue => InventoryItem::KeyBlue,
                ActorType::KeyPink => InventoryItem::KeyPink,
                ActorType::KeyGreen => InventoryItem::KeyGreen,
                _ => unreachable!(),
            };

            p.hero.inventory.set(item);
            p.hero.score.add(1000);
            p.actor_adder
                .add_actor(ActorType::Score1000, self.position.top_left());
            p.general.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = match p.general.actor_type {
            ActorType::KeyRed => OBJECT_KEY_RED,
            ActorType::KeyBlue => OBJECT_KEY_BLUE,
            ActorType::KeyPink => OBJECT_KEY_PINK,
            ActorType::KeyGreen => OBJECT_KEY_GREEN,
            _ => unreachable!(),
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
