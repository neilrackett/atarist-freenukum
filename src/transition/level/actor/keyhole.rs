use super::super::super::hero::{HeroData, InventoryItem};
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorData, ActorMessageQueue, ActorMessageType, ActorQueue, ActorType,
};
use crate::{
    OBJECT_KEYHOLE_BLACK, OBJECT_KEYHOLE_BLUE, OBJECT_KEYHOLE_GREEN,
    OBJECT_KEYHOLE_PINK, OBJECT_KEYHOLE_RED, TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    counter: usize,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = false;

    Specific {
        tile: OBJECT_KEYHOLE_BLACK,
        counter: 0,
    }
}

fn act(
    _general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    if specific.counter < 5 {
        specific.counter += 1;
        specific.counter %= 4;
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = match (specific.counter, general.actor_type) {
        (0, _) => specific.tile,
        (_, ActorType::KeyholeRed) => OBJECT_KEYHOLE_RED,
        (_, ActorType::KeyholeBlue) => OBJECT_KEYHOLE_BLUE,
        (_, ActorType::KeyholePink) => OBJECT_KEYHOLE_PINK,
        (_, ActorType::KeyholeGreen) => OBJECT_KEYHOLE_GREEN,
        _ => unreachable!(),
    };

    tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
        None,
        target,
        Some(general.position),
    );
}

fn hero_interact_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    hero_data: &mut HeroData,
    info_message_queue: &mut InfoMessageQueue,
    actor_message_queue: &mut ActorMessageQueue,
) {
    let (required_item, door_actor_type) = match general.actor_type {
        ActorType::KeyholeRed => {
            (InventoryItem::KeyRed, ActorType::DoorRed)
        }
        ActorType::KeyholeBlue => {
            (InventoryItem::KeyBlue, ActorType::DoorBlue)
        }
        ActorType::KeyholePink => {
            (InventoryItem::KeyPink, ActorType::DoorPink)
        }
        ActorType::KeyholeGreen => {
            (InventoryItem::KeyGreen, ActorType::DoorGreen)
        }
        _ => unreachable!(),
    };

    if hero_data.inventory.is_set(required_item) {
        actor_message_queue
            .push_back(door_actor_type, ActorMessageType::OpenDoor);
        specific.counter = 5;
        hero_data.inventory.unset(required_item);
    } else if specific.counter < 5 {
        let color = match required_item {
            InventoryItem::KeyRed => "red",
            InventoryItem::KeyBlue => "blue",
            InventoryItem::KeyPink => "pink",
            InventoryItem::KeyGreen => "green",
            _ => unreachable!(),
        };
        info_message_queue
            .push_back(format!("You don't have the {} key.", color));
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_keyhole_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call(super::hero_interact_start);
    }
}
