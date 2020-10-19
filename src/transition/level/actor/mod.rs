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

use super::super::geometry::Geometry;
use super::super::hero::HeroData;
use super::super::infobox::InfoMessageQueue;
use super::super::level::LevelData;
use super::super::tilecache::TileCache;

pub type ActorsList = Vec<Actor>;

#[repr(C)]
#[derive(Debug)]
pub struct Actor {
    general: ActorData,
    specific: Box<dyn ActorInterface>,
}

impl Actor {
    fn act(
        &mut self,
        level_data: &mut LevelData,
        hero_data: &mut HeroData,
        actor_queue: &mut ActorQueue,
    ) -> bool {
        self.check_hero_touch(hero_data, actor_queue);

        self.specific.act(
            &mut self.general,
            level_data,
            actor_queue,
            hero_data,
        );
        self.general.is_alive
    }

    fn check_hero_touch(
        &mut self,
        hero_data: &mut HeroData,
        actor_queue: &mut ActorQueue,
    ) {
        let touching_hero =
            self.general.position.overlaps(hero_data.position.geometry);

        if touching_hero {
            if !self.general.touches_hero {
                self.general.touches_hero = true;

                self.specific.hero_touch_start(
                    &mut self.general,
                    actor_queue,
                    hero_data,
                );
            }
        } else {
            if self.general.touches_hero {
                self.general.touches_hero = false;
                self.specific.hero_touch_end(&mut self.general, hero_data);
            }
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
}

#[repr(C)]
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
    fn create_actor_interface(
        &self,
        g: &mut ActorData,
        l: &mut LevelData,
    ) -> Box<dyn ActorInterface> {
        match self {
            ActorType::FireWheelBot => {
                firewheelbot::Specific::create_boxed(g, l)
            }
            ActorType::FlameGnomeBot => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::FlyingBot => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::FootBot => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::HelicopterBot => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::RabbitoidBot => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::RedBallJumping => {
                redball_jumping::Specific::create_boxed(g, l)
            }
            ActorType::RedBallLying => {
                redball_lying::Specific::create_boxed(g, l)
            }
            ActorType::Robot => robot::Specific::create_boxed(g, l),
            ActorType::RobotDisappearing => {
                singleanimation::Specific::create_boxed(g, l)
            }
            ActorType::SnakeBot => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::TankBot => tankbot::Specific::create_boxed(g, l),
            ActorType::WallCrawlerBotLeft => {
                wallcrawler::Specific::create_boxed(g, l)
            }
            ActorType::WallCrawlerBotRight => {
                wallcrawler::Specific::create_boxed(g, l)
            }
            ActorType::DrProton => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::Camera => camera::Specific::create_boxed(g, l),
            ActorType::Explosion => {
                singleanimation::Specific::create_boxed(g, l)
            }
            ActorType::DustCloud => {
                singleanimation::Specific::create_boxed(g, l)
            }
            ActorType::Steam => {
                singleanimation::Specific::create_boxed(g, l)
            }
            ActorType::ParticlePink => {
                particle::Specific::create_boxed(g, l)
            }
            ActorType::ParticleBlue => {
                particle::Specific::create_boxed(g, l)
            }
            ActorType::ParticleWhite => {
                particle::Specific::create_boxed(g, l)
            }
            ActorType::ParticleGreen => {
                particle::Specific::create_boxed(g, l)
            }
            ActorType::Rocket => rocket::Specific::create_boxed(g, l),
            ActorType::Bomb => bomb::Specific::create_boxed(g, l),
            ActorType::BombFire => {
                singleanimation::Specific::create_boxed(g, l)
            }
            ActorType::Water => placeholder::Specific::create_boxed(g, l),
            ActorType::ExitDoor => exitdoor::Specific::create_boxed(g, l),
            ActorType::Notebook => notebook::Specific::create_boxed(g, l),
            ActorType::SurveillanceScreen => {
                surveillancescreen::Specific::create_boxed(g, l)
            }
            ActorType::HostileShotLeft => {
                hostileshot::Specific::create_boxed(g, l)
            }
            ActorType::HostileShotRight => {
                hostileshot::Specific::create_boxed(g, l)
            }
            ActorType::Soda => item::Specific::create_boxed(g, l),
            ActorType::SodaFlying => {
                soda_flying::Specific::create_boxed(g, l)
            }
            ActorType::UnstableFloor => {
                unstablefloor::Specific::create_boxed(g, l)
            }
            ActorType::ExpandingFloor => {
                expandingfloor::Specific::create_boxed(g, l)
            }
            ActorType::ConveyorLeftMovingRightEnd => {
                conveyor::Specific::create_boxed(g, l)
            }
            ActorType::ConveyorRightMovingRightEnd => {
                conveyor::Specific::create_boxed(g, l)
            }
            ActorType::FanLeft => fan::Specific::create_boxed(g, l),
            ActorType::FanRight => fan::Specific::create_boxed(g, l),
            ActorType::BrokenWallBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::StoneBackground => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::Teleporter1 => {
                teleporter::Specific::create_boxed(g, l)
            }
            ActorType::Teleporter2 => {
                teleporter::Specific::create_boxed(g, l)
            }
            ActorType::FenceBackground => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::StoneWindowBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::WindowLeftBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::WindowRightBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::Screen => placeholder::Specific::create_boxed(g, l),
            ActorType::BoxGreyEmpty => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyBoots => item::Specific::create_boxed(g, l),
            ActorType::Boots => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyClamps => item::Specific::create_boxed(g, l),
            ActorType::Clamps => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyGun => item::Specific::create_boxed(g, l),
            ActorType::Gun => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyBomb => item::Specific::create_boxed(g, l),
            ActorType::BoxRedSoda => item::Specific::create_boxed(g, l),
            ActorType::BoxRedChicken => item::Specific::create_boxed(g, l),
            ActorType::ChickenSingle => item::Specific::create_boxed(g, l),
            ActorType::ChickenDouble => item::Specific::create_boxed(g, l),
            ActorType::BoxBlueFootball => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::Football => item::Specific::create_boxed(g, l),
            ActorType::Flag => item::Specific::create_boxed(g, l),
            ActorType::BoxBlueJoystick => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::Joystick => item::Specific::create_boxed(g, l),
            ActorType::BoxBlueDisk => item::Specific::create_boxed(g, l),
            ActorType::Disk => item::Specific::create_boxed(g, l),
            ActorType::BoxBlueBalloon => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::Balloon => balloon::Specific::create_boxed(g, l),
            ActorType::BoxGreyGlove => item::Specific::create_boxed(g, l),
            ActorType::Glove => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyFullLife => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::FullLife => item::Specific::create_boxed(g, l),
            ActorType::BoxBlueFlag => item::Specific::create_boxed(g, l),
            ActorType::BlueFlag => item::Specific::create_boxed(g, l),
            ActorType::BoxBlueRadio => item::Specific::create_boxed(g, l),
            ActorType::Radio => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyAccessCard => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::AccessCard => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyLetterD => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::LetterD => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyLetterU => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::LetterU => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyLetterK => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::LetterK => item::Specific::create_boxed(g, l),
            ActorType::BoxGreyLetterE => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::LetterE => item::Specific::create_boxed(g, l),
            ActorType::AccessCardSlot => {
                item::Specific::create_boxed(g, l)
            }
            ActorType::GloveSlot => {
                glove_slot::Specific::create_boxed(g, l)
            }
            ActorType::KeyRed => key::Specific::create_boxed(g, l),
            ActorType::KeyholeRed => keyhole::Specific::create_boxed(g, l),
            ActorType::DoorRed => door::Specific::create_boxed(g, l),
            ActorType::KeyBlue => key::Specific::create_boxed(g, l),
            ActorType::KeyholeBlue => {
                keyhole::Specific::create_boxed(g, l)
            }
            ActorType::DoorBlue => door::Specific::create_boxed(g, l),
            ActorType::KeyPink => key::Specific::create_boxed(g, l),
            ActorType::KeyholePink => {
                keyhole::Specific::create_boxed(g, l)
            }
            ActorType::DoorPink => door::Specific::create_boxed(g, l),
            ActorType::KeyGreen => key::Specific::create_boxed(g, l),
            ActorType::KeyholeGreen => {
                keyhole::Specific::create_boxed(g, l)
            }
            ActorType::DoorGreen => door::Specific::create_boxed(g, l),
            ActorType::ShootableWall => {
                shootable_wall::Specific::create_boxed(g, l)
            }
            ActorType::Lift => elevator::Specific::create_boxed(g, l),
            ActorType::Acme => acme::Specific::create_boxed(g, l),
            ActorType::FireRight => fire::Specific::create_boxed(g, l),
            ActorType::FireLeft => fire::Specific::create_boxed(g, l),
            ActorType::Mill => mill::Specific::create_boxed(g, l),
            ActorType::Laserbeam => {
                placeholder::Specific::create_boxed(g, l)
            }
            ActorType::AccessCardDoor => {
                accesscard_door::Specific::create_boxed(g, l)
            }
            ActorType::SpikesUp => spikes::Specific::create_boxed(g, l),
            ActorType::SpikesDown => spikes::Specific::create_boxed(g, l),
            ActorType::Spike => spikes::Specific::create_boxed(g, l),
            ActorType::Score100 => score::Specific::create_boxed(g, l),
            ActorType::Score200 => score::Specific::create_boxed(g, l),
            ActorType::Score500 => score::Specific::create_boxed(g, l),
            ActorType::Score1000 => score::Specific::create_boxed(g, l),
            ActorType::Score2000 => score::Specific::create_boxed(g, l),
            ActorType::Score5000 => score::Specific::create_boxed(g, l),
            ActorType::Score10000 => score::Specific::create_boxed(g, l),
            ActorType::ScoreBonus1Left => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus1Right => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus2Left => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus2Right => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus3Left => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus3Right => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus4Left => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus4Right => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus5Left => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus5Right => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus6Left => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus6Right => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus7Left => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::ScoreBonus7Right => {
                score::Specific::create_boxed(g, l)
            }
            ActorType::BlueLightBackground1 => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::BlueLightBackground2 => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::BlueLightBackground3 => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::BlueLightBackground4 => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::TextOnScreenBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::HighVoltageFlashBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::RedFlashlightBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::BlueFlashlightBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::KeypanelBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::RedRotationLightBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::UpArrowBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::GreenPoisonBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
            ActorType::LavaBackground => {
                simpleanimation::Specific::create_boxed(g, l)
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ActorData {
    pub actor_type: ActorType,
    pub position: Geometry,
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
            position: Geometry::default(),
            is_in_foreground: true,
            hurts_hero: false,
            is_alive: true,
            touches_hero: false,
            is_visible: false,
            acts_while_invisible: false,
        }
    }
}

#[repr(C)]
pub struct ActorQueueItem {
    pub actor_type: ActorType,
    pub x: u16,
    pub y: u16,
}

#[derive(Default)]
pub struct ActorQueue {
    pub actors: Vec<ActorQueueItem>,
}

impl ActorQueue {
    pub fn push_back(&mut self, actor_type: ActorType, x: u16, y: u16) {
        self.actors.push(ActorQueueItem { actor_type, x, y });
    }

    pub fn push_particle_firework(
        &mut self,
        x: u16,
        y: u16,
        count: usize,
    ) {
        for i in 0..count {
            let actor_type = match i % 4 {
                0 => ActorType::ParticlePink,
                1 => ActorType::ParticleBlue,
                2 => ActorType::ParticleWhite,
                3 => ActorType::ParticleGreen,
                _ => unreachable!(),
            };
            self.push_back(actor_type, x, y);
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ActorMessageType {
    OpenDoor,
    Teleport,
    Expand,
}

#[repr(C)]
pub struct ActorMessage {
    pub receivers: ActorType,
    pub message: ActorMessageType,
}

#[derive(Default)]
pub struct ActorMessageQueue {
    pub messages: Vec<ActorMessage>,
}

impl ActorMessageQueue {
    pub fn push_back(
        &mut self,
        receivers: ActorType,
        message: ActorMessageType,
    ) {
        self.messages.push(ActorMessage { receivers, message });
    }
}

pub(crate) trait ActorCreateInterface: Sized {
    fn create(general: &mut ActorData, level_data: &mut LevelData)
        -> Self;

    fn create_boxed(
        general: &mut ActorData,
        level_data: &mut LevelData,
    ) -> Box<Self> {
        Box::new(Self::create(general, level_data))
    }
}

pub(crate) trait ActorInterface: std::fmt::Debug {
    fn hero_touch_start(
        &mut self,
        _general: &mut ActorData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
    }

    fn hero_touch_end(
        &mut self,
        _general: &mut ActorData,
        _hero_data: &mut HeroData,
    ) {
    }

    fn hero_can_interact(&self) -> bool {
        false
    }

    fn hero_interact_start(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _hero_data: &mut HeroData,
        _info_message_queue: &mut InfoMessageQueue,
        _actor_message_queue: &mut ActorMessageQueue,
    ) {
    }

    fn hero_interact_end(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _hero_data: &mut HeroData,
    ) {
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    );

    fn blit(
        &mut self,
        general: &mut ActorData,
        hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut transdl::video::Surface,
    );

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        false
    }

    fn shot(
        &mut self,
        _general: &mut ActorData,
        _level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
    }

    fn receive_message(
        &mut self,
        _general: &mut ActorData,
        _message: ActorMessageType,
        _hero_data: &mut HeroData,
        _level_data: &mut LevelData,
    ) {
    }
}

pub mod ffi {
    pub type FnLevelActor = super::Actor;
    type FnLevelActorData = super::ActorData;
    pub type FnLevelActorType = super::ActorType;
    type FnLevelActorQueue = super::ActorQueue;
    type FnLevelActorQueueItem = super::ActorQueueItem;
    type FnLevelActorMessage = super::ActorMessage;
    pub type FnLevelActorMessageType = super::ActorMessageType;
    type FnLevelActorMessageQueue = super::ActorMessageQueue;
    pub type FnLevelActorsList = super::ActorsList;

    use super::super::super::geometry::ffi::FnGeometry;
    use super::super::super::hero::ffi::FnHeroData;
    use super::super::super::infobox::ffi::FnInfoMessageQueue;
    use super::super::super::level::ffi::FnLevelData;
    use super::super::super::tilecache::ffi::FnTileCache;
    use transdl::ll::SDL_Surface;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_create(
        actor_type: FnLevelActorType,
        level_data: &mut FnLevelData,
        x: i16,
        y: i16,
    ) -> *mut FnLevelActor {
        let mut general = FnLevelActorData::new(actor_type);
        general.position.x = x;
        general.position.y = y;
        let specific =
            actor_type.create_actor_interface(&mut general, level_data);
        Box::into_raw(Box::new(FnLevelActor { general, specific }))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_free(ptr: *mut FnLevelActor) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_hero_can_interact(
        actor: *const FnLevelActor,
        hero_data: *const FnHeroData,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        assert!(!hero_data.is_null());
        let hero_data = unsafe { &(*hero_data) };

        actor.hero_can_interact(hero_data)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_type(
        actor: *const FnLevelActor,
    ) -> FnLevelActorType {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.general.actor_type
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_can_get_shot(
        actor: *const FnLevelActor,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.specific.can_get_shot(&actor.general)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_in_foreground(
        actor: *const FnLevelActor,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.general.is_in_foreground
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_acts_while_invisible(
        actor: *const FnLevelActor,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.general.acts_while_invisible
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_is_visible(
        actor: *const FnLevelActor,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.general.is_visible
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_is_alive(
        actor: *const FnLevelActor,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.general.is_alive
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_hurts_hero(
        actor: *const FnLevelActor,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.general.hurts_hero
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_set_visible(
        actor: *mut FnLevelActor,
        visible: bool,
    ) {
        assert!(!actor.is_null());
        let actor = unsafe { &mut (*actor) };

        actor.general.is_visible = visible
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_get_position(
        actor: *const FnLevelActor,
    ) -> FnGeometry {
        assert!(!actor.is_null());
        let actor = unsafe { &(*actor) };

        actor.general.position
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_act(
        actor: *mut FnLevelActor,
        level_data: *mut FnLevelData,
        hero_data: *mut FnHeroData,
        actor_queue: *mut FnLevelActorQueue,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &mut (*actor) };

        assert!(!level_data.is_null());
        let level_data = unsafe { &mut (*level_data) };

        assert!(!hero_data.is_null());
        let hero_data = unsafe { &mut (*hero_data) };

        assert!(!actor_queue.is_null());
        let actor_queue = unsafe { &mut (*actor_queue) };

        actor.act(level_data, hero_data, actor_queue)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_hero_interact_start(
        actor: *mut FnLevelActor,
        level_data: *mut FnLevelData,
        hero_data: *mut FnHeroData,
        info_message_queue: *mut FnInfoMessageQueue,
        actor_message_queue: *mut FnLevelActorMessageQueue,
    ) {
        assert!(!actor.is_null());
        let actor = unsafe { &mut (*actor) };

        assert!(!level_data.is_null());
        let level_data = unsafe { &mut (*level_data) };

        assert!(!hero_data.is_null());
        let hero_data = unsafe { &mut (*hero_data) };

        assert!(!info_message_queue.is_null());
        let info_message_queue = unsafe { &mut (*info_message_queue) };

        assert!(!actor_message_queue.is_null());
        let actor_message_queue = unsafe { &mut (*actor_message_queue) };

        actor.specific.hero_interact_start(
            &mut actor.general,
            level_data,
            hero_data,
            info_message_queue,
            actor_message_queue,
        )
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_hero_interact_end(
        actor: *mut FnLevelActor,
        level_data: *mut FnLevelData,
        hero_data: *mut FnHeroData,
    ) {
        assert!(!actor.is_null());
        let actor = unsafe { &mut (*actor) };

        assert!(!level_data.is_null());
        let level_data = unsafe { &mut (*level_data) };

        assert!(!hero_data.is_null());
        let hero_data = unsafe { &mut (*hero_data) };

        actor.specific.hero_interact_end(
            &mut actor.general,
            level_data,
            hero_data,
        )
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_shot(
        actor: *mut FnLevelActor,
        level_data: *mut FnLevelData,
        hero_data: *mut FnHeroData,
        actor_queue: *mut FnLevelActorQueue,
    ) -> bool {
        assert!(!actor.is_null());
        let actor = unsafe { &mut (*actor) };

        assert!(!level_data.is_null());
        let level_data = unsafe { &mut (*level_data) };

        assert!(!hero_data.is_null());
        let hero_data = unsafe { &mut (*hero_data) };

        assert!(!actor_queue.is_null());
        let actor_queue = unsafe { &mut (*actor_queue) };

        if actor.specific.can_get_shot(&actor.general) {
            actor.specific.shot(
                &mut actor.general,
                level_data,
                actor_queue,
                hero_data,
            );
            true
        } else {
            false
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_blit(
        actor: *mut FnLevelActor,
        hero_data: *mut FnHeroData,
        tilecache: *const FnTileCache,
        target: *mut SDL_Surface,
        draw_collision_bounds: bool,
    ) {
        assert!(!actor.is_null());
        let actor = unsafe { &mut (*actor) };

        assert!(!hero_data.is_null());
        let hero_data = unsafe { &mut (*hero_data) };

        assert!(!tilecache.is_null());
        let tilecache = unsafe { &(*tilecache) };

        assert!(!target.is_null());
        let target = unsafe { &mut (*target) };

        let mut target = transdl::video::Surface { raw: target };

        actor.specific.blit(
            &mut actor.general,
            hero_data,
            tilecache,
            &mut target,
        );

        if draw_collision_bounds {
            let color = crate::collision_bounds_color(&target.format());
            actor.general.position.draw_outline(&mut target, color);
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_receive_message(
        actor: *mut FnLevelActor,
        message: FnLevelActorMessageType,
        hero_data: *mut FnHeroData,
        level_data: *mut FnLevelData,
    ) {
        assert!(!actor.is_null());
        let actor = unsafe { &mut (*actor) };

        assert!(!hero_data.is_null());
        let hero_data = unsafe { &mut (*hero_data) };

        assert!(!level_data.is_null());
        let level_data = unsafe { &mut (*level_data) };

        actor.specific.receive_message(
            &mut actor.general,
            message,
            hero_data,
            level_data,
        )
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_queue_create(
    ) -> *mut FnLevelActorQueue {
        Box::into_raw(Box::new(FnLevelActorQueue::default()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_queue_free(
        ptr: *mut FnLevelActorQueue,
    ) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_queue_push_back(
        ptr: *mut FnLevelActorQueue,
        actor_type: FnLevelActorType,
        x: u16,
        y: u16,
    ) {
        assert!(!ptr.is_null());
        let queue = unsafe { &mut (*ptr) };
        queue
            .actors
            .push(super::ActorQueueItem { actor_type, x, y })
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_queue_has_items(
        ptr: *const FnLevelActorQueue,
    ) -> bool {
        let queue = unsafe { &(*ptr) };
        !queue.actors.is_empty()
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_queue_pop_front(
        ptr: *mut FnLevelActorQueue,
    ) -> FnLevelActorQueueItem {
        assert!(!ptr.is_null());
        let queue = unsafe { &mut (*ptr) };
        queue.actors.remove(0)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_message_queue_create(
    ) -> *mut FnLevelActorMessageQueue {
        Box::into_raw(Box::new(FnLevelActorMessageQueue::default()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_message_queue_free(
        ptr: *mut FnLevelActorMessageQueue,
    ) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_message_queue_has_items(
        ptr: *const FnLevelActorMessageQueue,
    ) -> bool {
        let queue = unsafe { &(*ptr) };
        !queue.messages.is_empty()
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_message_queue_pop_front(
        ptr: *mut FnLevelActorMessageQueue,
    ) -> FnLevelActorMessage {
        assert!(!ptr.is_null());
        let queue = unsafe { &mut (*ptr) };
        queue.messages.remove(0)
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actors_list_count(
        ptr: *const FnLevelActorsList,
    ) -> usize {
        assert!(!ptr.is_null());
        let d: &FnLevelActorsList = unsafe { &(*ptr) };
        d.len()
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actors_list_get(
        ptr: *mut FnLevelActorsList,
        index: usize,
    ) -> *mut FnLevelActor {
        assert!(!ptr.is_null());
        let d: &mut FnLevelActorsList = unsafe { &mut (*ptr) };
        match d.get_mut(index) {
            Some(actor) => actor as *mut FnLevelActor,
            None => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actors_list_remove_dead(
        ptr: *mut FnLevelActorsList,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelActorsList = unsafe { &mut (*ptr) };
        d.retain(|a| a.general.is_alive);
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actors_list_add_actor(
        ptr: *mut FnLevelActorsList,
        level_data: *mut FnLevelData,
        actor_type: FnLevelActorType,
        x: i16,
        y: i16,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelActorsList = unsafe { &mut (*ptr) };

        assert!(!level_data.is_null());
        let level_data: &mut FnLevelData = unsafe { &mut (*level_data) };

        let mut general = super::ActorData::new(actor_type);
        general.position.x = x;
        general.position.y = y;
        let specific =
            actor_type.create_actor_interface(&mut general, level_data);
        d.push(super::Actor { general, specific });
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actors_list_send_message(
        ptr: *mut FnLevelActorsList,
        receivers: FnLevelActorType,
        message: FnLevelActorMessageType,
        hero_data: *mut FnHeroData,
        level_data: *mut FnLevelData,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnLevelActorsList = unsafe { &mut (*ptr) };

        assert!(!hero_data.is_null());
        let hero_data: &mut FnHeroData = unsafe { &mut (*hero_data) };

        assert!(!level_data.is_null());
        let level_data: &mut FnLevelData = unsafe { &mut (*level_data) };

        for actor in
            d.iter_mut().filter(|a| a.general.actor_type == receivers)
        {
            actor.specific.receive_message(
                &mut actor.general,
                message,
                hero_data,
                level_data,
            );
        }
    }
}
