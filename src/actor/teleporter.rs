use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ActorType, HeroInteractStartParameters,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, ANIMATION_TELEPORTER1, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.is_in_foreground = true;

        Specific {
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        let other = if p.general.actor_type == ActorType::Teleporter1 {
            ActorType::Teleporter2
        } else {
            ActorType::Teleporter1
        };

        p.actor_message_queue
            .push_back(other, ActorMessageType::Teleport);
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
        if p.message != ActorMessageType::Teleport {
            return;
        }

        p.hero.position.move_to(
            self.position.x(),
            self.position.y() - TILE_HEIGHT as i32,
        );
    }

    fn position(&self) -> Rect {
        self.position
    }
}
