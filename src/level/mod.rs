pub mod raw;
pub mod solids;
pub mod tiles;

use super::actor::{
    ActorAdder, ActorMessageQueue, ActorQueue, ActorType, ActorsList,
    LevelActorAdder,
};
use super::geometry::Geometry;
use super::hero::HeroData;
use super::infobox::InfoMessageQueue;
use super::level::raw::LevelRaw;
use super::level::solids::LevelSolids;
use super::level::tiles::LevelTiles;
use super::shot::{Shot, ShotList};
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use super::HorizontalDirection;
use crate::{
    Result, ANIMATION_START, HALFTILE_WIDTH, LEVELWINDOW_HEIGHT,
    LEVELWINDOW_WIDTH, LEVEL_HEIGHT, LEVEL_WIDTH, SOLID_BLACK,
    SOLID_CONVEYORBELT_LEFTEND, TILE_HEIGHT, TILE_WIDTH,
};
use log::warn;
use std::io::Read;
use transdl::video::Surface;

#[derive(Debug)]
pub struct LevelData {
    pub tiles: LevelTiles,
    pub solids: LevelSolids,
    pub do_play: bool,
    pub level_passed: bool,
    pub actors: ActorsList,
    pub animated_frames_since_last_act: usize,
    pub shots: ShotList,
    pub surface_fixed: Surface,
    pub surface: Surface,
}

impl LevelData {
    pub fn load<R: Read>(
        reader: &mut R,
        hero: &mut HeroData,
        tilecache: &TileCache,
        texture_creation_params: TextureCreationParams,
        raw: &mut Option<&mut LevelRaw>,
    ) -> Result<Self> {
        let mut tiles = LevelTiles::new();
        let mut solids = LevelSolids::new();
        let mut actor_queue = ActorQueue::new();

        for i in 0..LEVEL_HEIGHT * LEVEL_WIDTH {
            let x = i % LEVEL_WIDTH;
            let y = i / LEVEL_WIDTH;

            let mut tile_buf = [0u8; 2];
            reader.read_exact(&mut tile_buf)?;

            let tx = (x * TILE_WIDTH) as u16;
            let ty = (y * TILE_HEIGHT) as u16;

            let mut aa = |actor_type, x, y| {
                actor_queue.add_actor(actor_type, x, y);
            };

            let tile = u16::from_le_bytes(tile_buf);
            if let Some(raw) = raw {
                raw.set(x, y, tile);
            }
            match tile {
                0x0000 => {}
                t if t >= 4 && t <= 0x2fe0 => {
                    tiles.set(x, y, t / 0x20);
                    solids.set(x, y, t >= 0x1800);
                }
                0x0080 =>
                /* written text on black screen */
                {
                    aa(ActorType::TextOnScreenBackground, tx, ty);
                }
                0x0100 =>
                /* blue high voltage flash */
                {
                    aa(ActorType::HighVoltageFlashBackground, tx, ty)
                }
                0x0180 =>
                /* red flash light */
                {
                    aa(ActorType::RedFlashlightBackground, tx, ty)
                }
                0x0200 =>
                /* blue high voltage flash */
                {
                    aa(ActorType::BlueFlashlightBackground, tx, ty)
                }
                0x0280 =>
                /* key panel on the wall */
                {
                    aa(ActorType::KeypanelBackground, tx, ty)
                }
                0x0300 =>
                /* red rotation light */
                {
                    aa(ActorType::RedRotationLightBackground, tx, ty)
                }
                0x0380 =>
                /* flashing up arrow */
                {
                    aa(ActorType::UpArrowBackground, tx, ty)
                }
                0x0400 =>
                /* background blinking blue box */
                {
                    aa(ActorType::BlueLightBackground1, tx, ty)
                }
                0x0420 =>
                /* background blinking blue box */
                {
                    aa(ActorType::BlueLightBackground2, tx, ty)
                }
                0x0440 =>
                /* background blinking blue box */
                {
                    aa(ActorType::BlueLightBackground3, tx, ty)
                }
                0x0460 =>
                /* background blinking blue box */
                {
                    aa(ActorType::BlueLightBackground4, tx, ty)
                }
                0x0480 =>
                /* background green poison liquid */
                {
                    aa(ActorType::GreenPoisonBackground, tx, ty)
                }
                0x0500 =>
                /* background lava */
                {
                    aa(ActorType::LavaBackground, tx, ty)
                }
                0x1800 =>
                /* solid wall which can be shot */
                {
                    tiles.set(x, y, 0x17E0 / 0x20);
                    aa(ActorType::ShootableWall, tx, ty);
                }
                0x1C00 =>
                /* center conveyor */
                {
                    tiles.set(x, y, SOLID_BLACK as u16);
                    solids.set(x, y, true);
                }
                0x3000 =>
                /* grey box, empty */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyEmpty, tx, ty);
                }
                0x3001 =>
                /* lift */
                {
                    solids.set(x, y, true);
                    aa(ActorType::Lift, tx, ty);
                }
                0x3002 =>
                /* left end of left-moving conveyor */
                {
                    tiles.set(x, y, SOLID_CONVEYORBELT_LEFTEND as u16);
                    solids.set(x, y, true);
                }
                0x3003 =>
                /* right end of left-moving conveyor */
                {
                    tiles.set(x, y, SOLID_BLACK as u16);
                    solids.set(x, y, true);
                    aa(ActorType::ConveyorLeftMovingRightEnd, tx, ty);
                }
                0x3004 =>
                /* left end of right-moving conveyor */
                {
                    tiles.set(x, y, SOLID_CONVEYORBELT_LEFTEND as u16);
                    solids.set(x, y, true);
                }
                0x3005 =>
                /* right end of right-moving conveyor */
                {
                    tiles.set(x, y, SOLID_BLACK as u16);
                    solids.set(x, y, true);
                    aa(ActorType::ConveyorRightMovingRightEnd, tx, ty);
                }
                0x3006 =>
                /* grey box with boots inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyBoots, tx, ty);
                }
                0x3007 =>
                /* rocket which gets started if shot
                 * and leaves a blue box with a balloon */
                {
                    aa(ActorType::Rocket, tx, ty);
                }
                0x3008 =>
                /* grey box with clamps inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyClamps, tx, ty);
                }
                0x3009 =>
                /* fire burning to the right */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::FireRight, tx, ty);
                }
                0x300A =>
                /* fire burning to the left */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::FireLeft, tx, ty);
                }
                0x300b =>
                /* flying techbot */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::FlyingBot, tx, ty);
                }
                0x300c =>
                /* footbot */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::FootBot, tx, ty);
                }
                0x300d =>
                /* tankbot */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::TankBot, tx, ty);
                }
                0x300e =>
                /* fire wheel bot */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::FireWheelBot, tx, ty);
                }
                0x300F =>
                /* grey box with gun inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyGun, tx, ty);
                }
                0x3010 =>
                /* robot */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Robot, tx, ty);
                }
                0x3011 =>
                /* exit door */
                {
                    aa(ActorType::ExitDoor, tx, ty - TILE_HEIGHT as u16);
                }
                0x3012 =>
                /* grey box with bomb inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyBomb, tx, ty);
                }
                0x3013 =>
                /* bot consisting of several white-blue balls */
                {
                    aa(ActorType::SnakeBot, tx, ty);
                }
                0x3014 =>
                /* water mirroring everything that is above */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    solids.set(x, y, true);
                    aa(ActorType::Water, tx, ty);
                }
                0x3015 =>
                /* red box with soda inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxRedSoda, tx, ty);
                }
                0x3016 =>
                /* crab bot crawling along wall left of him */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::WallCrawlerBotLeft, tx, ty);
                }
                0x3017 =>
                /* crab bot crawling along wall right of him */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::WallCrawlerBotRight, tx, ty);
                }
                0x3018 =>
                /* red box with chicken inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxRedChicken, tx, ty);
                }
                0x3019 =>
                /* floor that breaks on second jump onto it */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::UnstableFloor, tx, ty);
                }
                0x301a =>
                /* horizontal laser beam which gets deactivated when mill is shot */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::Laserbeam, tx, ty);
                }
                0x301b =>
                /* fan wheel mounted on right wall blowing to the left */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::FanLeft, tx, ty);
                }
                0x301c =>
                /* fan wheel mounted on left wall blowing to the right*/
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::FanRight, tx, ty);
                }
                0x301d =>
                /* blue box with football insdie */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxBlueFootball, tx, ty);
                }
                0x301e =>
                /* blue box with joystick inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxBlueJoystick, tx, ty);
                }
                0x301f =>
                /* blue box with disk inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxBlueDisk, tx, ty);
                }
                0x3020 =>
                /* grey box with glove inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyGlove, tx, ty);
                }
                0x3021 =>
                /* laser beam which is deactivated by access card */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    solids.set(x, y, true);
                    aa(ActorType::AccessCardDoor, tx, ty);
                }
                0x3022 =>
                /* helicopter */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::HelicopterBot, tx, ty);
                }
                0x3023 =>
                /* blue box with balloon inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxBlueBalloon, tx, ty);
                }
                0x3024 =>
                /* camera */
                {
                    aa(ActorType::Camera, tx, ty);
                }
                0x3025 =>
                /* broken wall background */
                {
                    /* take the part from one above */
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::BrokenWallBackground, tx, ty);
                }
                0x3026 =>
                /* left end of background stone wall */
                { /* TODO */ }
                0x3027 =>
                /* right end of background stone wall */
                { /* TODO */ }
                0x3028 =>
                /* window inside background stone wall */
                {
                    aa(ActorType::StoneWindowBackground, tx, ty);
                }
                0x3029 =>
                /* grey box with full life */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyFullLife, tx, ty);
                }
                0x302a =>
                /* "ACME" brick that comes falling down */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    solids.set(x, y, true);
                    aa(ActorType::Acme, tx, ty);
                }
                0x302b =>
                /* rotating mill that can kill duke on touch */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::Mill, tx, ty);
                }
                0x302c =>
                /* single spike standing out of the floor */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::Spike, tx, ty);
                }
                0x302d =>
                /* blue box with flag inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxBlueFlag, tx, ty);
                }
                0x302e =>
                /* blue box with radio inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxBlueRadio, tx, ty);
                }
                0x302f =>
                /* teleporter station */
                {
                    aa(ActorType::Teleporter1, tx, ty);
                }
                0x3030 =>
                /* teleporter station */
                {
                    aa(ActorType::Teleporter2, tx, ty);
                }
                0x3031 =>
                /* jumping mines */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::RedBallJumping, tx, ty);
                }
                0x3032 =>
                /* we found our hero! */
                {
                    hero.enter_level(
                        (x * TILE_WIDTH) as i16,
                        ((y - 1) * TILE_HEIGHT) as i16,
                    );
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                }
                0x3033 =>
                /* grey box with the access card inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyAccessCard, tx, ty);
                }
                0x3034 =>
                /* slot for access card */
                {
                    aa(ActorType::AccessCardSlot, tx, ty);
                }
                0x3035 =>
                /* slot for glove */
                {
                    aa(ActorType::GloveSlot, tx, ty);
                }
                0x3036 =>
                /* floor which expands to right by access of glove slot */
                {
                    solids.set(x, y, true);
                    aa(ActorType::ExpandingFloor, tx, ty);
                }
                0x3037 =>
                /* grey box with a D inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyLetterD, tx, ty);
                }
                0x3038 =>
                /* grey box with a U inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyLetterU, tx, ty);
                }
                0x3039 =>
                /* grey box with a K inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyLetterK, tx, ty);
                }
                0x303a =>
                /* grey box with a E inside */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::BoxGreyLetterE, tx, ty);
                }
                0x303b =>
                /* bunny bot */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::RabbitoidBot, tx, ty);
                }
                0x303c =>
                /* fire gnome */
                {
                    aa(ActorType::FlameGnomeBot, tx, ty);
                }
                0x303d =>
                /* fence with backdrop 1 behind it */
                {
                    aa(ActorType::FenceBackground, tx, ty);
                }
                0x303e =>
                /* window - left part */
                {
                    tiles.set(x, y, 0);
                    aa(ActorType::WindowLeftBackground, tx, ty);
                }
                0x303f =>
                /* window - right part */
                {
                    tiles.set(x, y, 0);
                    aa(ActorType::WindowRightBackground, tx, ty);
                }
                0x3040 =>
                /* the notebook */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Notebook, tx, ty);
                }
                0x3041 =>
                /* the surveillance screen */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::SurveillanceScreen, tx, ty);
                }
                0x3043 =>
                /* dr proton -the final opponent */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::DrProton, tx, ty);
                }
                0x3044 =>
                /* red key */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::KeyRed, tx, ty);
                }
                0x3045 =>
                /* green key */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::KeyGreen, tx, ty);
                }
                0x3046 =>
                /* blue key */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::KeyBlue, tx, ty);
                }
                0x3047 =>
                /* pink key */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::KeyPink, tx, ty);
                }
                0x3048 =>
                /* red keyhole */
                {
                    aa(ActorType::KeyholeRed, tx, ty);
                }
                0x3049 =>
                /* green keyhole */
                {
                    aa(ActorType::KeyholeGreen, tx, ty);
                }
                0x304a =>
                /* blue keyhole */
                {
                    aa(ActorType::KeyholeBlue, tx, ty);
                }
                0x304b =>
                /* pink keyhole */
                {
                    aa(ActorType::KeyholePink, tx, ty);
                }
                0x304c =>
                /* red door */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    solids.set(x, y, true);
                    aa(ActorType::DoorRed, tx, ty);
                }
                0x304d =>
                /* green door */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    solids.set(x, y, true);
                    aa(ActorType::DoorGreen, tx, ty);
                }
                0x304e =>
                /* blue door */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    solids.set(x, y, true);
                    aa(ActorType::DoorBlue, tx, ty);
                }
                0x304f =>
                /* pink door */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    solids.set(x, y, true);
                    aa(ActorType::DoorPink, tx, ty);
                }
                0x3050 =>
                /* football on its own */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Football, tx, ty);
                }
                0x3051 =>
                /* single chicken on its own */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::ChickenSingle, tx, ty);
                }
                0x3052 =>
                /* soda on its own */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Soda, tx, ty);
                }
                0x3053 =>
                /* a disk on its own */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Disk, tx, ty);
                }
                0x3054 =>
                /* a joystick on its own */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Joystick, tx, ty);
                }
                0x3055 =>
                /* a flag on its own */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Flag, tx, ty);
                }
                0x3056 =>
                /* a radio on its own */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::Radio, tx, ty);
                }
                0x3057 =>
                /* the red mine lying on the ground */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::RedBallLying, tx, ty);
                }
                0x3058 =>
                /* spikes showing up */
                {
                    if y > 0 {
                        tiles.copy_from_to(x, y - 1, x, y);
                    }
                    aa(ActorType::SpikesUp, tx, ty);
                }
                0x3059 =>
                /* spikes showing down */
                {
                    if x > 0 {
                        tiles.copy_from_to(x - 1, y, x, y);
                    }
                    aa(ActorType::SpikesDown, tx, ty);
                }
                t if (t as usize / 0x20 >= ANIMATION_START) => {
                    warn!(
                        "Unknown tile 0x{:04x} at x: {}, y: {}\n",
                        t, x, y
                    );
                    tiles.set(x, y, 2);
                }
                t => {
                    unreachable!("Unknown tile code: 0x{:04x}", t);
                }
            }
        }

        let mut actors = ActorsList::new();
        let mut actor_adder = LevelActorAdder {
            solids: &mut solids,
            tiles: &mut tiles,
            actors: &mut actors,
        };
        actor_queue.process(&mut actor_adder);

        let mut surface_fixed = texture_creation_params.create_surface(
            TILE_WIDTH as u16 * LEVEL_WIDTH as u16,
            TILE_HEIGHT as u16 * LEVEL_HEIGHT as u16,
        );
        let surface = texture_creation_params.create_surface(
            TILE_WIDTH as u16 * LEVEL_WIDTH as u16,
            TILE_HEIGHT as u16 * LEVEL_HEIGHT as u16,
        );

        let transparent = super::sdl_surface_transparent(&surface_fixed);
        surface_fixed.fill(transparent);

        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                let tilenr = tiles.get(x, y);
                if tilenr > 1 && tilenr < (48 * 8) {
                    let destrect = Geometry {
                        x: (TILE_WIDTH * x) as i16,
                        y: (TILE_HEIGHT * y) as i16,
                        w: TILE_WIDTH as u16,
                        h: TILE_HEIGHT as u16,
                    };
                    tilecache
                        .get_tile(tilenr as usize)
                        .unwrap()
                        .blit_to_sdl_surface(
                            None,
                            &mut surface_fixed,
                            Some(destrect),
                        );
                }
            }
        }

        Ok(LevelData {
            tiles,
            solids,
            do_play: true,
            level_passed: false,
            actors,
            animated_frames_since_last_act: 0,
            shots: Vec::new(),
            surface_fixed,
            surface,
        })
    }

    pub fn hero_interact_start(
        &mut self,
        hero: &mut HeroData,
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        self.actors.start_interaction(
            &mut self.level_passed,
            hero,
            info_message_queue,
            actor_message_queue,
        );
    }

    pub fn hero_interact_end(&mut self, hero: &mut HeroData) {
        self.actors.end_interaction(&mut self.level_passed, hero);
    }

    pub fn animated_frames_since_last_act_increase(&mut self) -> usize {
        self.animated_frames_since_last_act += 1;
        self.animated_frames_since_last_act %= 1;
        self.animated_frames_since_last_act
    }

    pub fn blit(
        &mut self,
        target: &mut Surface,
        tilecache: &TileCache,
        hero: &mut HeroData,
        draw_collision_bounds: bool,
        targetrect: Geometry,
        sourcerect: Geometry,
        backdrop1: Option<&Texture>,
        _backdrop2: Option<&Texture>,
    ) {
        if let Some(backdrop) = backdrop1 {
            backdrop.blit_to_sdl_surface(
                None,
                &mut self.surface,
                Some(sourcerect),
            )
        } else {
            self.surface.fill_rect(sourcerect.as_sdl_rect(), 0);
        }

        self.surface_fixed.blit(
            Some(sourcerect.as_sdl_rect()),
            &mut self.surface,
            Some(sourcerect.as_sdl_rect()),
        );

        // calculate the bounds of the area we have to blit
        let (x_start, x_end) = {
            let mut start = (sourcerect.x as usize / TILE_WIDTH)
                .saturating_sub(LEVELWINDOW_WIDTH / 2);
            let mut end = start + (sourcerect.w as usize / TILE_WIDTH) * 2;
            if end > LEVEL_WIDTH {
                let diff = end - LEVEL_WIDTH;
                start = start.saturating_sub(diff);
                end = end.saturating_sub(diff);
            }
            (start, end)
        };
        let (y_start, y_end) = {
            let mut start = (sourcerect.y as usize / TILE_HEIGHT)
                .saturating_sub(LEVELWINDOW_WIDTH / 2);
            let mut end = start
                + (sourcerect.h as usize / TILE_HEIGHT) * 2
                + LEVELWINDOW_HEIGHT / 2;
            if end > LEVEL_HEIGHT {
                let diff = end - LEVEL_HEIGHT;
                start = start.saturating_sub(diff);
                end = end.saturating_sub(diff);
            }
            (start, end)
        };

        self.actors.update_visibility(
            x_start as i16,
            x_end as i16,
            y_start as i16,
            y_end as i16,
        );

        self.actors.blit_background_actors(
            hero,
            &mut self.surface,
            tilecache,
            draw_collision_bounds,
        );

        hero.blit(
            &mut self.surface,
            tilecache,
            &self.solids,
            draw_collision_bounds,
        );

        self.actors.blit_foreground_actors(
            hero,
            &mut self.surface,
            tilecache,
            draw_collision_bounds,
        );

        for shot in self.shots.iter() {
            shot.blit(&mut self.surface, tilecache, draw_collision_bounds);
        }

        self.surface.blit(
            Some(sourcerect.as_sdl_rect()),
            target,
            Some(targetrect.as_sdl_rect()),
        );
    }

    pub fn act(
        &mut self,
        hero_data: &mut HeroData,
        actor_queue: &mut ActorQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        let animated_frames =
            self.animated_frames_since_last_act_increase();

        for shot in self.shots.iter_mut() {
            shot.act(
                hero_data,
                &mut self.actors,
                &mut self.solids,
                &mut self.tiles,
                actor_queue,
            );
        }
        self.shots.retain(|s| s.is_alive);

        for message in actor_message_queue.messages.drain(..) {
            self.actors.send_message(
                message.receivers,
                message.message,
                hero_data,
                &mut self.solids,
            );
        }

        self.actors.act(
            &mut self.solids,
            &mut self.tiles,
            hero_data,
            actor_queue,
            &mut self.do_play,
        );

        if animated_frames == 0 {
            hero_data.act(&mut self.solids);
        }
        hero_data.next_frame();
        hero_data.update_animation();
    }

    pub fn fire_shot(
        &mut self,
        hero: &mut HeroData,
        actor_adder: &mut dyn ActorAdder,
    ) {
        if self.shots.len() < hero.firepower.num_shots() as usize {
            let heropos = hero.position.geometry;
            let mut shot = Shot::new(heropos.x, heropos.y, hero.direction);

            let distance = match shot.direction {
                HorizontalDirection::Left => -(HALFTILE_WIDTH as i16),
                HorizontalDirection::Right => HALFTILE_WIDTH as i16,
                _ => unreachable!(),
            };

            // we only push half of the distance, but do it twice, so that
            // also the intermediate position gets covered, not just the
            // end position.
            shot.push(
                hero,
                &mut self.actors,
                &mut self.solids,
                &mut self.tiles,
                distance,
                actor_adder,
            );

            self.shots.push(shot);
        }
    }
}
