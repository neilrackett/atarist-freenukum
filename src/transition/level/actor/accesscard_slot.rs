use super::super::super::hero::{HeroData, InventoryItem};
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorData, ActorInterface, ActorMessageQueue, ActorMessageType,
    ActorQueue, ActorType,
};
use crate::{OBJECT_ACCESS_CARD_SLOT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        Specific {
            tile: OBJECT_ACCESS_CARD_SLOT,
            current_frame: 0,
            num_frames: 8,
        }
    }

    fn hero_interact_start(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        hero_data: &mut HeroData,
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        if hero_data.inventory.is_set(InventoryItem::AccessCard) {
            actor_message_queue.push_back(
                ActorType::AccessCardDoor,
                ActorMessageType::OpenDoor,
            );
            self.current_frame = 0;
            self.num_frames = 1;
            self.tile = OBJECT_ACCESS_CARD_SLOT + 8;
            hero_data.inventory.unset(InventoryItem::AccessCard);
        } else {
            info_message_queue
                .push_back("You don't have the access card\n".to_string());
        }
    }

    fn act(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        tilecache
            .get_tile(self.tile + self.current_frame)
            .unwrap()
            .blit_to_sdl_surface(None, target, Some(general.position));
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
