use super::super::geometry::Geometry;

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
    Fire,
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
pub struct ActorData {
    pub actor_type: ActorType,
    pub position: Geometry,
    pub is_in_foreground: bool,
    pub hurts_hero: bool,
    pub is_alive: bool,
}

impl ActorData {
    pub fn new(actor_type: ActorType) -> Self {
        ActorData {
            actor_type,
            position: Geometry::default(),
            is_in_foreground: true,
            hurts_hero: false,
            is_alive: true,
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
    pub fn push_back(&mut self, item: ActorQueueItem) {
        self.actors.push(item);
    }
}

pub mod ffi {
    type FnLevelActorData = super::ActorData;
    type FnLevelActorType = super::ActorType;
    type FnLevelActorQueue = super::ActorQueue;
    type FnLevelActorQueueItem = super::ActorQueueItem;

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
}
