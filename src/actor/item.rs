use crate::{
    actor::{
        ActParameters, ActorAdder, ActorCreateInterface, ActorData,
        ActorInterface, ActorType, RenderParameters, ShotParameters,
        ShotProcessing,
    },
    hero::{FetchedLetter, InventoryItem},
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, ANIMATION_SODA, HALFTILE_HEIGHT, OBJECT_ACCESS_CARD,
    OBJECT_BOOT, OBJECT_BOX_BLUE, OBJECT_BOX_GREY, OBJECT_BOX_RED,
    OBJECT_CHICKEN_DOUBLE, OBJECT_CHICKEN_SINGLE, OBJECT_CLAMP,
    OBJECT_DISK, OBJECT_FLAG, OBJECT_FOOTBALL, OBJECT_GLOVE, OBJECT_GUN,
    OBJECT_JOYSTICK, OBJECT_LETTER_D, OBJECT_LETTER_E, OBJECT_LETTER_K,
    OBJECT_LETTER_U, OBJECT_NUCLEARMOLECULE, OBJECT_RADIO, TILE_HEIGHT,
    TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
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
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl Specific {
    fn touched_by_hero(
        &mut self,
        actor_type: ActorType,
        general: &mut ActorData,
        hero: &mut Hero,
        actor_adder: &mut dyn ActorAdder,
    ) {
        match actor_type {
            ActorType::LetterD => {
                general.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::D);
                hero.score.add(500);
                actor_adder.add_actor(
                    ActorType::Score500,
                    self.position.top_left(),
                );
            }
            ActorType::LetterU => {
                general.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::U);
                hero.score.add(500);
                actor_adder.add_actor(
                    ActorType::Score500,
                    self.position.top_left(),
                );
            }
            ActorType::LetterK => {
                general.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::K);
                hero.score.add(500);
                actor_adder.add_actor(
                    ActorType::Score500,
                    self.position.top_left(),
                );
            }
            ActorType::LetterE => {
                general.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::E);
                hero.score.add(500);
                if hero.fetched_letter_state.succeeded() {
                    actor_adder.add_actor(
                        ActorType::Score10000,
                        self.position.top_left(),
                    );
                    hero.score.add(10000);
                } else {
                    actor_adder.add_actor(
                        ActorType::Score500,
                        self.position.top_left(),
                    );
                    hero.score.add(500);
                }
            }
            ActorType::FullLife => {
                hero.health.fill_max();
                general.is_alive = false;
                hero.score.add(1000);
                actor_adder.add_actor(
                    ActorType::Score1000,
                    self.position.top_left(),
                );
            }
            ActorType::Gun => {
                hero.firepower.increase(1);
                general.is_alive = false;
                hero.score.add(1000);
                actor_adder.add_actor(
                    ActorType::Score1000,
                    self.position.top_left(),
                );
            }
            ActorType::AccessCard => {
                hero.inventory.set(InventoryItem::AccessCard);
                general.is_alive = false;
                hero.score.add(1000);
                actor_adder.add_actor(
                    ActorType::Score1000,
                    self.position.top_left(),
                );
            }
            ActorType::Glove => {
                hero.inventory.set(InventoryItem::Glove);
                general.is_alive = false;
                hero.score.add(1000);
                actor_adder.add_actor(
                    ActorType::Score1000,
                    self.position.top_left(),
                );
            }
            ActorType::Boots => {
                hero.inventory.set(InventoryItem::Boot);
                general.is_alive = false;
                hero.score.add(1000);
                actor_adder.add_actor(
                    ActorType::Score1000,
                    self.position.top_left(),
                );
            }
            ActorType::Clamps => {
                hero.inventory.set(InventoryItem::Clamp);
                general.is_alive = false;
                hero.score.add(1000);
                actor_adder.add_actor(
                    ActorType::Score1000,
                    self.position.top_left(),
                );
            }
            ActorType::Football => {
                general.is_alive = false;
                hero.score.add(100);
                actor_adder.add_actor(
                    ActorType::Score100,
                    self.position.top_left(),
                );
            }
            ActorType::Disk => {
                general.is_alive = false;
                hero.score.add(5000);
                actor_adder.add_actor(
                    ActorType::Score5000,
                    self.position.top_left(),
                );
            }
            ActorType::Joystick => {
                general.is_alive = false;
                hero.score.add(2000);
                actor_adder.add_actor(
                    ActorType::Score2000,
                    self.position.top_left(),
                );
            }
            ActorType::Radio | ActorType::Flag => {
                general.is_alive = false;
                match self.current_frame {
                    0 => {
                        hero.score.add(100);
                        actor_adder.add_actor(
                            ActorType::Score100,
                            self.position.top_left(),
                        );
                    }
                    1 => {
                        hero.score.add(2000);
                        actor_adder.add_actor(
                            ActorType::Score2000,
                            self.position.top_left(),
                        );
                    }
                    2 => {
                        hero.score.add(5000);
                        actor_adder.add_actor(
                            ActorType::Score5000,
                            self.position.top_left(),
                        );
                    }
                    _ => unreachable!(),
                }
            }
            ActorType::Soda => {
                hero.health.increase(1);
                general.is_alive = false;
                hero.score.add(200);
                actor_adder.add_actor(
                    ActorType::Score200,
                    self.position.top_left(),
                );
            }
            ActorType::ChickenSingle => {
                hero.health.increase(1);
                general.is_alive = false;
                hero.score.add(100);
                actor_adder.add_actor(
                    ActorType::Score100,
                    self.position.top_left(),
                );
            }
            ActorType::ChickenDouble => {
                hero.health.increase(2);
                general.is_alive = false;
                hero.score.add(200);
                actor_adder.add_actor(
                    ActorType::Score200,
                    self.position.top_left(),
                );
            }
            _ => {}
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if !p.solids.get(
            self.position.x() as u32 / TILE_WIDTH,
            self.position.y() as u32 / TILE_HEIGHT + 1,
        ) {
            // fall down until the actor lands on solid ground
            self.position.offset(0, HALFTILE_HEIGHT as i32);
        }

        if self.position.has_intersection(p.hero.position.geometry) {
            self.touched_by_hero(
                p.general.actor_type,
                p.general,
                p.hero,
                p.actor_adder,
            );
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
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

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        let pos = self.position.top_left();
        match p.general.actor_type {
            ActorType::BoxBlueFootball => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Football, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxBlueJoystick => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Joystick, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxBlueDisk => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Disk, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxBlueBalloon => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(
                    ActorType::Balloon,
                    pos.offset(0, -(TILE_HEIGHT as i32)),
                );
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxBlueFlag => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Flag, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxBlueRadio => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Radio, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxRedSoda => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Soda, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxRedChicken => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::ChickenSingle, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyEmpty => {
                p.general.is_alive = false;
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyBoots => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Boots, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyClamps => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Clamps, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyGun => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Gun, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyBomb => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Bomb, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyGlove => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::Glove, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyFullLife => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::FullLife, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyAccessCard => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::AccessCard, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyLetterD => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterD, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyLetterU => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterU, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyLetterK => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterK, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::BoxGreyLetterE => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::LetterE, pos);
                p.actor_adder.add_particle_firework(pos, 4);
                ShotProcessing::Absorb
            }
            ActorType::ChickenSingle => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::ChickenDouble, pos);
                ShotProcessing::Absorb
            }
            ActorType::Soda => {
                p.general.is_alive = false;
                p.actor_adder.add_actor(ActorType::SodaFlying, pos);
                ShotProcessing::Absorb
            }
            _ => ShotProcessing::Ignore,
        }
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }
}
