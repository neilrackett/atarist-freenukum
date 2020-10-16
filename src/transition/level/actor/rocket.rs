use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorQueue};
use crate::{HALFTILE_HEIGHT, OBJECT_ROCKET, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Flying,
}

#[derive(Debug, PartialEq)]
struct Specific {
    state: State,
}

fn create(
    general: &mut ActorData,
    level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16;
    general.position.h = TILE_HEIGHT as u16;

    let tile_x = general.position.x as usize / TILE_WIDTH;
    let tile_y = general.position.y as usize / TILE_HEIGHT;
    level_data
        .tiles
        .copy_from_to(tile_x, tile_y - 1, tile_x, tile_y);

    Specific { state: State::Idle }
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    match specific.state {
        State::Idle => {}
        State::Flying => {
            general.position.y -= HALFTILE_HEIGHT as i16;
            if level_data.solids.collides(general.position) {
                let tile_x = general.position.x as usize / TILE_WIDTH;
                let tile_y = general.position.y as usize / TILE_HEIGHT;
                level_data.solids.set(tile_x, tile_y + 1, false);
                // TODO: trigger a re-rendering of the affected tiles
                level_data.tiles.copy_from_to(
                    tile_x,
                    tile_y - 1,
                    tile_x,
                    tile_y,
                );
            }
        }
    }
}

fn blit(
    general: &mut ActorData,
    specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let mut destrect = general.position;
    destrect.y -= TILE_HEIGHT as i16 * 3;

    let tile = tilecache.get_tile(OBJECT_ROCKET).unwrap();
    tile.blit_to_sdl_surface(None, target, Some(destrect));

    let tile = tilecache.get_tile(OBJECT_ROCKET + 1).unwrap();
    for _ in 0..2 {
        destrect.y += TILE_HEIGHT as i16;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }

    let tile = tilecache.get_tile(OBJECT_ROCKET + 2).unwrap();
    destrect.y += TILE_HEIGHT as i16;
    tile.blit_to_sdl_surface(None, target, Some(destrect));

    let tile = tilecache.get_tile(OBJECT_ROCKET + 3).unwrap();
    destrect.x -= TILE_WIDTH as i16;
    tile.blit_to_sdl_surface(None, target, Some(destrect));

    let tile = tilecache.get_tile(OBJECT_ROCKET + 4).unwrap();
    destrect.x += 2 * TILE_WIDTH as i16;
    tile.blit_to_sdl_surface(None, target, Some(destrect));

    if specific.state == State::Flying {
        let tile = tilecache.get_tile(OBJECT_ROCKET + 6).unwrap();
        destrect.x -= TILE_WIDTH as i16;
        destrect.y += TILE_HEIGHT as i16;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}

fn shot(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    _hero_data: &mut HeroData,
) {
    if specific.state == State::Idle {
        // TODO: create animation
        specific.state = State::Flying;
        let tile_x = general.position.x as usize / TILE_WIDTH;
        let tile_y = (general.position.y as usize
            + general.position.h as usize)
            / TILE_HEIGHT;

        level_data.solids.set(tile_x, tile_y, false);
        // TODO: trigger a re-rendering of the affected tiles
        level_data
            .tiles
            .copy_from_to(tile_x, tile_y + 1, tile_x, tile_y);
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_rocket_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call(super::shot);
    }
}
