use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageType, ActorType, HeroInteractStartParameters,
    ReceiveMessageParameters, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{ANIMATION_TELEPORTER1, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
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

    fn render(&mut self, p: RenderParameters) {
        let mut destrect = p.general.position;
        for i in 0..3 {
            for j in 0..3 {
                destrect.x =
                    p.general.position.x - (1 - j) * TILE_WIDTH as i16;
                destrect.y =
                    p.general.position.y - (2 - i) * TILE_HEIGHT as i16;
                let tile = p
                    .tilecache
                    .get_tile(
                        ANIMATION_TELEPORTER1
                            + i as usize * 3
                            + j as usize,
                    )
                    .unwrap();
                tile.blit_to_sdl_surface(None, p.target, Some(destrect));
            }
        }
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::Teleport {
            return;
        }

        p.hero_data.position.move_to(
            p.general.position.x as u16,
            p.general.position.y as u16 - TILE_HEIGHT as u16,
        );
    }
}
