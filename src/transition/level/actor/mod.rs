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
#[derive(Debug, PartialEq, Eq)]
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

pub(crate) trait ActorInterface {
    fn create(general: &mut ActorData, level_data: &mut LevelData)
        -> Self;

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
    type FnLevelActorData = super::ActorData;
    type FnLevelActorType = super::ActorType;
    type FnLevelActorQueue = super::ActorQueue;
    type FnLevelActorQueueItem = super::ActorQueueItem;
    type FnLevelActorMessage = super::ActorMessage;
    type FnLevelActorMessageType = super::ActorMessageType;
    type FnLevelActorMessageQueue = super::ActorMessageQueue;

    use super::super::super::hero::ffi::FnHeroData;
    use super::super::super::infobox::ffi::FnInfoMessageQueue;
    use super::super::super::level::ffi::FnLevelData;
    use super::super::super::tilecache::ffi::FnTileCache;
    use transdl::ll::SDL_Surface;

    #[repr(C)]
    pub struct FnLevelActorCreateParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut *mut std::ffi::c_void,
        pub level_data: *mut FnLevelData,
    }

    impl FnLevelActorCreateParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(&mut FnLevelActorData, &mut FnLevelData) -> S,
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*self.specific) };
            let level_data = unsafe { &mut (*self.level_data) };

            *specific = Box::into_raw(Box::new(f(general, level_data)))
                as *mut libc::c_void;
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*self.specific) };
            let level_data = unsafe { &mut (*self.level_data) };

            *specific =
                Box::into_raw(Box::new(T::create(general, level_data)))
                    as *mut libc::c_void;
        }
    }

    #[repr(C)]
    pub struct FnLevelActorFreeParams {
        pub specific: *mut *mut std::ffi::c_void,
    }

    impl FnLevelActorFreeParams {
        pub fn call<S>(self) {
            unsafe {
                if !(*self.specific).is_null() {
                    Box::from_raw(*self.specific);
                }
            }
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            unsafe {
                if !(*self.specific).is_null() {
                    Box::from_raw(*self.specific);
                }
            }
        }
    }

    #[repr(C)]
    pub struct FnLevelActorHeroTouchStartParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub actor_queue: *mut FnLevelActorQueue,
        pub hero_data: *mut FnHeroData,
    }

    impl FnLevelActorHeroTouchStartParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(
                &mut FnLevelActorData,
                &mut S,
                &mut FnLevelActorQueue,
                &mut FnHeroData,
            ),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.actor_queue.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let actor_queue = unsafe { &mut (*self.actor_queue) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            f(general, specific, actor_queue, hero_data)
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.actor_queue.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let actor_queue = unsafe { &mut (*self.actor_queue) };
            let hero_data = unsafe { &mut (*self.hero_data) };

            specific.hero_touch_start(general, actor_queue, hero_data);
        }
    }

    #[repr(C)]
    pub struct FnLevelActorHeroTouchEndParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub hero_data: *mut FnHeroData,
    }

    impl FnLevelActorHeroTouchEndParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(&mut FnLevelActorData, &mut S, &mut FnHeroData),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            f(general, specific, hero_data)
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let hero_data = unsafe { &mut (*self.hero_data) };

            specific.hero_touch_end(general, hero_data);
        }
    }

    #[repr(C)]
    pub struct FnLevelActorHeroInteractStartParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub level_data: *mut FnLevelData,
        pub hero_data: *mut FnHeroData,
        pub info_message_queue: *mut FnInfoMessageQueue,
        pub actor_message_queue: *mut FnLevelActorMessageQueue,
    }

    impl FnLevelActorHeroInteractStartParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(
                &mut FnLevelActorData,
                &mut S,
                &mut FnLevelData,
                &mut FnHeroData,
                &mut FnInfoMessageQueue,
                &mut FnLevelActorMessageQueue,
            ),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.hero_data.is_null());
            assert!(!self.info_message_queue.is_null());
            assert!(!self.actor_message_queue.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            let info_message_queue =
                unsafe { &mut (*self.info_message_queue) };
            let actor_message_queue =
                unsafe { &mut (*self.actor_message_queue) };
            f(
                general,
                specific,
                level_data,
                hero_data,
                info_message_queue,
                actor_message_queue,
            )
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.hero_data.is_null());
            assert!(!self.info_message_queue.is_null());
            assert!(!self.actor_message_queue.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            let info_message_queue =
                unsafe { &mut (*self.info_message_queue) };
            let actor_message_queue =
                unsafe { &mut (*self.actor_message_queue) };
            specific.hero_interact_start(
                general,
                level_data,
                hero_data,
                info_message_queue,
                actor_message_queue,
            );
        }
    }

    #[repr(C)]
    pub struct FnLevelActorHeroInteractEndParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub level_data: *mut FnLevelData,
        pub hero_data: *mut FnHeroData,
    }

    impl FnLevelActorHeroInteractEndParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(
                &mut FnLevelActorData,
                &mut S,
                &mut FnLevelData,
                &mut FnHeroData,
            ),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            f(general, specific, level_data, hero_data)
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            specific.hero_interact_end(general, level_data, hero_data);
        }
    }

    #[repr(C)]
    pub struct FnLevelActorActParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub level_data: *mut FnLevelData,
        pub actor_queue: *mut FnLevelActorQueue,
        pub hero_data: *mut FnHeroData,
    }

    impl FnLevelActorActParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(
                &mut FnLevelActorData,
                &mut S,
                &mut FnLevelData,
                &mut FnLevelActorQueue,
                &mut FnHeroData,
            ),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.actor_queue.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let actor_queue = unsafe { &mut (*self.actor_queue) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            f(general, specific, level_data, actor_queue, hero_data)
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.actor_queue.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let actor_queue = unsafe { &mut (*self.actor_queue) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            specific.act(general, level_data, actor_queue, hero_data);
        }
    }

    #[repr(C)]
    pub struct FnLevelActorBlitParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub hero_data: *mut FnHeroData,
        pub tilecache: *const FnTileCache,
        pub target: *mut SDL_Surface,
    }

    impl FnLevelActorBlitParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(
                &mut FnLevelActorData,
                &mut S,
                &mut FnHeroData,
                &FnTileCache,
                &mut transdl::video::Surface,
            ),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.hero_data.is_null());
            assert!(!self.tilecache.is_null());
            assert!(!self.target.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            let tilecache = unsafe { &(*self.tilecache) };
            let target = unsafe { &mut (*self.target) };

            let mut target = transdl::video::Surface { raw: target };

            f(general, specific, hero_data, tilecache, &mut target)
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.hero_data.is_null());
            assert!(!self.tilecache.is_null());
            assert!(!self.target.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            let tilecache = unsafe { &(*self.tilecache) };
            let target = unsafe { &mut (*self.target) };

            let mut target = transdl::video::Surface { raw: target };

            specific.blit(general, hero_data, tilecache, &mut target);
        }
    }

    #[repr(C)]
    pub struct FnLevelActorShotParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub level_data: *mut FnLevelData,
        pub actor_queue: *mut FnLevelActorQueue,
        pub hero_data: *mut FnHeroData,
    }

    impl FnLevelActorShotParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(
                &mut FnLevelActorData,
                &mut S,
                &mut FnLevelData,
                &mut FnLevelActorQueue,
                &mut FnHeroData,
            ),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.actor_queue.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let actor_queue = unsafe { &mut (*self.actor_queue) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            f(general, specific, level_data, actor_queue, hero_data)
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.level_data.is_null());
            assert!(!self.actor_queue.is_null());
            assert!(!self.hero_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let level_data = unsafe { &mut (*self.level_data) };
            let actor_queue = unsafe { &mut (*self.actor_queue) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            specific.shot(general, level_data, actor_queue, hero_data);
        }
    }

    #[repr(C)]
    pub struct FnLevelActorReceiveMessageParams {
        pub general: *mut FnLevelActorData,
        pub specific: *mut std::ffi::c_void,
        pub message: FnLevelActorMessageType,
        pub hero_data: *mut FnHeroData,
        pub level_data: *mut FnLevelData,
    }

    impl FnLevelActorReceiveMessageParams {
        pub fn call<F, S>(self, mut f: F)
        where
            F: FnMut(
                &mut FnLevelActorData,
                &mut S,
                FnLevelActorMessageType,
                &mut FnHeroData,
                &mut FnLevelData,
            ),
            S: std::fmt::Debug,
        {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.hero_data.is_null());
            assert!(!self.level_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut S)) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            let level_data = unsafe { &mut (*self.level_data) };
            f(general, specific, self.message, hero_data, level_data);
        }

        pub(crate) fn call_interface<T: super::ActorInterface>(self) {
            assert!(!self.general.is_null());
            assert!(!self.specific.is_null());
            assert!(!self.hero_data.is_null());
            assert!(!self.level_data.is_null());
            let general = unsafe { &mut (*self.general) };
            let specific = unsafe { &mut (*(self.specific as *mut T)) };
            let hero_data = unsafe { &mut (*self.hero_data) };
            let level_data = unsafe { &mut (*self.level_data) };
            specific.receive_message(
                general,
                self.message,
                hero_data,
                level_data,
            );
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_expose_all_function_parameters(
        _a: FnLevelActorCreateParams,
        _b: FnLevelActorFreeParams,
        _c: FnLevelActorHeroTouchStartParams,
        _d: FnLevelActorHeroTouchEndParams,
        _e: FnLevelActorHeroInteractStartParams,
        _f: FnLevelActorHeroInteractEndParams,
        _g: FnLevelActorActParams,
        _h: FnLevelActorBlitParams,
        _i: FnLevelActorShotParams,
        _j: FnLevelActorReceiveMessageParams,
    ) {
        // TODO: remove this function once all parameters are used.
        // Nothing to do here.
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_data_create(
        actor_type: FnLevelActorType,
    ) -> *mut FnLevelActorData {
        Box::into_raw(Box::new(FnLevelActorData::new(actor_type)))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_data_free(
        ptr: *mut FnLevelActorData,
    ) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
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
    pub extern "C" fn fn_level_actor_queue_push_particle_firework(
        ptr: *mut FnLevelActorQueue,
        x: u16,
        y: u16,
        count: usize,
    ) {
        assert!(!ptr.is_null());
        let queue = unsafe { &mut (*ptr) };
        queue.push_particle_firework(x, y, count);
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
    pub extern "C" fn fn_level_actor_message_queue_push_back(
        ptr: *mut FnLevelActorMessageQueue,
        receivers: FnLevelActorType,
        message: FnLevelActorMessageType,
    ) {
        assert!(!ptr.is_null());
        let queue = unsafe { &mut (*ptr) };
        queue.push_back(receivers, message);
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
}
