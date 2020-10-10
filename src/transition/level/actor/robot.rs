use super::super::super::{HorizontalDirection, VerticalDirection};

struct Specific {
    direction: VerticalDirection,
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: bool,
    touching_hero: bool,
}

pub mod ffi {
    use super::super::super::super::{
        HorizontalDirection, VerticalDirection,
    };
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchEndParams, FnLevelActorHeroTouchStartParams,
        FnLevelActorShotParams,
    };
    use super::super::{ActorQueueItem, ActorType};
    use super::Specific;
    use crate::{
        ANIMATION_WALLCRAWLERBOT_LEFT, ANIMATION_WALLCRAWLERBOT_RIGHT,
        HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH,
    };
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_create(
        p: FnLevelActorCreateParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*p.specific) };

        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        let (tile, orientation) = match general.actor_type {
            ActorType::WallCrawlerBotLeft => {
                (ANIMATION_WALLCRAWLERBOT_LEFT, HorizontalDirection::Left)
            }
            ActorType::WallCrawlerBotRight => (
                ANIMATION_WALLCRAWLERBOT_RIGHT,
                HorizontalDirection::Right,
            ),
            _ => unreachable!(),
        };

        let data = Box::new(Specific {
            direction: VerticalDirection::Up,
            orientation,
            tile,
            current_frame: 0,
            num_frames: 4,
            was_shot: false,
            touching_hero: false,
        });

        *specific = Box::into_raw(data) as *mut libc::c_void;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_free(
        p: FnLevelActorFreeParams,
    ) {
        unsafe {
            if !(*p.specific).is_null() {
                Box::from_raw(*p.specific);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        general.hurts_hero = true;
        specific.touching_hero = true;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_hero_touch_end(
        p: FnLevelActorHeroTouchEndParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };

        general.hurts_hero = false;
        specific.touching_hero = false;
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_act(
        p: FnLevelActorActParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.level_data.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let level_data = unsafe { &mut (*p.level_data) };

        let direction = match specific.direction {
            VerticalDirection::Up => 1,
            VerticalDirection::Down => -1,
            VerticalDirection::Center => unreachable!(),
        };
        let orientation = match specific.orientation {
            HorizontalDirection::Left => -1,
            HorizontalDirection::Right => 1,
            HorizontalDirection::Center => unreachable!(),
        };

        if direction > 0 {
            // going up
            specific.current_frame += 1;
            specific.current_frame %= specific.num_frames;

            if
            // bot collides with solid tile
            level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                (general.position.y as usize - 1) / TILE_WIDTH) ||
            // bot has no more wall to stick upon
            !level_data.solids.get(
                (
                    general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (general.position.y as usize - 1) / TILE_HEIGHT)
            {
                general.position.y += 1;
                specific.direction = VerticalDirection::Down;
            } else {
                general.position.y -= 1;
            }
        } else {
            // going down
            if specific.current_frame == 0 {
                specific.current_frame = specific.num_frames;
            }
            specific.current_frame -= 1;

            if
            // bot collides with solid tile
            level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    (
                        general.position.y as usize + TILE_HEIGHT
                    ) / TILE_HEIGHT) ||
            // bot has no more wall to stick upon
            !level_data.solids.get(
                (
                    general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize) as usize /
                TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT)
            {
                general.position.y -= 1;
                specific.direction = VerticalDirection::Up;
            } else {
                general.position.y += 1;
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_blit(
        p: FnLevelActorBlitParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.tilecache.is_null());
        assert!(!p.target.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let tilecache = unsafe { &(*p.tilecache) };
        let target = unsafe { &mut (*p.target) };

        let mut target = Surface { raw: target };

        let tile = tilecache
            .get_tile(specific.tile + specific.current_frame)
            .unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, &mut target, Some(destrect));
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_wallcrawler_shot(
        p: FnLevelActorShotParams,
    ) {
        assert!(!p.general.is_null());
        assert!(!p.specific.is_null());
        assert!(!p.hero_data.is_null());
        assert!(!p.actor_queue.is_null());
        let general = unsafe { &mut (*p.general) };
        let specific = unsafe { &mut (*(p.specific as *mut Specific)) };
        let hero_data = unsafe { &mut (*p.hero_data) };
        let actor_queue = unsafe { &mut (*p.actor_queue) };

        if !specific.was_shot {
            if specific.touching_hero {
                general.hurts_hero = false;
                specific.touching_hero = false;
                specific.current_frame = 0;
            }
            general.is_alive = false;

            hero_data.score.add(100);
            actor_queue.push_back(ActorQueueItem {
                actor_type: ActorType::Steam,
                x: general.position.x as u16,
                y: general.position.y as u16,
            });
            actor_queue.push_back(ActorQueueItem {
                actor_type: ActorType::Explosion,
                x: general.position.x as u16,
                y: general.position.y as u16,
            });
        }
    }
}
