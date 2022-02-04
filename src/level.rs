pub mod tiles;

use crate::{
    actor::{
        ActorAdder, ActorMessageQueue, ActorQueue, ActorType, ActorsList,
        BackgroundAnimationType, BoxBlueContent, BoxGreyContent,
        BoxRedContent, ItemType, LevelActorAdder, SpikeType,
        TeleporterIndex,
    },
    hero::Hero,
    infobox::InfoMessageQueue,
    rendering::Renderer,
    shot::{Shot, ShotList},
    HorizontalDirection, KeyColor, Result, Sizes, ANIMATION_START,
    LEVEL_HEIGHT, LEVEL_WIDTH, SOLID_BLACK, SOLID_CONVEYORBELT_LEFTEND,
};
use log::warn;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    surface::Surface,
};
use std::io::Read;
use tiles::LevelTiles;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayState {
    Playing,
    LevelFinished,
    KilledPlayingAnimation(usize),
    RestartLevel,
    GoToMainScreen,
}

impl PlayState {
    pub fn keep_acting(&self) -> bool {
        matches!(
            self,
            PlayState::Playing | PlayState::KilledPlayingAnimation(_)
        )
    }

    pub fn hero_can_act(&self) -> bool {
        matches!(self, PlayState::Playing)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum BackgroundTileStrategy {
    KeepEmpty,
    SetTile(u16),
    CopyFromAbove,
    CopyFromLeft,
    CopyFromRight,
    CopyFromBelow,
}

#[derive(Debug)]
pub struct Level {
    pub tiles: LevelTiles,
    pub play_state: PlayState,
    pub actors: ActorsList,
    pub animated_frames_since_last_act: usize,
    pub shots: ShotList,
}

impl Level {
    pub fn load<R: Read>(
        reader: &mut R,
        hero: &mut Hero,
        sizes: &dyn Sizes,
    ) -> Result<Self> {
        let mut tiles = LevelTiles::new();
        let mut actor_queue = ActorQueue::new();

        for i in 0..LEVEL_HEIGHT * LEVEL_WIDTH {
            let x = (i % LEVEL_WIDTH) as i32;
            let y = (i / LEVEL_WIDTH) as i32;

            let mut tile_buf = [0u8; 2];
            reader.read_exact(&mut tile_buf)?;

            let tx = x * sizes.width() as i32;
            let ty = y * sizes.height() as i32;

            let mut aa = |actor_type, x, y| {
                actor_queue.add_actor(actor_type, Point::new(x, y));
            };

            let tile = u16::from_le_bytes(tile_buf);
            let mut t = tiles.get_mut(x, y)?;
            t.raw_number = tile;
            if tile >= 4 && tile <= 0x2fe0 {
                t.solid = tile >= 0x1800;
                t.effective_number = tile / 0x20;
            }
            match tile {
                0x0000 => {}
                0x0080 =>
                /* written text on black screen */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::TextOnScreen,
                        ),
                        tx,
                        ty,
                    );
                }
                0x0100 =>
                /* blue high voltage flash */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::HighVoltageFlash,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0180 =>
                /* red flash light */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::RedFlashlight,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0200 =>
                /* blue high voltage flash */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::BlueFlashlight,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0280 =>
                /* key panel on the wall */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::Keypanel,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0300 =>
                /* red rotation light */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::RedRotationLight,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0380 =>
                /* flashing up arrow */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::UpArrow,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0400 =>
                /* background blinking blue box */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::BlueLight1,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0420 =>
                /* background blinking blue box */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::BlueLight2,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0440 =>
                /* background blinking blue box */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::BlueLight3,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0460 =>
                /* background blinking blue box */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::BlueLight4,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0500 =>
                /* background green poison liquid */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::GreenPoison,
                        ),
                        tx,
                        ty,
                    )
                }
                0x0580 =>
                /* background lava */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::Lava,
                        ),
                        tx,
                        ty,
                    )
                }
                0x1800 =>
                /* solid wall which can be shot */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number = 0x17E0 / 0x20;
                    }
                    aa(ActorType::ShootableWall, tx, ty);
                }
                0x1C00 =>
                /* center conveyor */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number = SOLID_BLACK as u16;
                        t.solid = true;
                    }
                }
                0x3000 =>
                /* grey box, empty */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::Nothing,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3001 =>
                /* elevator */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::Elevator, tx, ty);
                }
                0x3002 =>
                /* left end of left-moving conveyor */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number =
                            SOLID_CONVEYORBELT_LEFTEND as u16;
                        t.solid = true;
                    }
                }
                0x3003 =>
                /* right end of left-moving conveyor */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number = SOLID_BLACK as u16;
                        t.solid = true;
                    }
                    aa(
                        ActorType::ConveyorRightEnd(
                            HorizontalDirection::Left,
                        ),
                        tx,
                        ty,
                    );
                }
                0x3004 =>
                /* left end of right-moving conveyor */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number =
                            SOLID_CONVEYORBELT_LEFTEND as u16;
                        t.solid = true;
                    }
                }
                0x3005 =>
                /* right end of right-moving conveyor */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number = SOLID_BLACK as u16;
                        t.solid = true;
                    }
                    aa(
                        ActorType::ConveyorRightEnd(
                            HorizontalDirection::Right,
                        ),
                        tx,
                        ty,
                    );
                }
                0x3006 =>
                /* grey box with boots inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::Boots,
                        )),
                        tx,
                        ty,
                    );
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
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::Clamps,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3009 =>
                /* fire burning to the right */
                {
                    aa(
                        ActorType::Fire(HorizontalDirection::Right),
                        tx,
                        ty,
                    );
                }
                0x300A =>
                /* fire burning to the left */
                {
                    aa(ActorType::Fire(HorizontalDirection::Left), tx, ty);
                }
                0x300b =>
                /* flying techbot */
                {
                    aa(ActorType::FlyingBot, tx, ty);
                }
                0x300c =>
                /* footbot */
                {
                    aa(ActorType::FootBot, tx, ty);
                }
                0x300d =>
                /* tankbot */
                {
                    aa(ActorType::TankBot, tx, ty);
                }
                0x300e =>
                /* fire wheel bot */
                {
                    aa(ActorType::FireWheelBot, tx, ty);
                }
                0x300F =>
                /* grey box with gun inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::Gun,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3010 =>
                /* robot */
                {
                    aa(ActorType::Robot, tx, ty);
                }
                0x3011 =>
                /* exit door */
                {
                    aa(
                        ActorType::ExitDoor,
                        tx,
                        ty - sizes.height() as i32,
                    );
                }
                0x3012 =>
                /* grey box with bomb inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::Bomb,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3013 =>
                /* bot consisting of several white-blue balls */
                {
                    aa(ActorType::SnakeBot, tx, ty);
                }
                0x3014 =>
                /* water mirroring everything that is above */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::Water, tx, ty);
                }
                0x3015 =>
                /* red box with soda inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxRed(
                            BoxRedContent::Soda,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3016 =>
                /* crab bot crawling along wall left of him */
                {
                    aa(
                        ActorType::WallCrawler(HorizontalDirection::Left),
                        tx,
                        ty,
                    );
                }
                0x3017 =>
                /* crab bot crawling along wall right of him */
                {
                    aa(
                        ActorType::WallCrawler(HorizontalDirection::Right),
                        tx,
                        ty,
                    );
                }
                0x3018 =>
                /* red box with chicken inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxRed(
                            BoxRedContent::Chicken,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3019 =>
                /* floor that breaks on second jump onto it */
                {
                    aa(ActorType::UnstableFloor, tx, ty);
                }
                0x301a =>
                /* horizontal electric arc which gets deactivated when mill is shot */
                {
                    aa(ActorType::ElectricArc, tx, ty);
                }
                0x301b =>
                /* fan wheel mounted on right wall blowing to the left */
                {
                    aa(ActorType::Fan(HorizontalDirection::Left), tx, ty);
                }
                0x301c =>
                /* fan wheel mounted on left wall blowing to the right*/
                {
                    aa(ActorType::Fan(HorizontalDirection::Right), tx, ty);
                }
                0x301d =>
                /* blue box with football insdie */
                {
                    aa(
                        ActorType::Item(ItemType::BoxBlue(
                            BoxBlueContent::Football,
                        )),
                        tx,
                        ty,
                    );
                }
                0x301e =>
                /* blue box with joystick inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxBlue(
                            BoxBlueContent::Joystick,
                        )),
                        tx,
                        ty,
                    );
                }
                0x301f =>
                /* blue box with disk inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxBlue(
                            BoxBlueContent::Disk,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3020 =>
                /* grey box with glove inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::Glove,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3021 =>
                /* laser beam which is deactivated by access card */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::AccessCardDoor, tx, ty);
                }
                0x3022 =>
                /* helicopter */
                {
                    aa(ActorType::HelicopterBot, tx, ty);
                }
                0x3023 =>
                /* blue box with balloon inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxBlue(
                            BoxBlueContent::Balloon,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3024 =>
                /* camera */
                {
                    aa(ActorType::Camera, tx, ty);
                }
                0x3025 =>
                /* broken wall background */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::BrokenWall,
                        ),
                        tx,
                        ty,
                    );
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
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::StoneWindow,
                        ),
                        tx,
                        ty,
                    );
                }
                0x3029 =>
                /* grey box with full life */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::FullLife,
                        )),
                        tx,
                        ty,
                    );
                }
                0x302a =>
                /* "ACME" brick that comes falling down */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::Acme, tx, ty);
                }
                0x302b =>
                /* rotating mill that can kill the hero on touch */
                {
                    aa(ActorType::Mill, tx, ty);
                }
                0x302c =>
                /* single spike standing out of the floor */
                {
                    aa(
                        ActorType::Spikes(SpikeType::SingleSpikeUp),
                        tx,
                        ty,
                    );
                }
                0x302d =>
                /* blue box with flag inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxBlue(
                            BoxBlueContent::Flag,
                        )),
                        tx,
                        ty,
                    );
                }
                0x302e =>
                /* blue box with radio inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxBlue(
                            BoxBlueContent::Radio,
                        )),
                        tx,
                        ty,
                    );
                }
                0x302f =>
                /* teleporter station */
                {
                    aa(
                        ActorType::Teleporter(TeleporterIndex::First),
                        tx,
                        ty,
                    );
                }
                0x3030 =>
                /* teleporter station */
                {
                    aa(
                        ActorType::Teleporter(TeleporterIndex::Second),
                        tx,
                        ty,
                    );
                }
                0x3031 =>
                /* jumping mines */
                {
                    aa(ActorType::MineJumping, tx, ty);
                }
                0x3032 =>
                /* we found our hero! */
                {
                    hero.enter_level(tx, ty - sizes.height() as i32);
                    let source = tiles
                        .get(x as i32 - 1, y as i32)
                        .map(|t| t.effective_number)
                        .unwrap_or(0);
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number = source;
                    }
                }
                0x3033 =>
                /* grey box with the access card inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::AccessCard,
                        )),
                        tx,
                        ty,
                    );
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
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::ExpandingFloor, tx, ty);
                }
                0x3037 =>
                /* grey box with a D inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::LetterD,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3038 =>
                /* grey box with a U inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::LetterU,
                        )),
                        tx,
                        ty,
                    );
                }
                0x3039 =>
                /* grey box with a K inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::LetterK,
                        )),
                        tx,
                        ty,
                    );
                }
                0x303a =>
                /* grey box with a E inside */
                {
                    aa(
                        ActorType::Item(ItemType::BoxGrey(
                            BoxGreyContent::LetterE,
                        )),
                        tx,
                        ty,
                    );
                }
                0x303b =>
                /* bunny bot */
                {
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
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::WindowLeft,
                        ),
                        tx,
                        ty,
                    );
                }
                0x303f =>
                /* window - right part */
                {
                    aa(
                        ActorType::BackgroundAnimation(
                            BackgroundAnimationType::WindowRight,
                        ),
                        tx,
                        ty,
                    );
                }
                0x3040 =>
                /* the notebook */
                {
                    aa(ActorType::NoteBook, tx, ty);
                }
                0x3041 =>
                /* the surveillance screen */
                {
                    aa(ActorType::SurveillanceScreen, tx, ty);
                }
                0x3043 =>
                /* dr proton -the final opponent */
                {
                    aa(ActorType::DrProton, tx, ty);
                }
                0x3044 =>
                /* red key */
                {
                    aa(ActorType::Key(KeyColor::Red), tx, ty);
                }
                0x3045 =>
                /* green key */
                {
                    aa(ActorType::Key(KeyColor::Green), tx, ty);
                }
                0x3046 =>
                /* blue key */
                {
                    aa(ActorType::Key(KeyColor::Blue), tx, ty);
                }
                0x3047 =>
                /* pink key */
                {
                    aa(ActorType::Key(KeyColor::Pink), tx, ty);
                }
                0x3048 =>
                /* red keyhole */
                {
                    aa(ActorType::Keyhole(KeyColor::Red), tx, ty);
                }
                0x3049 =>
                /* green keyhole */
                {
                    aa(ActorType::Keyhole(KeyColor::Green), tx, ty);
                }
                0x304a =>
                /* blue keyhole */
                {
                    aa(ActorType::Keyhole(KeyColor::Blue), tx, ty);
                }
                0x304b =>
                /* pink keyhole */
                {
                    aa(ActorType::Keyhole(KeyColor::Pink), tx, ty);
                }
                0x304c =>
                /* red door */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::Door(KeyColor::Red), tx, ty);
                }
                0x304d =>
                /* green door */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::Door(KeyColor::Green), tx, ty);
                }
                0x304e =>
                /* blue door */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::Door(KeyColor::Blue), tx, ty);
                }
                0x304f =>
                /* pink door */
                {
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.solid = true;
                    }
                    aa(ActorType::Door(KeyColor::Pink), tx, ty);
                }
                0x3050 =>
                /* football on its own */
                {
                    aa(ActorType::Item(ItemType::Football), tx, ty);
                }
                0x3051 =>
                /* single chicken on its own */
                {
                    aa(ActorType::Item(ItemType::ChickenSingle), tx, ty);
                }
                0x3052 =>
                /* soda on its own */
                {
                    aa(ActorType::Item(ItemType::Soda), tx, ty);
                }
                0x3053 =>
                /* a disk on its own */
                {
                    aa(ActorType::Item(ItemType::Disk), tx, ty);
                }
                0x3054 =>
                /* a joystick on its own */
                {
                    aa(ActorType::Item(ItemType::Joystick), tx, ty);
                }
                0x3055 =>
                /* a flag on its own */
                {
                    aa(ActorType::Item(ItemType::Flag), tx, ty);
                }
                0x3056 =>
                /* a radio on its own */
                {
                    aa(ActorType::Item(ItemType::Radio), tx, ty);
                }
                0x3057 =>
                /* the red mine lying on the ground */
                {
                    aa(ActorType::MineLying, tx, ty);
                }
                0x3058 =>
                /* spikes showing up */
                {
                    aa(ActorType::Spikes(SpikeType::SpikesUp), tx, ty);
                }
                0x3059 =>
                /* spikes showing down */
                {
                    aa(ActorType::Spikes(SpikeType::SpikesDown), tx, ty);
                }
                t if t >= 4 && t <= 0x2fe0 => {}
                t if (t as usize / 0x20 >= ANIMATION_START) => {
                    warn!(
                        "Unknown tile 0x{:04x} at x: {}, y: {}\n",
                        t, x, y
                    );
                    if let Ok(ref mut t) = tiles.get_mut(x, y) {
                        t.effective_number = 2;
                    }
                }
                t => {
                    unreachable!("Unknown tile code: 0x{:04x}", t);
                }
            }
        }

        let mut actors = ActorsList::new();
        let mut actor_adder = LevelActorAdder {
            sizes,
            tiles: &mut tiles,
            actors: &mut actors,
            copy_background: true,
        };
        actor_queue.process(&mut actor_adder);

        Ok(Self {
            tiles,
            play_state: PlayState::Playing,
            actors,
            animated_frames_since_last_act: 0,
            shots: Vec::new(),
        })
    }

    pub fn hero_interact_start(
        &mut self,
        hero: &mut Hero,
        info_message_queue: &mut InfoMessageQueue,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        self.actors.start_interaction(
            &mut self.play_state,
            hero,
            info_message_queue,
            actor_message_queue,
        );
    }

    pub fn hero_interact_end(&mut self, hero: &mut Hero) {
        self.actors.end_interaction(&mut self.play_state, hero);
    }

    pub fn animated_frames_since_last_act_increase(&mut self) -> usize {
        self.animated_frames_since_last_act += 1;
        self.animated_frames_since_last_act %= 1;
        self.animated_frames_since_last_act
    }

    pub fn render(
        &mut self,
        renderer: &mut dyn Renderer,
        sizes: &dyn Sizes,
        hero: &mut Hero,
        draw_collision_bounds: bool,
        srcrect: Rect,
        backdrop1: Option<&Surface>,
        _backdrop2: Option<&Surface>,
    ) -> Result<()> {
        if let Some(backdrop) = backdrop1 {
            renderer.place_surface(backdrop, srcrect)?;
        } else {
            renderer.fill_rect(srcrect, Color::RGB(0, 0, 0))?;
        }

        let start_x = srcrect.left() / sizes.width() as i32;
        let end_x = (srcrect.right() / sizes.width() as i32) + 1;
        let start_y = srcrect.top() / sizes.height() as i32;
        let end_y = (srcrect.bottom() / sizes.height() as i32) + 1;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let tilenr = self
                    .tiles
                    .get(x, y)
                    .map(|t| t.effective_number)
                    .unwrap_or(0);
                if tilenr > 1 && tilenr < (48 * 8) {
                    let point = Point::new(
                        sizes.width() as i32 * x,
                        sizes.height() as i32 * y,
                    );
                    renderer.place_tile(tilenr as usize, point)?;
                }
            }
        }

        self.actors.render_background_actors(
            renderer,
            sizes,
            draw_collision_bounds,
            srcrect,
        )?;

        hero.render(renderer, sizes, &self.tiles, draw_collision_bounds)?;

        self.actors.render_foreground_actors(
            renderer,
            sizes,
            draw_collision_bounds,
            srcrect,
        )?;

        for shot in self.shots.iter() {
            shot.render(renderer, sizes, draw_collision_bounds)?;
        }
        Ok(())
    }

    pub fn act(
        &mut self,
        sizes: &dyn Sizes,
        hero: &mut Hero,
        actor_queue: &mut ActorQueue,
        actor_message_queue: &mut ActorMessageQueue,
        visible_rect: Rect,
    ) -> Result<()> {
        let animated_frames =
            self.animated_frames_since_last_act_increase();

        for shot in self.shots.iter_mut() {
            shot.act(
                sizes,
                hero,
                &mut self.actors,
                &mut self.tiles,
                actor_queue,
                actor_message_queue,
            );
        }
        self.shots.retain(|s| s.is_alive);

        for message in actor_message_queue.messages.drain(..) {
            self.actors.send_message(
                message,
                sizes,
                hero,
                &mut self.tiles,
            );
        }

        self.actors.act(
            sizes,
            &mut self.tiles,
            hero,
            actor_queue,
            &mut self.play_state,
            visible_rect,
            actor_message_queue,
        );

        if self.play_state.hero_can_act() && animated_frames == 0 {
            hero.act(sizes, &self.tiles, actor_queue)?;
        }
        hero.next_frame();
        hero.update_animation();

        if hero.health.life().is_none()
            && self.play_state == PlayState::Playing
        {
            self.play_state = PlayState::KilledPlayingAnimation(80);
        }

        Ok(())
    }

    pub fn fire_shot(
        &mut self,
        sizes: &dyn Sizes,
        hero: &mut Hero,
        actor_adder: &mut dyn ActorAdder,
        actor_message_queue: &mut ActorMessageQueue,
    ) {
        if self.shots.len() < hero.firepower.num_shots() as usize
            && self.play_state.hero_can_act()
        {
            let heropos = hero.position.geometry;
            let mut shot =
                Shot::new(sizes, heropos.x, heropos.y, hero.direction);

            let distance =
                sizes.half_width() as i32 * shot.direction.as_factor_i32();

            // we only push half of the distance, but do it twice, so that
            // also the intermediate position gets covered, not just the
            // end position.
            shot.push(
                sizes,
                hero,
                &mut self.actors,
                &mut self.tiles,
                distance,
                actor_adder,
                actor_message_queue,
            );

            self.shots.push(shot);
        }
    }
}
