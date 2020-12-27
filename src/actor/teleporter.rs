use crate::{
    actor::{
        ActParameters, Actor, ActorMessageType,
        CreateActorWithDetails, HeroInteractStartParameters,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, ANIMATION_TELEPORTER1, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    position: Rect,
    index: TeleporterIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeleporterIndex {
    First,
    Second,
}

impl TeleporterIndex {
    fn other(self) -> Self {
        match self {
            TeleporterIndex::First => TeleporterIndex::Second,
            TeleporterIndex::Second => TeleporterIndex::First,
        }
    }
}

impl CreateActorWithDetails for Specific {
    type Details = TeleporterIndex;

    fn create_with_details(
        index: TeleporterIndex,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            index,
        }
    }
}

impl Actor for Specific {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        p.actor_message_queue
            .push_back(ActorMessageType::TeleportTo(self.index.other()))
    }

    fn act(&mut self, _p: ActParameters) {}

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        for i in 0..3 {
            for j in 0..3 {
                let pos = self.position.top_left().offset(
                    (j - 1) * TILE_WIDTH as i32,
                    (i - 2) * TILE_HEIGHT as i32,
                );
                let tile =
                    ANIMATION_TELEPORTER1 + i as usize * 3 + j as usize;
                p.renderer.place_tile(tile, pos)?;
            }
        }
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        match p.message {
            ActorMessageType::TeleportTo(index) if index == self.index => {
                p.hero.position.move_to(
                    self.position.x(),
                    self.position.y() - TILE_HEIGHT as i32,
                );
            }
            _ => {}
        }
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }
}
