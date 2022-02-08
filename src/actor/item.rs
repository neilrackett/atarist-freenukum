// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActorWithDetails,
        RenderParameters, ScoreType, ShotParameters, ShotProcessing,
    },
    game::GameCommands,
    hero::{FetchedLetter, InventoryItem},
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Hero, Result, Sizes, SoundIndex, ANIMATION_SODA, OBJECT_ACCESS_CARD,
    OBJECT_BOOT, OBJECT_BOX_BLUE, OBJECT_BOX_GREY, OBJECT_BOX_RED,
    OBJECT_CHICKEN_DOUBLE, OBJECT_CHICKEN_SINGLE, OBJECT_CLAMP,
    OBJECT_DISK, OBJECT_FLAG, OBJECT_FOOTBALL, OBJECT_GLOVE, OBJECT_GUN,
    OBJECT_JOYSTICK, OBJECT_LETTER_D, OBJECT_LETTER_E, OBJECT_LETTER_K,
    OBJECT_LETTER_U, OBJECT_NUCLEARMOLECULE, OBJECT_RADIO,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxBlueContent {
    Balloon,
    Disk,
    Flag,
    Football,
    Joystick,
    Radio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxRedContent {
    Chicken,
    Soda,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxGreyContent {
    AccessCard,
    Bomb,
    Boots,
    Clamps,
    FullLife,
    Glove,
    Gun,
    LetterD,
    LetterE,
    LetterK,
    LetterU,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    AccessCard,
    Boots,
    BoxBlue(BoxBlueContent),
    BoxGrey(BoxGreyContent),
    BoxRed(BoxRedContent),
    ChickenDouble,
    ChickenSingle,
    Clamps,
    Disk,
    Flag,
    Football,
    FullLife,
    Glove,
    Gun,
    Joystick,
    LetterD,
    LetterE,
    LetterK,
    LetterU,
    Radio,
    Soda,
}

impl ItemType {
    fn tile_and_num_frames(&self) -> (usize, usize) {
        match self {
            ItemType::AccessCard => (OBJECT_ACCESS_CARD, 1),
            ItemType::Boots => (OBJECT_BOOT, 1),
            ItemType::BoxBlue(_) => (OBJECT_BOX_BLUE, 1),
            ItemType::BoxGrey(_) => (OBJECT_BOX_GREY, 1),
            ItemType::BoxRed(_) => (OBJECT_BOX_RED, 1),
            ItemType::ChickenDouble => (OBJECT_CHICKEN_DOUBLE, 1),
            ItemType::ChickenSingle => (OBJECT_CHICKEN_SINGLE, 1),
            ItemType::Clamps => (OBJECT_CLAMP, 1),
            ItemType::Disk => (OBJECT_DISK, 1),
            ItemType::Flag => (OBJECT_FLAG, 3),
            ItemType::Football => (OBJECT_FOOTBALL, 1),
            ItemType::FullLife => (OBJECT_NUCLEARMOLECULE, 8),
            ItemType::Glove => (OBJECT_GLOVE, 1),
            ItemType::Gun => (OBJECT_GUN, 1),
            ItemType::Joystick => (OBJECT_JOYSTICK, 1),
            ItemType::LetterD => (OBJECT_LETTER_D, 1),
            ItemType::LetterE => (OBJECT_LETTER_E, 1),
            ItemType::LetterK => (OBJECT_LETTER_K, 1),
            ItemType::LetterU => (OBJECT_LETTER_U, 1),
            ItemType::Radio => (OBJECT_RADIO, 3),
            ItemType::Soda => (ANIMATION_SODA, 4),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Item {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
    is_alive: bool,
    item_type: ItemType,
}

impl CreateActorWithDetails for Item {
    type Details = ItemType;

    fn create_with_details(
        item_type: ItemType,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        let (tile, num_frames) = item_type.tile_and_num_frames();

        Actor::Item(Self {
            tile,
            current_frame: 0,
            num_frames,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
            item_type,
        })
    }
}

impl Item {
    fn touched_by_hero(
        &mut self,
        item_type: ItemType,
        hero: &mut Hero,
        game_commands: &mut dyn GameCommands,
    ) {
        match item_type {
            ItemType::LetterD => {
                self.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::D);
                hero.score.add(500);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score500),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETBONUSOBJ);
            }
            ItemType::LetterU => {
                self.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::U);
                hero.score.add(500);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score500),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETBONUSOBJ);
            }
            ItemType::LetterK => {
                self.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::K);
                hero.score.add(500);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score500),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETBONUSOBJ);
            }
            ItemType::LetterE => {
                self.is_alive = false;
                hero.fetched_letter_state.picked(FetchedLetter::E);
                hero.score.add(500);
                if hero.fetched_letter_state.succeeded() {
                    game_commands.add_actor(
                        ActorType::Score(ScoreType::Score10000),
                        self.position.top_left(),
                    );
                    hero.score.add(10000);
                    game_commands.add_sound(SoundIndex::GETDUKESND);
                } else {
                    game_commands.add_actor(
                        ActorType::Score(ScoreType::Score500),
                        self.position.top_left(),
                    );
                    hero.score.add(500);
                    game_commands.add_sound(SoundIndex::GETBONUSOBJ);
                }
            }
            ItemType::FullLife => {
                hero.health.fill_max();
                self.is_alive = false;
                hero.score.add(1000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score1000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETPOWERUP);
            }
            ItemType::Gun => {
                hero.firepower.increase(1);
                self.is_alive = false;
                hero.score.add(1000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score1000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::SPECIALITEM);
            }
            ItemType::AccessCard => {
                hero.inventory.set(InventoryItem::AccessCard);
                self.is_alive = false;
                hero.score.add(1000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score1000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::SPECIALITEM);
            }
            ItemType::Glove => {
                hero.inventory.set(InventoryItem::Glove);
                self.is_alive = false;
                hero.score.add(1000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score1000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::SPECIALITEM);
            }
            ItemType::Boots => {
                hero.inventory.set(InventoryItem::Boot);
                self.is_alive = false;
                hero.score.add(1000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score1000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::SPECIALITEM);
            }
            ItemType::Clamps => {
                hero.inventory.set(InventoryItem::Clamp);
                self.is_alive = false;
                hero.score.add(1000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score1000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::SPECIALITEM);
            }
            ItemType::Football => {
                self.is_alive = false;
                hero.score.add(100);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score100),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETBONUSOBJ);
            }
            ItemType::Disk => {
                self.is_alive = false;
                hero.score.add(5000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score5000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETBONUSOBJ);
            }
            ItemType::Joystick => {
                self.is_alive = false;
                hero.score.add(2000);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score2000),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETBONUSOBJ);
            }
            ItemType::Radio | ItemType::Flag => {
                self.is_alive = false;
                match self.current_frame {
                    0 => {
                        hero.score.add(100);
                        game_commands.add_actor(
                            ActorType::Score(ScoreType::Score100),
                            self.position.top_left(),
                        );
                    }
                    1 => {
                        hero.score.add(2000);
                        game_commands.add_actor(
                            ActorType::Score(ScoreType::Score2000),
                            self.position.top_left(),
                        );
                    }
                    2 => {
                        hero.score.add(5000);
                        game_commands.add_actor(
                            ActorType::Score(ScoreType::Score5000),
                            self.position.top_left(),
                        );
                    }
                    _ => unreachable!(),
                }
                game_commands.add_sound(SoundIndex::GETBONUSOBJ);
            }
            ItemType::Soda => {
                hero.health.increase(1);
                self.is_alive = false;
                hero.score.add(200);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score200),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETFOODITEM);
            }
            ItemType::ChickenSingle => {
                hero.health.increase(1);
                self.is_alive = false;
                hero.score.add(100);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score100),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETFOODITEM);
            }
            ItemType::ChickenDouble => {
                hero.health.increase(2);
                self.is_alive = false;
                hero.score.add(200);
                game_commands.add_actor(
                    ActorType::Score(ScoreType::Score200),
                    self.position.top_left(),
                );
                game_commands.add_sound(SoundIndex::GETFOODITEM);
            }
            _ => {}
        }
    }
}

impl ActorExt for Item {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if p.tiles
            .get(
                self.position.x() / p.sizes.width() as i32,
                self.position.y() / p.sizes.height() as i32 + 1,
            )
            .map(|t| !t.solid)
            .unwrap_or(true)
        {
            // fall down until the actor lands on solid ground
            self.position.offset(0, p.sizes.half_height() as i32);
        }

        if self.position.has_intersection(p.hero.position.geometry) {
            self.touched_by_hero(self.item_type, p.hero, p.game_commands);
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        matches!(
            self.item_type,
            ItemType::BoxBlue(_)
                | ItemType::BoxRed(_)
                | ItemType::BoxGrey(_)
                | ItemType::ChickenSingle
                | ItemType::Soda
        )
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        let pos = self.position.top_left();
        match self.item_type {
            ItemType::BoxBlue(BoxBlueContent::Football) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Football), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxBlue(BoxBlueContent::Joystick) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Joystick), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxBlue(BoxBlueContent::Disk) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Disk), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxBlue(BoxBlueContent::Balloon) => {
                self.is_alive = false;
                p.game_commands.add_actor(
                    ActorType::Balloon,
                    pos.offset(0, -(p.sizes.height() as i32)),
                );
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxBlue(BoxBlueContent::Flag) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Flag), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxBlue(BoxBlueContent::Radio) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Radio), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxRed(BoxRedContent::Soda) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Soda), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxRed(BoxRedContent::Chicken) => {
                self.is_alive = false;
                p.game_commands.add_actor(
                    ActorType::Item(ItemType::ChickenSingle),
                    pos,
                );
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::Nothing) => {
                self.is_alive = false;
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::Boots) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Boots), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::Clamps) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Clamps), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::Gun) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Gun), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::Bomb) => {
                self.is_alive = false;
                p.game_commands.add_actor(ActorType::Bomb, pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::Glove) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::Glove), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::FullLife) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::FullLife), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::AccessCard) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::AccessCard), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::LetterD) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::LetterD), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::LetterU) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::LetterU), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::LetterK) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::LetterK), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::BoxGrey(BoxGreyContent::LetterE) => {
                self.is_alive = false;
                p.game_commands
                    .add_actor(ActorType::Item(ItemType::LetterE), pos);
                p.game_commands.add_particle_firework(pos, 4);
                p.game_commands.add_sound(SoundIndex::BOXEXPLODE);
                ShotProcessing::Absorb
            }
            ItemType::ChickenSingle => {
                self.is_alive = false;
                p.game_commands.add_actor(
                    ActorType::Item(ItemType::ChickenDouble),
                    pos,
                );
                ShotProcessing::Absorb
            }
            ItemType::Soda => {
                self.is_alive = false;
                p.game_commands.add_actor(ActorType::SodaFlying, pos);
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

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }
}
