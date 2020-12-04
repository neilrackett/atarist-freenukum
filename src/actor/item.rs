use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchStartParameters, RenderParameters,
        ShotParameters,
    },
    hero::{FetchedLetter, InventoryItem},
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_SODA, HALFTILE_HEIGHT, OBJECT_ACCESS_CARD,
    OBJECT_BOOT, OBJECT_BOX_BLUE, OBJECT_BOX_GREY, OBJECT_BOX_RED,
    OBJECT_CHICKEN_DOUBLE, OBJECT_CHICKEN_SINGLE, OBJECT_CLAMP,
    OBJECT_DISK, OBJECT_FLAG, OBJECT_FOOTBALL, OBJECT_GLOVE, OBJECT_GUN,
    OBJECT_JOYSTICK, OBJECT_LETTER_D, OBJECT_LETTER_E, OBJECT_LETTER_K,
    OBJECT_LETTER_U, OBJECT_NUCLEARMOLECULE, OBJECT_RADIO, TILE_HEIGHT,
    TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
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
            _ => unreachable!(
                "Attempted to load actor type {:?} as item",
                general.actor_type
            ),
        };

        Specific {
            tile,
            current_frame: 0,
            num_frames,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        match p.general.actor_type {
            ActorType::LetterD => {
                p.general.is_alive = false;
                p.hero_data.fetched_letter_state.picked(FetchedLetter::D);
                p.hero_data.score.add(500);
                p.actor_adder.add_actor(
                    ActorType::Score500,
                    p.general.position.top_left(),
                );
            }
            ActorType::LetterU => {
                p.general.is_alive = false;
                p.hero_data.fetched_letter_state.picked(FetchedLetter::U);
                p.hero_data.score.add(500);
                p.actor_adder.add_actor(
                    ActorType::Score500,
                    p.general.position.top_left(),
                );
            }
            ActorType::LetterK => {
                p.general.is_alive = false;
                p.hero_data.fetched_letter_state.picked(FetchedLetter::K);
                p.hero_data.score.add(500);
                p.actor_adder.add_actor(
                    ActorType::Score500,
                    p.general.position.top_left(),
                );
            }
            ActorType::LetterE => {
                p.general.is_alive = false;
                p.hero_data.fetched_letter_state.picked(FetchedLetter::E);
                p.hero_data.score.add(500);
                if p.hero_data.fetched_letter_state.succeeded() {
                    p.actor_adder.add_actor(
                        ActorType::Score10000,
                        p.general.position.top_left(),
                    );
                    p.hero_data.score.add(10000);
                } else {
                    p.actor_adder.add_actor(
                        ActorType::Score500,
                        p.general.position.top_left(),
                    );
                    p.hero_data.score.add(500);
                }
            }
            ActorType::FullLife => {
                p.hero_data.health.fill_max();
                p.general.is_alive = false;
                p.hero_data.score.add(1000);
                p.actor_adder.add_actor(
                    ActorType::Score1000,
                    p.general.position.top_left(),
                );
            }
            ActorType::Gun => {
                p.hero_data.firepower.increase(1);
                p.general.is_alive = false;
                p.hero_data.score.add(1000);
                p.actor_adder.add_actor(
                    ActorType::Score1000,
                    p.general.position.top_left(),
                );
            }
            ActorType::AccessCard => {
                p.hero_data.inventory.set(InventoryItem::AccessCard);
                p.general.is_alive = false;
                p.hero_data.score.add(1000);
                p.actor_adder.add_actor(
                    ActorType::Score1000,
                    p.general.position.top_left(),
                );
            }
            ActorType::Glove => {
                p.hero_data.inventory.set(InventoryItem::Glove);
                p.general.is_alive = false;
                p.hero_data.score.add(1000);
                p.actor_adder.add_actor(
                    ActorType::Score1000,
                    p.general.position.top_left(),
                );
            }
            ActorType::Boots => {
                p.hero_data.inventory.set(InventoryItem::Boot);
                p.general.is_alive = false;
                p.hero_data.score.add(1000);
                p.actor_adder.add_actor(
                    ActorType::Score1000,
                    p.general.position.top_left(),
                );
            }
            ActorType::Clamps => {
                p.hero_data.inventory.set(InventoryItem::Clamp);
                p.general.is_alive = false;
                p.hero_data.score.add(1000);
                p.actor_adder.add_actor(
                    ActorType::Score1000,
                    p.general.position.top_left(),
                );
            }
            ActorType::Football => {
                p.general.is_alive = false;
                p.hero_data.score.add(100);
                p.actor_adder.add_actor(
                    ActorType::Score100,
                    p.general.position.top_left(),
                );
            }
            ActorType::Disk => {
                p.general.is_alive = false;
                p.hero_data.score.add(5000);
                p.actor_adder.add_actor(
                    ActorType::Score5000,
                    p.general.position.top_left(),
                );
            }
            ActorType::Joystick => {
                p.general.is_alive = false;
                p.hero_data.score.add(2000);
                p.actor_adder.add_actor(
                    ActorType::Score2000,
                    p.general.position.top_left(),
                );
            }
            ActorType::Radio | ActorType::Flag => {
                p.general.is_alive = false;
                match self.current_frame {
                    0 => {
                        p.hero_data.score.add(100);
                        p.actor_adder.add_actor(
                            ActorType::Score100,
                            p.general.position.top_left(),
                        );
                    }
                    1 => {
                        p.hero_data.score.add(2000);
                        p.actor_adder.add_actor(
                            ActorType::Score2000,
                            p.general.position.top_left(),
                        );
                    }
                    2 => {
                        p.hero_data.score.add(5000);
                        p.actor_adder.add_actor(
                            ActorType::Score5000,
                            p.general.position.top_left(),
                        );
                    }
                    _ => unreachable!(),
                }
            }
            ActorType::Soda => {
                p.hero_data.health.increase(1);
                p.general.is_alive = false;
                p.hero_data.score.add(200);
                p.actor_adder.add_actor(
                    ActorType::Score200,
                    p.general.position.top_left(),
                );
            }
            ActorType::ChickenSingle => {
                p.hero_data.health.increase(1);
                p.general.is_alive = false;
                p.hero_data.score.add(100);
                p.actor_adder.add_actor(
                    ActorType::Score100,
                    p.general.position.top_left(),
                );
            }
            ActorType::ChickenDouble => {
                p.hero_data.health.increase(2);
                p.general.is_alive = false;
                p.hero_data.score.add(200);
                p.actor_adder.add_actor(
                    ActorType::Score200,
                    p.general.position.top_left(),
                );
            }
            _ => {}
        }
    }

    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if !p.solids.get(
            p.general.position.x() as u32 / TILE_WIDTH,
            p.general.position.y() as u32 / TILE_HEIGHT + 1,
        ) {
            // fall down until the actor lands on solid ground
            p.general.position.offset(0, HALFTILE_HEIGHT as i32);
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            p.general.position.top_left(),
        )?;
        Ok(())
    }

    fn can_get_shot(&self, general: &ActorData) -> bool {
        matches!(
            general.actor_type,
            ActorType::BoxBlueFootball
                | ActorType::BoxBlueJoystick
                | ActorType::BoxBlueDisk
                | ActorType::BoxBlueBalloon
                | ActorType::BoxBlueFlag
                | ActorType::BoxBlueRadio
                | ActorType::BoxRedSoda
                | ActorType::BoxRedChicken
                | ActorType::BoxGreyEmpty
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
                | ActorType::BoxGreyLetterE
                | ActorType::ChickenSingle
                | ActorType::Soda
        )
    }

    fn shot(&mut self, p: ShotParameters) {
        let pos = p.general.position.top_left();
        match p.general.actor_type {
            ActorType::BoxBlueFootball => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Football, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxBlueJoystick => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Joystick, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxBlueDisk => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Disk, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxBlueBalloon => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(
                    ActorType::Balloon,
                    pos.offset(0, -(TILE_HEIGHT as i32)),
                );
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxBlueFlag => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Flag, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxBlueRadio => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Radio, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxRedSoda => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Soda, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxRedChicken => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::ChickenSingle, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyEmpty => {
                p.general.is_alive = false;
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyBoots => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Boots, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyClamps => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Clamps, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyGun => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Gun, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyBomb => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Bomb, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyGlove => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Glove, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyFullLife => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::FullLife, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyAccessCard => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::AccessCard, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyLetterD => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterD, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyLetterU => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterU, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyLetterK => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterK, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::BoxGreyLetterE => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterE, pos);
                p.actor_adder.add_particle_firework(pos, 4);
            }
            ActorType::ChickenSingle => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::ChickenDouble, pos);
            }
            ActorType::Soda => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::SodaFlying, pos);
            }
            _ => {}
        }
    }
}
