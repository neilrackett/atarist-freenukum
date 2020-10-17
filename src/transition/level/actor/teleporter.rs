use super::super::super::hero::HeroData;
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorMessageQueue,
    ActorMessageType, ActorQueue, ActorType,
};
use crate::{ANIMATION_TELEPORTER1, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
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

    fn hero_interact_start(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        _hero_data: &mut HeroData,
        _info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        let other = if general.actor_type == ActorType::Teleporter1 {
            ActorType::Teleporter2
        } else {
            ActorType::Teleporter1
        };

        actor_message_queue.push_back(other, ActorMessageType::Teleport);
    }

    fn act(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let mut destrect = general.position;
        for i in 0..3 {
            for j in 0..3 {
                destrect.x =
                    general.position.x - (1 - j) * TILE_WIDTH as i16;
                destrect.y =
                    general.position.y - (2 - i) * TILE_HEIGHT as i16;
                let tile = tilecache
                    .get_tile(
                        ANIMATION_TELEPORTER1
                            + i as usize * 3
                            + j as usize,
                    )
                    .unwrap();
                tile.blit_to_sdl_surface(None, target, Some(destrect));
            }
        }
    }

    fn receive_message(
        &mut self,
        general: &mut ActorData,
        message: ActorMessageType,
        hero_data: &mut HeroData,
        _level_data: &mut LevelData,
    ) {
        if message != ActorMessageType::Teleport {
            return;
        }

        hero_data.position.move_to(
            general.position.x as u16,
            general.position.y as u16 - TILE_HEIGHT as u16,
        );
    }
}
