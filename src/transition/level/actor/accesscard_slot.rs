use super::super::super::hero::{HeroData, InventoryItem};
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorData, ActorMessageQueue, ActorMessageType, ActorQueue, ActorType,
};
use crate::{OBJECT_ACCESS_CARD_SLOT, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
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
    _general: &mut ActorData,
    specific: &mut Specific,
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
        specific.current_frame = 0;
        specific.num_frames = 1;
        specific.tile = OBJECT_ACCESS_CARD_SLOT + 8;
        hero_data.inventory.unset(InventoryItem::AccessCard);
    } else {
        info_message_queue
            .push_back("You don't have the access card\n".to_string());
    }
}

fn act(
    _general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    specific.current_frame += 1;
    specific.current_frame %= specific.num_frames;
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    tilecache
        .get_tile(specific.tile + specific.current_frame)
        .unwrap()
        .blit_to_sdl_surface(None, target, Some(general.position));
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
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call(super::hero_interact_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_accesscard_slot_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
