use crate::{
    actor::{
        ActParameters, Actor, ActorMessageType, CreateActorWithDetails,
        HeroInteractStartParameters, ReceiveMessageParameters,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, Sizes, ANIMATION_TELEPORTER1,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Teleporter {
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

impl CreateActorWithDetails for Teleporter {
    type Details = TeleporterIndex;

    fn create_with_details(
        index: TeleporterIndex,
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
            index,
        }
    }
}

impl Actor for Teleporter {
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
                    (j - 1) * p.sizes.width() as i32,
                    (i - 2) * p.sizes.height() as i32,
                );
                let tile =
                    ANIMATION_TELEPORTER1 + i as usize * 3 + j as usize;
                p.renderer.place_tile(tile, pos)?;
            }
        }
        Ok(())
    }

    fn can_receive_message(&self, message: ActorMessageType) -> bool {
        message == ActorMessageType::TeleportTo(self.index)
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        p.hero.position.move_to(
            p.sizes,
            self.position.x(),
            self.position.y() - p.sizes.height() as i32,
        );
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }
}
