mod accesscard_door;
mod accesscard_slot;
mod acme;
mod balloon;
mod bomb;
mod camera;
mod conveyor;
mod door;
mod electric_arc;
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
    geometry::RectExt,
    hero::Hero,
    infobox::InfoMessageQueue,
    level::{solids::LevelSolids, tiles::LevelTiles, PlayState},
    rendering::Renderer,
    Result,
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
                Some(a) if !a.is_alive() => {
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
        hero: &mut Hero,
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
                hero,
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
        hero: &mut Hero,
        actor_message_queue: &mut ActorMessageQueue,
    ) -> bool {
        for actor in self.actors.iter_mut() {
            if actor.can_get_shot()
                && shot_position.touches(actor.position())
                && actor.shot(
                    solids,
                    tiles,
                    actor_adder,
                    hero,
                    actor_message_queue,
                ) == ShotProcessing::Absorb
            {
                return true;
            }
        }
        false
    }

    pub fn start_interaction(
        &mut self,
        play_state: &mut PlayState,
        hero: &mut Hero,
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        let new_interactor = self.actors.iter().position(|actor| {
            actor.hero_can_interact(hero)
                && hero.position.geometry.touches(actor.position())
        });

        if let Some(i) = new_interactor {
            self.end_interaction(play_state, hero);

            let actor = self.actors.get_mut(i).unwrap();
            let p = HeroInteractStartParameters {
                general: &mut actor.general,
                play_state,
                hero,
                info_message_queue,
                actor_message_queue,
            };
            self.interaction_target = Some(i);
            actor.specific.hero_interact_start(p);
        }
    }

    pub fn end_interaction(
        &mut self,
        play_state: &mut PlayState,
        hero: &mut Hero,
    ) {
        if let Some(i) = self.interaction_target.take() {
            if let Some(actor) = self.actors.get_mut(i) {
                let p = HeroInteractEndParameters {
                    general: &mut actor.general,
                    play_state,
                    hero,
                };
                actor.specific.hero_interact_end(p);
            }
        }
    }

    pub fn act(
        &mut self,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        hero: &mut Hero,
        actor_queue: &mut ActorQueue,
        play_state: &mut PlayState,
        visible_rect: Rect,
    ) {
        let mut actors_hurting_hero = 0usize;
        for actor in self.actors.iter_mut() {
            if actor.acts_while_invisible()
                || actor.position().has_intersection(visible_rect)
            {
                actor.act(solids, tiles, hero, actor_queue, play_state);
                if actor.is_alive() && actor.hurts_hero(hero) {
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
        hero.gets_hurt = actors_hurting_hero > 0;
    }

    pub fn render_background_actors(
        &mut self,
        renderer: &mut dyn Renderer,
        draw_collision_bounds: bool,
        visible_rect: Rect,
    ) -> Result<()> {
        Ok(self
            .actors
            .iter_mut()
            .filter(|actor| {
                !actor.is_in_foreground()
                    && actor.position().has_intersection(visible_rect)
            })
            .map(|actor| actor.render(renderer, draw_collision_bounds))
            .collect::<Result<_>>()?)
    }

    pub fn render_foreground_actors(
        &mut self,
        renderer: &mut dyn Renderer,
        draw_collision_bounds: bool,
        visible_rect: Rect,
    ) -> Result<()> {
        Ok(self
            .actors
            .iter_mut()
            .filter(|actor| {
                actor.is_in_foreground()
                    && actor.position().has_intersection(visible_rect)
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
        hero: &mut Hero,
        actor_adder: &mut dyn ActorAdder,
        play_state: &mut PlayState,
    ) -> bool {
        let p = ActParameters {
            general: &mut self.general,
            solids,
            tiles,
            hero,
            actor_adder,
            play_state,
        };
        self.specific.act(p);
        self.is_alive()
    }

    fn hero_can_interact(&self, hero: &Hero) -> bool {
        self.specific.hero_can_interact(hero)
    }

    pub fn can_get_shot(&self) -> bool {
        self.specific.can_get_shot(&self.general)
    }

    pub fn shot(
        &mut self,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero: &mut Hero,
        actor_message_queue: &mut ActorMessageQueue,
    ) -> ShotProcessing {
        let p = ShotParameters {
            general: &mut self.general,
            solids,
            tiles,
            actor_adder,
            hero,
            actor_message_queue,
        };
        self.specific.shot(p)
    }

    pub fn is_alive(&self) -> bool {
        self.specific.is_alive()
    }

    pub fn position(&self) -> Rect {
        self.specific.position()
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
            renderer.draw_rect(self.position(), color)?;
        }
        Ok(())
    }

    pub fn is_in_foreground(&self) -> bool {
        self.specific.is_in_foreground()
    }

    pub fn hurts_hero(&self, hero: &Hero) -> bool {
        self.specific.hurts_hero(hero)
    }

    pub fn acts_while_invisible(&self) -> bool {
        self.specific.acts_while_invisible()
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
    ElectricArc,
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
        p: Point,
        s: &mut LevelSolids,
        t: &mut LevelTiles,
    ) -> Box<dyn ActorInterface> {
        match self {
            ActorType::FireWheelBot => {
                firewheelbot::Specific::create_boxed(g, p, s, t)
            }
            ActorType::FlameGnomeBot => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::FlyingBot => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::FootBot => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::HelicopterBot => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::RabbitoidBot => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::RedBallJumping => {
                redball_jumping::Specific::create_boxed(g, p, s, t)
            }
            ActorType::RedBallLying => {
                redball_lying::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Robot => robot::Specific::create_boxed(g, p, s, t),
            ActorType::RobotDisappearing => {
                singleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::SnakeBot => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::TankBot => {
                tankbot::Specific::create_boxed(g, p, s, t)
            }
            ActorType::WallCrawlerBotLeft => {
                wallcrawler::Specific::create_boxed(g, p, s, t)
            }
            ActorType::WallCrawlerBotRight => {
                wallcrawler::Specific::create_boxed(g, p, s, t)
            }
            ActorType::DrProton => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Camera => {
                camera::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Explosion => {
                singleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::DustCloud => {
                singleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Steam => {
                singleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ParticlePink => {
                particle::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ParticleBlue => {
                particle::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ParticleWhite => {
                particle::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ParticleGreen => {
                particle::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Rocket => {
                rocket::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Bomb => bomb::Specific::create_boxed(g, p, s, t),
            ActorType::BombFire => {
                singleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Water => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ExitDoor => {
                exitdoor::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Notebook => {
                notebook::Specific::create_boxed(g, p, s, t)
            }
            ActorType::SurveillanceScreen => {
                surveillancescreen::Specific::create_boxed(g, p, s, t)
            }
            ActorType::HostileShotLeft => {
                hostileshot::Specific::create_boxed(g, p, s, t)
            }
            ActorType::HostileShotRight => {
                hostileshot::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Soda => item::Specific::create_boxed(g, p, s, t),
            ActorType::SodaFlying => {
                soda_flying::Specific::create_boxed(g, p, s, t)
            }
            ActorType::UnstableFloor => {
                unstablefloor::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ExpandingFloor => {
                expandingfloor::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ConveyorLeftMovingRightEnd => {
                conveyor::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ConveyorRightMovingRightEnd => {
                conveyor::Specific::create_boxed(g, p, s, t)
            }
            ActorType::FanLeft => fan::Specific::create_boxed(g, p, s, t),
            ActorType::FanRight => fan::Specific::create_boxed(g, p, s, t),
            ActorType::BrokenWallBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::StoneBackground => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Teleporter1 => {
                teleporter::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Teleporter2 => {
                teleporter::Specific::create_boxed(g, p, s, t)
            }
            ActorType::FenceBackground => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::StoneWindowBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::WindowLeftBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::WindowRightBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Screen => {
                placeholder::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxGreyEmpty => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxGreyBoots => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Boots => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyClamps => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Clamps => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyGun => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Gun => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyBomb => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxRedSoda => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxRedChicken => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ChickenSingle => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ChickenDouble => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxBlueFootball => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Football => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Flag => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxBlueJoystick => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Joystick => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxBlueDisk => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Disk => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxBlueBalloon => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Balloon => {
                balloon::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxGreyGlove => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Glove => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyFullLife => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::FullLife => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxBlueFlag => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BlueFlag => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxBlueRadio => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Radio => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyAccessCard => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::AccessCard => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BoxGreyLetterD => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::LetterD => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyLetterU => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::LetterU => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyLetterK => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::LetterK => item::Specific::create_boxed(g, p, s, t),
            ActorType::BoxGreyLetterE => {
                item::Specific::create_boxed(g, p, s, t)
            }
            ActorType::LetterE => item::Specific::create_boxed(g, p, s, t),
            ActorType::AccessCardSlot => {
                accesscard_slot::Specific::create_boxed(g, p, s, t)
            }
            ActorType::GloveSlot => {
                glove_slot::Specific::create_boxed(g, p, s, t)
            }
            ActorType::KeyRed => key::Specific::create_boxed(g, p, s, t),
            ActorType::KeyholeRed => {
                keyhole::Specific::create_boxed(g, p, s, t)
            }
            ActorType::DoorRed => door::Specific::create_boxed(g, p, s, t),
            ActorType::KeyBlue => key::Specific::create_boxed(g, p, s, t),
            ActorType::KeyholeBlue => {
                keyhole::Specific::create_boxed(g, p, s, t)
            }
            ActorType::DoorBlue => {
                door::Specific::create_boxed(g, p, s, t)
            }
            ActorType::KeyPink => key::Specific::create_boxed(g, p, s, t),
            ActorType::KeyholePink => {
                keyhole::Specific::create_boxed(g, p, s, t)
            }
            ActorType::DoorPink => {
                door::Specific::create_boxed(g, p, s, t)
            }
            ActorType::KeyGreen => key::Specific::create_boxed(g, p, s, t),
            ActorType::KeyholeGreen => {
                keyhole::Specific::create_boxed(g, p, s, t)
            }
            ActorType::DoorGreen => {
                door::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ShootableWall => {
                shootable_wall::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Lift => {
                elevator::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Acme => acme::Specific::create_boxed(g, p, s, t),
            ActorType::FireRight => {
                fire::Specific::create_boxed(g, p, s, t)
            }
            ActorType::FireLeft => {
                fire::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Mill => mill::Specific::create_boxed(g, p, s, t),
            ActorType::ElectricArc => {
                electric_arc::Specific::create_boxed(g, p, s, t)
            }
            ActorType::AccessCardDoor => {
                accesscard_door::Specific::create_boxed(g, p, s, t)
            }
            ActorType::SpikesUp => {
                spikes::Specific::create_boxed(g, p, s, t)
            }
            ActorType::SpikesDown => {
                spikes::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Spike => spikes::Specific::create_boxed(g, p, s, t),
            ActorType::Score100 => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Score200 => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Score500 => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Score1000 => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Score2000 => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Score5000 => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::Score10000 => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus1Left => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus1Right => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus2Left => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus2Right => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus3Left => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus3Right => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus4Left => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus4Right => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus5Left => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus5Right => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus6Left => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus6Right => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus7Left => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::ScoreBonus7Right => {
                score::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BlueLightBackground1 => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BlueLightBackground2 => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BlueLightBackground3 => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BlueLightBackground4 => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::TextOnScreenBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::HighVoltageFlashBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::RedFlashlightBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::BlueFlashlightBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::KeypanelBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::RedRotationLightBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::UpArrowBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::GreenPoisonBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
            ActorType::LavaBackground => {
                simpleanimation::Specific::create_boxed(g, p, s, t)
            }
        }
    }
}

#[derive(Debug)]
pub struct ActorData {
    pub actor_type: ActorType,
}

impl ActorData {
    pub fn new(actor_type: ActorType) -> Self {
        ActorData { actor_type }
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
        let specific = actor_type.create_actor_interface(
            &mut general,
            pos,
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
    Remove,
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
        position: Point,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self;

    fn create_boxed(
        general: &mut ActorData,
        position: Point,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Box<Self> {
        Box::new(Self::create(general, position, solids, tiles))
    }
}

pub struct ActParameters<'a> {
    pub general: &'a mut ActorData,
    pub solids: &'a mut LevelSolids,
    pub tiles: &'a mut LevelTiles,
    pub hero: &'a mut Hero,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub play_state: &'a mut PlayState,
}

pub struct ShotParameters<'a> {
    pub general: &'a mut ActorData,
    pub solids: &'a mut LevelSolids,
    pub tiles: &'a mut LevelTiles,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub hero: &'a mut Hero,
    pub actor_message_queue: &'a mut ActorMessageQueue,
}

pub struct RenderParameters<'a> {
    pub general: &'a mut ActorData,
    pub renderer: &'a mut dyn Renderer,
}

pub struct ReceiveMessageParameters<'a> {
    pub general: &'a mut ActorData,
    pub message: ActorMessageType,
    pub hero: &'a mut Hero,
    pub solids: &'a mut LevelSolids,
}

pub struct HeroInteractStartParameters<'a> {
    pub general: &'a mut ActorData,
    pub play_state: &'a mut PlayState,
    pub hero: &'a mut Hero,
    pub info_message_queue: &'a mut InfoMessageQueue,
    pub actor_message_queue: &'a mut ActorMessageQueue,
}

pub struct HeroInteractEndParameters<'a> {
    pub general: &'a mut ActorData,
    pub hero: &'a mut Hero,
    pub play_state: &'a mut PlayState,
}

pub(crate) trait ActorInterface: std::fmt::Debug {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
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

    fn position(&self) -> Rect;

    fn is_in_foreground(&self) -> bool;

    fn hurts_hero(&self, _hero: &Hero) -> bool {
        false
    }

    fn is_alive(&self) -> bool {
        true
    }

    fn acts_while_invisible(&self) -> bool {
        false
    }
}
