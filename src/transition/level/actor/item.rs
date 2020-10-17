use super::super::super::hero::HeroData;
use super::super::super::hero::{FetchedLetter, InventoryItem};
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{
    ANIMATION_SODA, HALFTILE_HEIGHT, OBJECT_ACCESS_CARD, OBJECT_BOOT,
    OBJECT_BOX_BLUE, OBJECT_BOX_GREY, OBJECT_BOX_RED,
    OBJECT_CHICKEN_DOUBLE, OBJECT_CHICKEN_SINGLE, OBJECT_CLAMP,
    OBJECT_DISK, OBJECT_FLAG, OBJECT_FOOTBALL, OBJECT_GLOVE, OBJECT_GUN,
    OBJECT_JOYSTICK, OBJECT_LETTER_D, OBJECT_LETTER_E, OBJECT_LETTER_K,
    OBJECT_LETTER_U, OBJECT_NUCLEARMOLECULE, OBJECT_RADIO, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        let (tile, num_frames) = match general.actor_type {
            ActorType::BoxRedSoda | ActorType::BoxRedChicken => {
                (OBJECT_BOX_RED, 1)
            }
            ActorType::BoxBlueFootball
            | ActorType::BoxBlueJoystick
            | ActorType::BoxBlueDisk
            | ActorType::BoxBlueBalloon
            | ActorType::BoxBlueFlag
            | ActorType::BoxBlueRadio => (OBJECT_BOX_BLUE, 1),
            ActorType::BoxGreyEmpty
            | ActorType::BoxGreyBoots
            | ActorType::BoxGreyClamps
            | ActorType::BoxGreyGun
            | ActorType::BoxGreyBomb
            | ActorType::BoxGreyGlove
            | ActorType::BoxGreyFullLife
            | ActorType::BoxGreyAccessCard
            | ActorType::BoxGreyLetterD
            | ActorType::BoxGreyLetterU
            | ActorType::BoxGreyLetterK
            | ActorType::BoxGreyLetterE => (OBJECT_BOX_GREY, 1),
            ActorType::Joystick => (OBJECT_JOYSTICK, 1),
            ActorType::Football => (OBJECT_FOOTBALL, 1),
            ActorType::Flag => (OBJECT_FLAG, 3),
            ActorType::Disk => (OBJECT_DISK, 1),
            ActorType::Radio => (OBJECT_RADIO, 3),
            ActorType::Soda => (ANIMATION_SODA, 4),
            ActorType::Boots => (OBJECT_BOOT, 1),
            ActorType::Gun => (OBJECT_GUN, 1),
            ActorType::FullLife => (OBJECT_NUCLEARMOLECULE, 8),
            ActorType::ChickenSingle => (OBJECT_CHICKEN_SINGLE, 1),
            ActorType::ChickenDouble => (OBJECT_CHICKEN_DOUBLE, 1),
            ActorType::LetterD => (OBJECT_LETTER_D, 1),
            ActorType::LetterU => (OBJECT_LETTER_U, 1),
            ActorType::LetterK => (OBJECT_LETTER_K, 1),
            ActorType::LetterE => (OBJECT_LETTER_E, 1),
            ActorType::AccessCard => (OBJECT_ACCESS_CARD, 1),
            ActorType::Glove => (OBJECT_GLOVE, 1),
            ActorType::Clamps => (OBJECT_CLAMP, 1),
            _ => unreachable!(),
        };

        Specific {
            tile,
            current_frame: 0,
            num_frames,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        match general.actor_type {
            ActorType::LetterD => {
                general.is_alive = false;
                hero_data.fetched_letter_state.picked(FetchedLetter::D);
                hero_data.score.add(500);
                actor_queue.push_back(
                    ActorType::Score500,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::LetterU => {
                general.is_alive = false;
                hero_data.fetched_letter_state.picked(FetchedLetter::U);
                hero_data.score.add(500);
                actor_queue.push_back(
                    ActorType::Score500,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::LetterK => {
                general.is_alive = false;
                hero_data.fetched_letter_state.picked(FetchedLetter::K);
                hero_data.score.add(500);
                actor_queue.push_back(
                    ActorType::Score500,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::LetterE => {
                general.is_alive = false;
                hero_data.fetched_letter_state.picked(FetchedLetter::E);
                hero_data.score.add(500);
                if hero_data.fetched_letter_state.succeeded() {
                    actor_queue.push_back(
                        ActorType::Score10000,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                    hero_data.score.add(10000);
                } else {
                    actor_queue.push_back(
                        ActorType::Score500,
                        general.position.x as u16,
                        general.position.y as u16,
                    );
                    hero_data.score.add(500);
                }
            }
            ActorType::FullLife => {
                hero_data.health.fill_max();
                general.is_alive = false;
                hero_data.score.add(1000);
                actor_queue.push_back(
                    ActorType::Score1000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Gun => {
                hero_data.firepower.increase(1);
                general.is_alive = false;
                hero_data.score.add(1000);
                actor_queue.push_back(
                    ActorType::Score1000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::AccessCard => {
                hero_data.inventory.set(InventoryItem::AccessCard);
                general.is_alive = false;
                hero_data.score.add(1000);
                actor_queue.push_back(
                    ActorType::Score1000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Glove => {
                hero_data.inventory.set(InventoryItem::Glove);
                general.is_alive = false;
                hero_data.score.add(1000);
                actor_queue.push_back(
                    ActorType::Score1000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Boots => {
                hero_data.inventory.set(InventoryItem::Boot);
                general.is_alive = false;
                hero_data.score.add(1000);
                actor_queue.push_back(
                    ActorType::Score1000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Clamps => {
                hero_data.inventory.set(InventoryItem::Clamp);
                general.is_alive = false;
                hero_data.score.add(1000);
                actor_queue.push_back(
                    ActorType::Score1000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Football => {
                general.is_alive = false;
                hero_data.score.add(100);
                actor_queue.push_back(
                    ActorType::Score100,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Disk => {
                general.is_alive = false;
                hero_data.score.add(5000);
                actor_queue.push_back(
                    ActorType::Score5000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Joystick => {
                general.is_alive = false;
                hero_data.score.add(2000);
                actor_queue.push_back(
                    ActorType::Score2000,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::Radio | ActorType::Flag => {
                general.is_alive = false;
                match self.current_frame {
                    0 => {
                        hero_data.score.add(100);
                        actor_queue.push_back(
                            ActorType::Score100,
                            general.position.x as u16,
                            general.position.y as u16,
                        );
                    }
                    1 => {
                        hero_data.score.add(2000);
                        actor_queue.push_back(
                            ActorType::Score2000,
                            general.position.x as u16,
                            general.position.y as u16,
                        );
                    }
                    2 => {
                        hero_data.score.add(5000);
                        actor_queue.push_back(
                            ActorType::Score5000,
                            general.position.x as u16,
                            general.position.y as u16,
                        );
                    }
                    _ => unreachable!(),
                }
            }
            ActorType::Soda => {
                hero_data.health.increase(1);
                general.is_alive = false;
                hero_data.score.add(200);
                actor_queue.push_back(
                    ActorType::Score200,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::ChickenSingle => {
                hero_data.health.increase(1);
                general.is_alive = false;
                hero_data.score.add(100);
                actor_queue.push_back(
                    ActorType::Score100,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            ActorType::ChickenDouble => {
                hero_data.health.increase(2);
                general.is_alive = false;
                hero_data.score.add(200);
                actor_queue.push_back(
                    ActorType::Score200,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
            _ => {}
        }
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if !level_data.solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            // fall down until the actor lands on solid ground
            general.position.y += HALFTILE_HEIGHT as i16;
        }
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

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        let x = general.position.x as u16;
        let y = general.position.y as u16;
        match general.actor_type {
            ActorType::BoxBlueFootball => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Football, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxBlueJoystick => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Joystick, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxBlueDisk => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Disk, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxBlueBalloon => {
                general.is_alive = false;
                actor_queue.push_back(
                    ActorType::Balloon,
                    x,
                    y - TILE_HEIGHT as u16,
                );
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxBlueFlag => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Flag, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxBlueRadio => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Radio, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxRedSoda => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Soda, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxRedChicken => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::ChickenSingle, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyEmpty => {
                general.is_alive = false;
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyBoots => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Boots, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyClamps => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Clamps, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyGun => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Gun, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyBomb => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Bomb, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyGlove => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::Glove, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyFullLife => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::FullLife, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyAccessCard => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::AccessCard, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyLetterD => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::LetterD, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyLetterU => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::LetterU, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyLetterK => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::LetterK, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::BoxGreyLetterE => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::LetterE, x, y);
                actor_queue.push_particle_firework(x, y, 4);
            }
            ActorType::ChickenSingle => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::ChickenDouble, x, y);
            }
            ActorType::Soda => {
                general.is_alive = false;
                actor_queue.push_back(ActorType::SodaFlying, x, y);
            }
            _ => {}
        }
    }
}

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
        FnLevelActorHeroTouchStartParams, FnLevelActorShotParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_item_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_item_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_item_hero_touch_start(
        p: FnLevelActorHeroTouchStartParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_item_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_item_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_item_shot(
        p: FnLevelActorShotParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
