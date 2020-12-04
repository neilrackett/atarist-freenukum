use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ActorType, HeroInteractStartParameters,
        RenderParameters,
    },
    hero::InventoryItem,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_ACCESS_CARD_SLOT, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = false;

        Specific {
            tile: OBJECT_ACCESS_CARD_SLOT,
            current_frame: 0,
            num_frames: 8,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        if p.hero_data.inventory.is_set(InventoryItem::AccessCard) {
            p.actor_message_queue.push_back(
                ActorType::AccessCardDoor,
                ActorMessageType::OpenDoor,
            );
            self.current_frame = 0;
            self.num_frames = 1;
            self.tile = OBJECT_ACCESS_CARD_SLOT + 8;
            p.hero_data.inventory.unset(InventoryItem::AccessCard);
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
            p.general.position.top_left(),
        )?;
        Ok(())
    }
}
