use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ActorType, HeroInteractStartParameters,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_TELEPORTER1, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = true;

        Specific {}
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self) -> bool {
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
                let pos = p.general.position.top_left().offset(
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

        p.hero_data.position.move_to(
            p.general.position.x(),
            p.general.position.y() - TILE_HEIGHT as i32,
        );
    }
}
