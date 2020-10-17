use super::super::super::hero::{HeroData, InventoryItem};
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorInterface, ActorQueue, ActorType};
use crate::{
    OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK, OBJECT_KEY_RED,
    TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub struct Specific {}

impl ActorInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        Specific {}
    }

    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        let item = match general.actor_type {
            ActorType::KeyRed => InventoryItem::KeyRed,
            ActorType::KeyBlue => InventoryItem::KeyBlue,
            ActorType::KeyPink => InventoryItem::KeyPink,
            ActorType::KeyGreen => InventoryItem::KeyGreen,
            _ => unreachable!(),
        };

        hero_data.inventory.set(item);
        hero_data.score.add(1000);
        actor_queue.push_back(
            ActorType::Score1000,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
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
        let tile = match general.actor_type {
            ActorType::KeyRed => OBJECT_KEY_RED,
            ActorType::KeyBlue => OBJECT_KEY_BLUE,
            ActorType::KeyPink => OBJECT_KEY_PINK,
            ActorType::KeyGreen => OBJECT_KEY_GREEN,
            _ => unreachable!(),
        };

        tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            target,
            Some(general.position),
        );
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorBlitParams, FnLevelActorCreateParams,
        FnLevelActorHeroTouchStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_key_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_key_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_key_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
