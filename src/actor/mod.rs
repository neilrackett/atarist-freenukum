mod accesscard_door;
mod accesscard_slot;
mod acme;
mod balloon;
mod bomb;
mod camera;
mod conveyor;
mod door;
mod elevator;
mod exitdoor;
mod expandingfloor;
mod fan;
mod fire;
mod firewheelbot;
mod glove_slot;
mod hostileshot;
mod item;
mod key;
mod keyhole;
mod mill;
mod notebook;
mod particle;
mod placeholder;
mod redball_jumping;
mod redball_lying;
mod robot;
mod rocket;
mod score;
mod shootable_wall;
mod simpleanimation;
mod singleanimation;
mod soda_flying;
mod spikes;
mod surveillancescreen;
mod tankbot;
mod teleporter;
mod unstablefloor;
mod wallcrawler;

use crate::{
    geometry::RectExt, hero::HeroData, infobox::InfoMessageQueue,
    level::solids::LevelSolids, level::tiles::LevelTiles,
    rendering::Renderer, Result,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct ActorsList {
    actors: Vec<Actor>,
    interaction_target: Option<usize>,
}

impl Default for ActorsList {
    fn default() -> Self {
        Self::new()
    }
}

impl ActorsList {
    pub fn new() -> Self {
        ActorsList {
            actors: Vec::new(),
            interaction_target: None,
        }
    }

    pub fn count(&self) -> usize {
        self.actors.len()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Actor> {
        self.actors.get_mut(index)
    }

    pub fn remove_dead(&mut self) {
        let mut i = 0;
        while i < self.actors.len() {
            match self.actors.get(i) {
                Some(a) if !a.general.is_alive => {
                    self.actors.remove(i);
                    self.interaction_target = match self.interaction_target
                    {
                        Some(index) if index == i => None,
                        Some(index) if index > i => Some(index - 1),
                        Some(index) => Some(index),
                        None => None,
                    };
                }
                _ => {
                    i += 1;
                }
            }
        }
    }

    pub fn send_message(
        &mut self,
        receivers: ActorType,
        message: ActorMessageType,
        hero_data: &mut HeroData,
        solids: &mut LevelSolids,
    ) {
        for actor in self
            .actors
            .iter_mut()
            .filter(|a| a.general.actor_type == receivers)
        {
            let p = ReceiveMessageParameters {
                general: &mut actor.general,
                message,
                hero_data,
                solids,
            };
            actor.specific.receive_message(p);
        }
    }

    pub fn process_shot(
        &mut self,
        shot_position: Rect,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
    ) -> bool {
        for actor in self.actors.iter_mut() {
            if actor.can_get_shot()
                && shot_position.touches(actor.position())
            {
                if actor.shot(solids, tiles, actor_adder, hero_data)
                    == ShotProcessing::Absorb
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn start_interaction(
        &mut self,
        level_passed: &mut bool,
        hero_data: &mut HeroData,
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        let new_interactor = self.actors.iter().position(|actor| {
            actor.hero_can_interact(hero_data)
                && hero_data
                    .position
                    .geometry
                    .touches(actor.general.position)
        });

        if let Some(i) = new_interactor {
            self.end_interaction(level_passed, hero_data);

            let actor = self.actors.get_mut(i).unwrap();
            let p = HeroInteractStartParameters {
                general: &mut actor.general,
                level_passed,
                hero_data,
                info_message_queue,
                actor_message_queue,
            };
            self.interaction_target = Some(i);
            actor.specific.hero_interact_start(p);
        }
    }

    pub fn end_interaction(
        &mut self,
        level_passed: &mut bool,
        hero_data: &mut HeroData,
    ) {
        if let Some(i) = self.interaction_target.take() {
            if let Some(actor) = self.actors.get_mut(i) {
                let p = HeroInteractEndParameters {
                    general: &mut actor.general,
                    level_passed,
                    hero_data,
                };
                actor.specific.hero_interact_end(p);
            }
        }
    }

    pub fn act(
        &mut self,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        hero_data: &mut HeroData,
        actor_queue: &mut ActorQueue,
        do_play: &mut bool,
    ) {
        let mut actors_hurting_hero = 0usize;
        for actor in self.actors.iter_mut() {
            if actor.general.acts_while_invisible
                || actor.general.is_visible
            {
                actor.act(solids, tiles, hero_data, actor_queue, do_play);
                if actor.general.is_alive && actor.general.hurts_hero {
                    actors_hurting_hero += 1;
                }
            }
        }

        let mut adder = LevelActorAdder {
            solids,
            tiles,
            actors: self,
        };

        actor_queue.process(&mut adder);
        self.remove_dead();
        hero_data.gets_hurt = actors_hurting_hero > 0;
    }

    pub fn update_visibility(&mut self, visible_rect: Rect) {
        self.actors.iter_mut().for_each(|actor| {
            actor.general.is_visible =
                actor.general.position.has_intersection(visible_rect);
        });
    }

    pub fn render_background_actors(
        &mut self,
        renderer: &mut dyn Renderer,
        draw_collision_bounds: bool,
    ) -> Result<()> {
        Ok(self
            .actors
            .iter_mut()
            .filter(|actor| {
                actor.general.is_visible && !actor.general.is_in_foreground
            })
            .map(|actor| actor.render(renderer, draw_collision_bounds))
            .collect::<Result<_>>()?)
    }

    pub fn render_foreground_actors(
        &mut self,
        renderer: &mut dyn Renderer,
        draw_collision_bounds: bool,
    ) -> Result<()> {
        Ok(self
            .actors
            .iter_mut()
            .filter(|actor| {
                actor.general.is_visible && actor.general.is_in_foreground
            })
            .map(|actor| actor.render(renderer, draw_collision_bounds))
            .collect::<Result<_>>()?)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShotProcessing {
    Absorb,
    Ignore,
}

#[derive(Debug)]
pub struct Actor {
    pub(crate) general: ActorData,
    pub(crate) specific: Box<dyn ActorInterface>,
}

impl Actor {
    fn act(
        &mut self,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        hero_data: &mut HeroData,
        actor_adder: &mut dyn ActorAdder,
        do_play: &mut bool,
    ) -> bool {
        self.check_hero_touch(hero_data, actor_adder);

        let p = ActParameters {
            general: &mut self.general,
            solids,
            tiles,
            hero_data,
            actor_adder,
            do_play,
        };
        self.specific.act(p);
        self.general.is_alive
    }

    fn check_hero_touch(
        &mut self,
        hero_data: &mut HeroData,
        actor_adder: &mut dyn ActorAdder,
    ) {
        let touching_hero = self
            .general
            .position
            .has_intersection(hero_data.position.geometry);

        if touching_hero {
            if !self.general.touches_hero {
                self.general.touches_hero = true;

                let p = HeroTouchStartParameters {
                    general: &mut self.general,
                    hero_data,
                    actor_adder,
                };

                self.specific.hero_touch_start(p);
            }
        } else if self.general.touches_hero {
            self.general.touches_hero = false;

            let p = HeroTouchEndParameters {
                general: &mut self.general,
                hero_data,
            };
            self.specific.hero_touch_end(p);
        }
    }

    fn hero_can_interact(&self, hero_data: &HeroData) -> bool {
        if self.general.actor_type == ActorType::Lift {
            /* This check needs to be done for elevator only because
             * if there are two elevators next to each other, the mostleft
             * elevator would be chosen for interaction instead of the one on
             * which the hero stands.
             */
            // TODO: this should be moved into ActorInterface::hero_can_interact
            self.general.position.x == hero_data.position.geometry.x
        } else {
            self.specific.hero_can_interact()
        }
    }

    pub fn can_get_shot(&self) -> bool {
        self.specific.can_get_shot(&self.general)
    }

    pub fn shot(
        &mut self,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
    ) -> ShotProcessing {
        let p = ShotParameters {
            general: &mut self.general,
            solids,
            tiles,
            actor_adder,
            hero_data,
        };
        self.specific.shot(p)
    }

    pub fn is_alive(&self) -> bool {
        self.general.is_alive
    }

    pub fn position(&self) -> Rect {
        self.general.position
    }

    pub fn render(
        &mut self,
        renderer: &mut dyn Renderer,
        draw_collision_bounds: bool,
    ) -> Result<()> {
        let p = RenderParameters {
            general: &mut self.general,
            renderer,
        };
        self.specific.render(p)?;

        if draw_collision_bounds {
            let color = crate::collision_bounds_color();
            renderer.draw_rect(self.general.position, color)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActorType {
    FireWheelBot,
    FlameGnomeBot,
    FlyingBot,
    FootBot,
    HelicopterBot,
    RabbitoidBot,
    RedBallJumping,
    RedBallLying,
    Robot,
    RobotDisappearing,
    SnakeBot,
    TankBot,
    WallCrawlerBotLeft,
    WallCrawlerBotRight,
    DrProton,
    Camera,
    Explosion,
    DustCloud,
    Steam,
    ParticlePink,
    ParticleBlue,
    ParticleWhite,
    ParticleGreen,
    Rocket,
    Bomb,
    BombFire,
    Water,
    ExitDoor,
    Notebook,
    SurveillanceScreen,
    HostileShotLeft,
    HostileShotRight,
    Soda,
    SodaFlying,
    UnstableFloor,
    ExpandingFloor,
    ConveyorLeftMovingRightEnd,
    ConveyorRightMovingRightEnd,
    FanLeft,
    FanRight,
    BrokenWallBackground,
    StoneBackground,
    Teleporter1,
    Teleporter2,
    FenceBackground,
    StoneWindowBackground,
    WindowLeftBackground,
    WindowRightBackground,
    Screen,
    BoxGreyEmpty,
    BoxGreyBoots,
    Boots,
    BoxGreyClamps,
    Clamps,
    BoxGreyGun,
    Gun,
    BoxGreyBomb,
    BoxRedSoda,
    BoxRedChicken,
    ChickenSingle,
    ChickenDouble,
    BoxBlueFootball,
    Football,
    Flag,
    BoxBlueJoystick,
    Joystick,
    BoxBlueDisk,
    Disk,
    BoxBlueBalloon,
    Balloon,
    BoxGreyGlove,
    Glove,
    BoxGreyFullLife,
    FullLife,
    BoxBlueFlag,
    BlueFlag,
    BoxBlueRadio,
    Radio,
    BoxGreyAccessCard,
    AccessCard,
    BoxGreyLetterD,
    LetterD,
    BoxGreyLetterU,
    LetterU,
    BoxGreyLetterK,
    LetterK,
    BoxGreyLetterE,
    LetterE,
    AccessCardSlot,
    GloveSlot,
    KeyRed,
    KeyholeRed,
    DoorRed,
    KeyBlue,
    KeyholeBlue,
    DoorBlue,
    KeyPink,
    KeyholePink,
    DoorPink,
    KeyGreen,
    KeyholeGreen,
    DoorGreen,
    ShootableWall,
    Lift,
    Acme,
    FireRight,
    FireLeft,
    Mill,
    Laserbeam,
    AccessCardDoor,
    SpikesUp,
    SpikesDown,
    Spike,
    Score100,
    Score200,
    Score500,
    Score1000,
    Score2000,
    Score5000,
    Score10000,
    ScoreBonus1Left,
    ScoreBonus1Right,
    ScoreBonus2Left,
    ScoreBonus2Right,
    ScoreBonus3Left,
    ScoreBonus3Right,
    ScoreBonus4Left,
    ScoreBonus4Right,
    ScoreBonus5Left,
    ScoreBonus5Right,
    ScoreBonus6Left,
    ScoreBonus6Right,
    ScoreBonus7Left,
    ScoreBonus7Right,
    BlueLightBackground1,
    BlueLightBackground2,
    BlueLightBackground3,
    BlueLightBackground4,
    TextOnScreenBackground,
    HighVoltageFlashBackground,
    RedFlashlightBackground,
    BlueFlashlightBackground,
    KeypanelBackground,
    RedRotationLightBackground,
    UpArrowBackground,
    GreenPoisonBackground,
    LavaBackground,
}

impl ActorType {
    pub(crate) fn create_actor_interface(
        &self,
        g: &mut ActorData,
        s: &mut LevelSolids,
        t: &mut LevelTiles,
    ) -> Box<dyn ActorInterface> {
        match self {
            ActorType::FireWheelBot => {
                firewheelbot::Specific::create_boxed(g, s, t)
            }
            ActorType::FlameGnomeBot => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::FlyingBot => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::FootBot => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::HelicopterBot => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::RabbitoidBot => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::RedBallJumping => {
                redball_jumping::Specific::create_boxed(g, s, t)
            }
            ActorType::RedBallLying => {
                redball_lying::Specific::create_boxed(g, s, t)
            }
            ActorType::Robot => robot::Specific::create_boxed(g, s, t),
            ActorType::RobotDisappearing => {
                singleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::SnakeBot => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::TankBot => tankbot::Specific::create_boxed(g, s, t),
            ActorType::WallCrawlerBotLeft => {
                wallcrawler::Specific::create_boxed(g, s, t)
            }
            ActorType::WallCrawlerBotRight => {
                wallcrawler::Specific::create_boxed(g, s, t)
            }
            ActorType::DrProton => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::Camera => camera::Specific::create_boxed(g, s, t),
            ActorType::Explosion => {
                singleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::DustCloud => {
                singleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::Steam => {
                singleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::ParticlePink => {
                particle::Specific::create_boxed(g, s, t)
            }
            ActorType::ParticleBlue => {
                particle::Specific::create_boxed(g, s, t)
            }
            ActorType::ParticleWhite => {
                particle::Specific::create_boxed(g, s, t)
            }
            ActorType::ParticleGreen => {
                particle::Specific::create_boxed(g, s, t)
            }
            ActorType::Rocket => rocket::Specific::create_boxed(g, s, t),
            ActorType::Bomb => bomb::Specific::create_boxed(g, s, t),
            ActorType::BombFire => {
                singleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::Water => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::ExitDoor => {
                exitdoor::Specific::create_boxed(g, s, t)
            }
            ActorType::Notebook => {
                notebook::Specific::create_boxed(g, s, t)
            }
            ActorType::SurveillanceScreen => {
                surveillancescreen::Specific::create_boxed(g, s, t)
            }
            ActorType::HostileShotLeft => {
                hostileshot::Specific::create_boxed(g, s, t)
            }
            ActorType::HostileShotRight => {
                hostileshot::Specific::create_boxed(g, s, t)
            }
            ActorType::Soda => item::Specific::create_boxed(g, s, t),
            ActorType::SodaFlying => {
                soda_flying::Specific::create_boxed(g, s, t)
            }
            ActorType::UnstableFloor => {
                unstablefloor::Specific::create_boxed(g, s, t)
            }
            ActorType::ExpandingFloor => {
                expandingfloor::Specific::create_boxed(g, s, t)
            }
            ActorType::ConveyorLeftMovingRightEnd => {
                conveyor::Specific::create_boxed(g, s, t)
            }
            ActorType::ConveyorRightMovingRightEnd => {
                conveyor::Specific::create_boxed(g, s, t)
            }
            ActorType::FanLeft => fan::Specific::create_boxed(g, s, t),
            ActorType::FanRight => fan::Specific::create_boxed(g, s, t),
            ActorType::BrokenWallBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::StoneBackground => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::Teleporter1 => {
                teleporter::Specific::create_boxed(g, s, t)
            }
            ActorType::Teleporter2 => {
                teleporter::Specific::create_boxed(g, s, t)
            }
            ActorType::FenceBackground => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::StoneWindowBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::WindowLeftBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::WindowRightBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::Screen => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::BoxGreyEmpty => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::BoxGreyBoots => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Boots => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyClamps => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Clamps => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyGun => item::Specific::create_boxed(g, s, t),
            ActorType::Gun => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyBomb => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::BoxRedSoda => item::Specific::create_boxed(g, s, t),
            ActorType::BoxRedChicken => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::ChickenSingle => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::ChickenDouble => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::BoxBlueFootball => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Football => item::Specific::create_boxed(g, s, t),
            ActorType::Flag => item::Specific::create_boxed(g, s, t),
            ActorType::BoxBlueJoystick => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Joystick => item::Specific::create_boxed(g, s, t),
            ActorType::BoxBlueDisk => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Disk => item::Specific::create_boxed(g, s, t),
            ActorType::BoxBlueBalloon => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Balloon => balloon::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyGlove => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Glove => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyFullLife => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::FullLife => item::Specific::create_boxed(g, s, t),
            ActorType::BoxBlueFlag => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::BlueFlag => item::Specific::create_boxed(g, s, t),
            ActorType::BoxBlueRadio => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::Radio => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyAccessCard => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::AccessCard => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyLetterD => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::LetterD => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyLetterU => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::LetterU => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyLetterK => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::LetterK => item::Specific::create_boxed(g, s, t),
            ActorType::BoxGreyLetterE => {
                item::Specific::create_boxed(g, s, t)
            }
            ActorType::LetterE => item::Specific::create_boxed(g, s, t),
            ActorType::AccessCardSlot => {
                accesscard_slot::Specific::create_boxed(g, s, t)
            }
            ActorType::GloveSlot => {
                glove_slot::Specific::create_boxed(g, s, t)
            }
            ActorType::KeyRed => key::Specific::create_boxed(g, s, t),
            ActorType::KeyholeRed => {
                keyhole::Specific::create_boxed(g, s, t)
            }
            ActorType::DoorRed => door::Specific::create_boxed(g, s, t),
            ActorType::KeyBlue => key::Specific::create_boxed(g, s, t),
            ActorType::KeyholeBlue => {
                keyhole::Specific::create_boxed(g, s, t)
            }
            ActorType::DoorBlue => door::Specific::create_boxed(g, s, t),
            ActorType::KeyPink => key::Specific::create_boxed(g, s, t),
            ActorType::KeyholePink => {
                keyhole::Specific::create_boxed(g, s, t)
            }
            ActorType::DoorPink => door::Specific::create_boxed(g, s, t),
            ActorType::KeyGreen => key::Specific::create_boxed(g, s, t),
            ActorType::KeyholeGreen => {
                keyhole::Specific::create_boxed(g, s, t)
            }
            ActorType::DoorGreen => door::Specific::create_boxed(g, s, t),
            ActorType::ShootableWall => {
                shootable_wall::Specific::create_boxed(g, s, t)
            }
            ActorType::Lift => elevator::Specific::create_boxed(g, s, t),
            ActorType::Acme => acme::Specific::create_boxed(g, s, t),
            ActorType::FireRight => fire::Specific::create_boxed(g, s, t),
            ActorType::FireLeft => fire::Specific::create_boxed(g, s, t),
            ActorType::Mill => mill::Specific::create_boxed(g, s, t),
            ActorType::Laserbeam => {
                placeholder::Specific::create_boxed(g, s, t)
            }
            ActorType::AccessCardDoor => {
                accesscard_door::Specific::create_boxed(g, s, t)
            }
            ActorType::SpikesUp => spikes::Specific::create_boxed(g, s, t),
            ActorType::SpikesDown => {
                spikes::Specific::create_boxed(g, s, t)
            }
            ActorType::Spike => spikes::Specific::create_boxed(g, s, t),
            ActorType::Score100 => score::Specific::create_boxed(g, s, t),
            ActorType::Score200 => score::Specific::create_boxed(g, s, t),
            ActorType::Score500 => score::Specific::create_boxed(g, s, t),
            ActorType::Score1000 => score::Specific::create_boxed(g, s, t),
            ActorType::Score2000 => score::Specific::create_boxed(g, s, t),
            ActorType::Score5000 => score::Specific::create_boxed(g, s, t),
            ActorType::Score10000 => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus1Left => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus1Right => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus2Left => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus2Right => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus3Left => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus3Right => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus4Left => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus4Right => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus5Left => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus5Right => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus6Left => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus6Right => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus7Left => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::ScoreBonus7Right => {
                score::Specific::create_boxed(g, s, t)
            }
            ActorType::BlueLightBackground1 => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::BlueLightBackground2 => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::BlueLightBackground3 => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::BlueLightBackground4 => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::TextOnScreenBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::HighVoltageFlashBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::RedFlashlightBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::BlueFlashlightBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::KeypanelBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::RedRotationLightBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::UpArrowBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::GreenPoisonBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
            ActorType::LavaBackground => {
                simpleanimation::Specific::create_boxed(g, s, t)
            }
        }
    }
}

#[derive(Debug)]
pub struct ActorData {
    pub actor_type: ActorType,
    pub position: Rect,
    pub is_in_foreground: bool,
    pub hurts_hero: bool,
    pub is_alive: bool,
    pub touches_hero: bool,
    pub is_visible: bool,
    pub acts_while_invisible: bool,
}

impl ActorData {
    pub fn new(actor_type: ActorType) -> Self {
        ActorData {
            actor_type,
            position: Rect::new(0, 0, 0, 0),
            is_in_foreground: true,
            hurts_hero: false,
            is_alive: true,
            touches_hero: false,
            is_visible: false,
            acts_while_invisible: false,
        }
    }
}

pub struct ActorQueueItem {
    pub actor_type: ActorType,
    pub pos: Point,
}

pub trait ActorAdder {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point);

    fn add_particle_firework(&mut self, pos: Point, count: usize) {
        for i in 0..count {
            let actor_type = match i % 4 {
                0 => ActorType::ParticlePink,
                1 => ActorType::ParticleBlue,
                2 => ActorType::ParticleWhite,
                3 => ActorType::ParticleGreen,
                _ => unreachable!(),
            };
            self.add_actor(actor_type, pos);
        }
    }
}

#[derive(Default)]
pub struct ActorQueue {
    pub actors: Vec<ActorQueueItem>,
}

impl ActorAdder for ActorQueue {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point) {
        self.push_back(actor_type, pos);
    }
}

impl ActorQueue {
    pub fn new() -> Self {
        Self { actors: Vec::new() }
    }

    pub fn push_back(&mut self, actor_type: ActorType, pos: Point) {
        self.actors.push(ActorQueueItem { actor_type, pos });
    }

    pub(crate) fn process(&mut self, destination: &mut dyn ActorAdder) {
        for ActorQueueItem { actor_type, pos } in self.actors.drain(..) {
            destination.add_actor(actor_type, pos);
        }
    }
}

pub struct LevelActorAdder<'a> {
    pub solids: &'a mut LevelSolids,
    pub tiles: &'a mut LevelTiles,
    pub actors: &'a mut ActorsList,
}
impl<'a> ActorAdder for LevelActorAdder<'a> {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point) {
        let mut general = ActorData::new(actor_type);
        general.position.reposition(pos);
        let specific = actor_type.create_actor_interface(
            &mut general,
            self.solids,
            self.tiles,
        );
        self.actors.actors.push(Actor { general, specific });
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ActorMessageType {
    OpenDoor,
    Teleport,
    Expand,
}

pub struct ActorMessage {
    pub receivers: ActorType,
    pub message: ActorMessageType,
}

#[derive(Default)]
pub struct ActorMessageQueue {
    pub messages: Vec<ActorMessage>,
}

impl ActorMessageQueue {
    pub fn new() -> Self {
        ActorMessageQueue {
            messages: Vec::new(),
        }
    }

    pub fn push_back(
        &mut self,
        receivers: ActorType,
        message: ActorMessageType,
    ) {
        self.messages.push(ActorMessage { receivers, message });
    }
}

pub(crate) trait ActorCreateInterface: Sized {
    fn create(
        general: &mut ActorData,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self;

    fn create_boxed(
        general: &mut ActorData,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Box<Self> {
        Box::new(Self::create(general, solids, tiles))
    }
}

pub struct ActParameters<'a> {
    pub general: &'a mut ActorData,
    pub solids: &'a mut LevelSolids,
    pub tiles: &'a mut LevelTiles,
    pub hero_data: &'a mut HeroData,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub do_play: &'a mut bool,
}

pub struct ShotParameters<'a> {
    pub general: &'a mut ActorData,
    pub solids: &'a mut LevelSolids,
    pub tiles: &'a mut LevelTiles,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub hero_data: &'a mut HeroData,
}

pub struct RenderParameters<'a> {
    pub general: &'a mut ActorData,
    pub renderer: &'a mut dyn Renderer,
}

pub struct ReceiveMessageParameters<'a> {
    pub general: &'a mut ActorData,
    pub message: ActorMessageType,
    pub hero_data: &'a mut HeroData,
    pub solids: &'a mut LevelSolids,
}

pub struct HeroInteractStartParameters<'a> {
    pub general: &'a mut ActorData,
    pub level_passed: &'a mut bool,
    pub hero_data: &'a mut HeroData,
    pub info_message_queue: &'a mut InfoMessageQueue,
    pub actor_message_queue: &'a mut ActorMessageQueue,
}

pub struct HeroInteractEndParameters<'a> {
    pub general: &'a mut ActorData,
    pub hero_data: &'a mut HeroData,
    pub level_passed: &'a mut bool,
}

pub struct HeroTouchStartParameters<'a> {
    pub general: &'a mut ActorData,
    pub hero_data: &'a mut HeroData,
    pub actor_adder: &'a mut dyn ActorAdder,
}

pub struct HeroTouchEndParameters<'a> {
    pub general: &'a mut ActorData,
    pub hero_data: &'a mut HeroData,
}

pub(crate) trait ActorInterface: std::fmt::Debug {
    fn hero_touch_start(&mut self, _p: HeroTouchStartParameters) {}

    fn hero_touch_end(&mut self, _p: HeroTouchEndParameters) {}

    fn hero_can_interact(&self) -> bool {
        false
    }

    fn hero_interact_start(&mut self, _p: HeroInteractStartParameters) {}

    fn hero_interact_end(&mut self, _p: HeroInteractEndParameters) {}

    fn act(&mut self, p: ActParameters);

    fn render(&mut self, p: RenderParameters) -> Result<()>;

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        false
    }

    fn shot(&mut self, _p: ShotParameters) -> ShotProcessing {
        ShotProcessing::Ignore
    }

    fn receive_message(&mut self, _p: ReceiveMessageParameters) {}
}
