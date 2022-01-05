use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType, CreateActor,
        HeroInteractStartParameters, RenderParameters,
    },
    hero::InventoryItem,
    level::tiles::LevelTiles,
    Hero, Result, Sizes, OBJECT_ACCESS_CARD_SLOT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct AccessCardSlot {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
}

impl CreateActor for AccessCardSlot {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::AccessCardSlot(Self {
            tile: OBJECT_ACCESS_CARD_SLOT,
            current_frame: 0,
            num_frames: 8,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for AccessCardSlot {
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
