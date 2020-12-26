use crate::{
    actor::{
        ActParameters, ActorInterface, ActorMessageType, CreateActor,
        HeroInteractStartParameters, RenderParameters,
    },
    hero::InventoryItem,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, OBJECT_ACCESS_CARD_SLOT, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        Specific {
            tile: OBJECT_ACCESS_CARD_SLOT,
            current_frame: 0,
            num_frames: 8,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        if p.hero.inventory.is_set(InventoryItem::AccessCard) {
            p.actor_message_queue
                .push_back(ActorMessageType::OpenDoorAccessCard);
            self.current_frame = 0;
            self.num_frames = 1;
            self.tile = OBJECT_ACCESS_CARD_SLOT + 8;
            p.hero.inventory.unset(InventoryItem::AccessCard);
        } else {
            p.info_message_queue
                .push_back("You don't have the access card\n".to_string());
        }
    }

    fn act(&mut self, _p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
