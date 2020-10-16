use super::super::super::hero::HeroData;
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{ActorData, ActorMessageQueue, ActorQueue};
use crate::{
    HALFTILE_HEIGHT, OBJECT_ELEVATOR_TOP, SOLID_ELEVATOR, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Idle,
    Ascending,
    Descending,
}

#[derive(Debug)]
struct Specific {
    state: State,
}

fn create(
    general: &mut ActorData,
    _level_data: &mut LevelData,
) -> Specific {
    general.position.w = TILE_WIDTH as u16 * 2;
    general.position.h = TILE_HEIGHT as u16;
    general.is_in_foreground = true;

    Specific { state: State::Idle }
}

fn act(
    general: &mut ActorData,
    specific: &mut Specific,
    level_data: &mut LevelData,
    _actor_queue: &mut ActorQueue,
    hero_data: &mut HeroData,
) {
    let hero_geometry = hero_data.position.geometry;

    if specific.state == State::Ascending
        || specific.state == State::Idle
            && general.position.h as usize > TILE_HEIGHT
    {
        // check if hero leaves elevator
        if !hero_geometry.touches(general.position)
            || general.position.x != hero_geometry.x
        {
            specific.state = State::Descending;
        }
    }

    match specific.state {
        State::Ascending => {
            if level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT - 3,
            ) {
                // hero touches solid with head
                specific.state = State::Idle;
            } else {
                let offset = hero_data.position.push_vertically(
                    &level_data.solids,
                    -(TILE_HEIGHT as i16),
                );
                if -offset < TILE_HEIGHT as i16 {
                    hero_data
                        .position
                        .push_vertically(&level_data.solids, -offset);
                    specific.state = State::Idle;
                } else {
                    general.position.h += (-offset) as u16;
                    general.position.y += offset as i16;

                    level_data.solids.set(
                        general.position.x as usize / TILE_WIDTH,
                        general.position.y as usize / TILE_HEIGHT,
                        true,
                    );
                }
            }
        }
        State::Descending => {
            for _ in 0..2 {
                if general.position.h as usize > TILE_HEIGHT {
                    level_data.solids.set(
                        general.position.x as usize / TILE_WIDTH,
                        general.position.y as usize / TILE_HEIGHT,
                        false,
                    );
                    general.position.y += TILE_HEIGHT as i16;
                    general.position.h -= TILE_HEIGHT as u16;
                } else {
                    specific.state = State::Idle;
                }
            }
        }
        State::Idle => {}
    }
}

fn hero_interact_start(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    hero_data: &mut HeroData,
    _info_message_queue: &mut InfoMessageQueue,
    _actor_message_queue: &mut ActorMessageQueue,
) {
    if hero_data.position.geometry.touches(general.position)
        && hero_data.position.geometry.y
            + hero_data.position.geometry.h as i16
            == general.position.y
    {
        specific.state = State::Ascending;
    }
}

fn hero_interact_end(
    general: &mut ActorData,
    specific: &mut Specific,
    _level_data: &mut LevelData,
    hero_data: &mut HeroData,
) {
    if hero_data.position.geometry.touches(general.position)
        && hero_data.position.geometry.x == general.position.x
    {
        specific.state = State::Idle;
    } else {
        specific.state = State::Descending;
    }
}

fn blit(
    general: &mut ActorData,
    _specific: &mut Specific,
    _hero_data: &mut HeroData,
    tilecache: &TileCache,
    target: &mut Surface,
) {
    let tile = tilecache.get_tile(SOLID_ELEVATOR).unwrap();
    let mut destrect = general.position;
    for _ in 0..(general.position.h as usize / TILE_HEIGHT - 1) * 2 {
        destrect.y += HALFTILE_HEIGHT as i16;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
    destrect = general.position;
    let tile = tilecache.get_tile(OBJECT_ELEVATOR_TOP).unwrap();
    tile.blit_to_sdl_surface(None, target, Some(destrect));
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroInteractEndParams,
        FnLevelActorHeroInteractStartParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call(super::create);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_act(
        p: FnLevelActorActParams,
    ) {
        p.call(super::act);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_hero_interact_start(
        p: FnLevelActorHeroInteractStartParams,
    ) {
        p.call(super::hero_interact_start);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_hero_interact_end(
        p: FnLevelActorHeroInteractEndParams,
    ) {
        p.call(super::hero_interact_end);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_elevator_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call(super::blit);
    }
}
