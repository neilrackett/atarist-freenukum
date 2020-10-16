use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue, ActorType};
use crate::{SOLID_START, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    touch_count: usize,
    touching_hero: bool,
    floor_length: usize,
}

fn create(
    general: &mut ActorData,
    level_data: &mut LevelData,
) -> Specific {
    let mut floor_length = 0;
    while !level_data.solids.get(
        general.position.x as usize / TILE_WIDTH + floor_length,
        general.position.y as usize / TILE_HEIGHT,
    ) {
        level_data.solids.set(
            general.position.x as usize / TILE_WIDTH + floor_length,
            general.position.y as usize / TILE_HEIGHT,
            true,
        );
        floor_length += 1;
    }

    general.position.w = (TILE_WIDTH * floor_length) as u16;
    general.position.h = TILE_HEIGHT as u16;

    Specific {
        tile: SOLID_START + 77,
        touch_count: 0,
        touching_hero: false,
        floor_length,
    }
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    // Detect whether the hero is standing upon the floor.
    // We can't use the hero_touch_start functionality here
    // because it only gets triggered when the hero geometry
    // overlaps with the part, which is not the case here.
    let hero_geometry = hero_data.position.geometry;
    let hero_center = hero_geometry.x + (hero_geometry.w as i16) / 2;
    let stands_upon = hero_center >= general.position.x
        && hero_center <= general.position.x + general.position.w as i16
        && hero_geometry.y + hero_geometry.h as i16 == general.position.y;

    if stands_upon {
        if !specific.touching_hero {
            specific.touching_hero = true;
            specific.touch_count += 1;
        }
    } else {
        specific.touching_hero = false;
    }

    if specific.touch_count >= 2 {
        let mut r = general.position;
        for _ in 0..specific.floor_length {
            level_data.solids.set(
                r.x as usize / TILE_WIDTH,
                r.y as usize / TILE_HEIGHT,
                false,
            );
            actor_queue.push_back(
                ActorType::Explosion,
                r.x as u16,
                r.y as u16,
            );
            actor_queue.push_particle_firework(r.x as u16, r.y as u16, 4);
            r.x += TILE_WIDTH as i16;
        }
        general.is_alive = false;
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = tilecache.get_tile(specific.tile as usize).unwrap();
    let mut destrect = general.position;
    for _ in 0..specific.floor_length {
        tile.blit_to_sdl_surface(None, target, Some(destrect));
        destrect.x += TILE_WIDTH as i16;
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_unstablefloor_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_unstablefloor_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_unstablefloor_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_unstablefloor_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
