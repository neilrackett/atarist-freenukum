mod accesscarddoor;
mod accesscardslot;
mod acme;
mod backgroundanimation;
mod balloon;
mod bomb;
mod camera;
mod conveyor;
mod door;
mod electricarc;
mod elevator;
mod exitdoor;
mod expandingfloor;
mod fan;
mod fire;
mod firewheelbot;
mod gloveslot;
mod hostileshot;
mod item;
mod key;
mod keyhole;
mod mill;
mod minejumping;
mod minelying;
mod notebook;
mod particle;
mod placeholder;
mod robot;
mod rocket;
mod score;
mod shootablewall;
mod singleanimation;
mod sodaflying;
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
    level::{tiles::LevelTiles, BackgroundTileStrategy, PlayState},
    rendering::Renderer,
    HorizontalDirection, KeyColor, Result, Sizes,
};
use sdl2::rect::{Point, Rect};
pub(crate) use {
    accesscarddoor::AccessCardDoor,
    accesscardslot::AccessCardSlot,
    acme::Acme,
    backgroundanimation::{BackgroundAnimation, BackgroundAnimationType},
    balloon::Balloon,
    bomb::Bomb,
    camera::Camera,
    conveyor::Conveyor,
    door::Door,
    electricarc::ElectricArc,
    elevator::Elevator,
    exitdoor::ExitDoor,
    expandingfloor::ExpandingFloor,
    fan::Fan,
    fire::Fire,
    firewheelbot::FireWheelBot,
    gloveslot::GloveSlot,
    hostileshot::HostileShot,
    item::{
        BoxBlueContent, BoxGreyContent, BoxRedContent, Item, ItemType,
    },
    key::Key,
    keyhole::KeyHole,
    mill::Mill,
    minejumping::MineJumping,
    minelying::MineLying,
    notebook::NoteBook,
    particle::{Particle, ParticleColor},
    placeholder::PlaceHolder,
    robot::Robot,
    rocket::Rocket,
    score::{Score, ScoreType},
    shootablewall::ShootableWall,
    singleanimation::{SingleAnimation, SingleAnimationType},
    sodaflying::SodaFlying,
    spikes::{SpikeType, Spikes},
    surveillancescreen::SurveillanceScreen,
    tankbot::TankBot,
    teleporter::{Teleporter, TeleporterIndex},
    unstablefloor::UnstableFloor,
    wallcrawler::WallCrawler,
};

#[derive(Debug)]
pub struct ActorsList {
    actors: Vec<Box<dyn Actor>>,
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
        message: ActorMessageType,
        sizes: &dyn Sizes,
        hero: &mut Hero,
        tiles: &mut LevelTiles,
    ) {
        for actor in self
            .actors
            .iter_mut()
            .filter(|a| a.can_receive_message(message))
        {
            let p = ReceiveMessageParameters {
                message,
                sizes,
                hero,
                tiles,
            };
            actor.receive_message(p);
        }
    }

    pub fn process_shot(
        &mut self,
        shot_position: Rect,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero: &mut Hero,
        actor_message_queue: &mut ActorMessageQueue,
    ) -> bool {
        for actor in self.actors.iter_mut() {
            let p = ShotParameters {
                sizes,
                tiles,
                actor_adder,
                hero,
                actor_message_queue,
            };
            if actor.can_get_shot()
                && shot_position.touches(actor.position())
                && actor.shot(p) == ShotProcessing::Absorb
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
                play_state,
                hero,
                info_message_queue,
                actor_message_queue,
            };
            self.interaction_target = Some(i);
            actor.hero_interact_start(p);
        }
    }

    pub fn end_interaction(
        &mut self,
        play_state: &mut PlayState,
        hero: &mut Hero,
    ) {
        if let Some(i) = self.interaction_target.take() {
            if let Some(actor) = self.actors.get_mut(i) {
                let p = HeroInteractEndParameters { play_state, hero };
                actor.hero_interact_end(p);
            }
        }
    }

    pub fn act(
        &mut self,
        sizes: &dyn Sizes,
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
                let p = ActParameters {
                    sizes,
                    tiles,
                    hero,
                    actor_adder: actor_queue,
                    play_state,
                };
                actor.act(p);
                if actor.is_alive() && actor.hurts_hero(hero) {
                    actors_hurting_hero += 1;
                }
            }
        }
        self.remove_dead();

        let mut adder = LevelActorAdder {
            sizes,
            tiles,
            actors: self,
            copy_background: false,
        };

        actor_queue.process(&mut adder);
        hero.gets_hurt = actors_hurting_hero > 0;
    }

    pub fn render_background_actors(
        &mut self,
        renderer: &mut dyn Renderer,
        sizes: &dyn Sizes,
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
            .map(|actor| {
                let p = RenderParameters { renderer, sizes };
                let res = actor.render(p);
                if res.is_ok() && draw_collision_bounds {
                    let color = crate::collision_bounds_color();
                    renderer.draw_rect(actor.position(), color)?;
                }
                res
            })
            .collect::<Result<_>>()?)
    }

    pub fn render_foreground_actors(
        &mut self,
        renderer: &mut dyn Renderer,
        sizes: &dyn Sizes,
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
            .map(|actor| {
                let p = RenderParameters { renderer, sizes };
                let res = actor.render(p);
                if res.is_ok() && draw_collision_bounds {
                    let color = crate::collision_bounds_color();
                    renderer.draw_rect(actor.position(), color)?;
                }
                res
            })
            .collect::<Result<_>>()?)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShotProcessing {
    Absorb,
    Ignore,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActorType {
    FireWheelBot,
    FlameGnomeBot,
    FlyingBot,
    FootBot,
    HelicopterBot,
    RabbitoidBot,
    MineJumping,
    MineLying,
    Robot,
    SingleAnimation(SingleAnimationType),
    SnakeBot,
    TankBot,
    WallCrawler(HorizontalDirection),
    DrProton,
    Camera,
    Particle(ParticleColor),
    Rocket,
    Bomb,
    Water,
    ExitDoor,
    NoteBook,
    SurveillanceScreen,
    HostileShot(HorizontalDirection),
    SodaFlying,
    UnstableFloor,
    ExpandingFloor,
    ConveyorRightEnd(HorizontalDirection),
    Fan(HorizontalDirection),
    StoneBackground,
    Teleporter(TeleporterIndex),
    FenceBackground,
    Screen,
    Item(ItemType),
    Balloon,
    AccessCardSlot,
    GloveSlot,
    Key(KeyColor),
    Keyhole(KeyColor),
    Door(KeyColor),
    ShootableWall,
    Elevator,
    Acme,
    Fire(HorizontalDirection),
    Mill,
    ElectricArc,
    AccessCardDoor,
    Spikes(SpikeType),
    Score(ScoreType),
    BackgroundAnimation(BackgroundAnimationType),
}

impl ActorType {
    pub(crate) fn create_actor_boxed(
        &self,
        p: Point,
        sz: &dyn Sizes,
        t: &mut LevelTiles,
    ) -> Box<dyn Actor> {
        match self {
            ActorType::FireWheelBot => {
                FireWheelBot::create_boxed(p, sz, t)
            }
            ActorType::FlameGnomeBot => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::FlyingBot => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::FootBot => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::HelicopterBot => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::RabbitoidBot => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::MineJumping => MineJumping::create_boxed(p, sz, t),
            ActorType::MineLying => MineLying::create_boxed(p, sz, t),
            ActorType::Robot => Robot::create_boxed(p, sz, t),
            ActorType::SingleAnimation(animation_type) => {
                SingleAnimation::create_boxed_with_details(
                    *animation_type,
                    p,
                    sz,
                    t,
                )
            }
            ActorType::SnakeBot => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::TankBot => TankBot::create_boxed(p, sz, t),
            ActorType::WallCrawler(direction) => {
                WallCrawler::create_boxed_with_details(
                    *direction, p, sz, t,
                )
            }
            ActorType::DrProton => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::Camera => Camera::create_boxed(p, sz, t),
            ActorType::Particle(color) => {
                Particle::create_boxed_with_details(*color, p, sz, t)
            }
            ActorType::Rocket => Rocket::create_boxed(p, sz, t),
            ActorType::Bomb => Bomb::create_boxed(p, sz, t),
            ActorType::Water => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::ExitDoor => ExitDoor::create_boxed(p, sz, t),
            ActorType::NoteBook => NoteBook::create_boxed(p, sz, t),
            ActorType::SurveillanceScreen => {
                SurveillanceScreen::create_boxed(p, sz, t)
            }
            ActorType::HostileShot(direction) => {
                HostileShot::create_boxed_with_details(
                    *direction, p, sz, t,
                )
            }
            ActorType::SodaFlying => SodaFlying::create_boxed(p, sz, t),
            ActorType::UnstableFloor => {
                UnstableFloor::create_boxed(p, sz, t)
            }
            ActorType::ExpandingFloor => {
                ExpandingFloor::create_boxed(p, sz, t)
            }
            ActorType::ConveyorRightEnd(direction) => {
                Conveyor::create_boxed_with_details(*direction, p, sz, t)
            }
            ActorType::Fan(direction) => {
                Fan::create_boxed_with_details(*direction, p, sz, t)
            }
            ActorType::StoneBackground => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::Teleporter(index) => {
                Teleporter::create_boxed_with_details(*index, p, sz, t)
            }
            ActorType::FenceBackground => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::Screen => {
                PlaceHolder::create_boxed_with_details(*self, p, sz, t)
            }
            ActorType::Item(item_type) => {
                Item::create_boxed_with_details(*item_type, p, sz, t)
            }
            ActorType::Balloon => Balloon::create_boxed(p, sz, t),
            ActorType::AccessCardSlot => {
                AccessCardSlot::create_boxed(p, sz, t)
            }
            ActorType::GloveSlot => GloveSlot::create_boxed(p, sz, t),
            ActorType::Key(color) => {
                Key::create_boxed_with_details(*color, p, sz, t)
            }
            ActorType::Keyhole(color) => {
                KeyHole::create_boxed_with_details(*color, p, sz, t)
            }
            ActorType::Door(color) => {
                Door::create_boxed_with_details(*color, p, sz, t)
            }
            ActorType::ShootableWall => {
                ShootableWall::create_boxed(p, sz, t)
            }
            ActorType::Elevator => Elevator::create_boxed(p, sz, t),
            ActorType::Acme => Acme::create_boxed(p, sz, t),
            ActorType::Fire(direction) => {
                Fire::create_boxed_with_details(*direction, p, sz, t)
            }
            ActorType::Mill => Mill::create_boxed(p, sz, t),
            ActorType::ElectricArc => ElectricArc::create_boxed(p, sz, t),
            ActorType::AccessCardDoor => {
                AccessCardDoor::create_boxed(p, sz, t)
            }
            ActorType::Spikes(spike_type) => {
                Spikes::create_boxed_with_details(*spike_type, p, sz, t)
            }
            ActorType::Score(score_type) => {
                Score::create_boxed_with_details(*score_type, p, sz, t)
            }
            ActorType::BackgroundAnimation(animation_type) => {
                BackgroundAnimation::create_boxed_with_details(
                    *animation_type,
                    p,
                    sz,
                    t,
                )
            }
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
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let color = match rng.gen_range(0..4) {
                0 => ParticleColor::Pink,
                1 => ParticleColor::Blue,
                2 => ParticleColor::White,
                3 => ParticleColor::Green,
                _ => unreachable!(),
            };
            self.add_actor(ActorType::Particle(color), pos);
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
    pub sizes: &'a dyn Sizes,
    pub tiles: &'a mut LevelTiles,
    pub actors: &'a mut ActorsList,
    pub copy_background: bool,
}

impl<'a> ActorAdder for LevelActorAdder<'a> {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point) {
        let actor =
            actor_type.create_actor_boxed(pos, self.sizes, self.tiles);
        if self.copy_background {
            let x = pos.x / self.sizes.width() as i32;
            let y = pos.y / self.sizes.height() as i32;

            let effective_number = match actor.background_tile_strategy() {
                BackgroundTileStrategy::KeepEmpty => 0,
                BackgroundTileStrategy::SetTile(tile) => tile,
                BackgroundTileStrategy::CopyFromAbove => self
                    .tiles
                    .get(x, y - 1)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
                BackgroundTileStrategy::CopyFromLeft => self
                    .tiles
                    .get(x - 1, y)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
                BackgroundTileStrategy::CopyFromRight => self
                    .tiles
                    .get(x + 1, y)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
                BackgroundTileStrategy::CopyFromBelow => self
                    .tiles
                    .get(x, y + 1)
                    .map(|t| t.effective_number)
                    .unwrap_or(0),
            };

            if let Ok(t) = self.tiles.get_mut(x, y) {
                t.effective_number = effective_number;
            }
        }
        self.actors.actors.push(actor);
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ActorMessageType {
    OpenDoor(KeyColor),
    OpenDoorAccessCard,
    TeleportTo(TeleporterIndex),
    ExpandFloor,
    RemoveElectricArc,
}

#[derive(Default)]
pub struct ActorMessageQueue {
    pub messages: Vec<ActorMessageType>,
}

impl ActorMessageQueue {
    pub fn new() -> Self {
        ActorMessageQueue {
            messages: Vec::new(),
        }
    }

    pub fn push_back(&mut self, message: ActorMessageType) {
        self.messages.push(message);
    }
}

trait CreateActorWithDetails: Sized {
    type Details;

    fn create_with_details(
        details: Self::Details,
        position: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Self;

    fn create_boxed_with_details(
        details: Self::Details,
        position: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Box<Self> {
        Box::new(Self::create_with_details(
            details, position, sizes, tiles,
        ))
    }
}

trait CreateActor: Sized {
    fn create(
        position: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Self;

    fn create_boxed(
        position: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Box<Self> {
        Box::new(Self::create(position, sizes, tiles))
    }
}

pub struct ActParameters<'a> {
    pub tiles: &'a mut LevelTiles,
    pub hero: &'a mut Hero,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub play_state: &'a mut PlayState,
    pub sizes: &'a dyn Sizes,
}

pub struct ShotParameters<'a> {
    pub tiles: &'a mut LevelTiles,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub hero: &'a mut Hero,
    pub actor_message_queue: &'a mut ActorMessageQueue,
    pub sizes: &'a dyn Sizes,
}

pub struct RenderParameters<'a> {
    pub renderer: &'a mut dyn Renderer,
    pub sizes: &'a dyn Sizes,
}

pub struct ReceiveMessageParameters<'a> {
    pub message: ActorMessageType,
    pub hero: &'a mut Hero,
    pub tiles: &'a mut LevelTiles,
    pub sizes: &'a dyn Sizes,
}

pub struct HeroInteractStartParameters<'a> {
    pub play_state: &'a mut PlayState,
    pub hero: &'a mut Hero,
    pub info_message_queue: &'a mut InfoMessageQueue,
    pub actor_message_queue: &'a mut ActorMessageQueue,
}

pub struct HeroInteractEndParameters<'a> {
    pub hero: &'a mut Hero,
    pub play_state: &'a mut PlayState,
}

pub(crate) trait Actor: std::fmt::Debug {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        false
    }

    fn hero_interact_start(&mut self, _p: HeroInteractStartParameters) {}

    fn hero_interact_end(&mut self, _p: HeroInteractEndParameters) {}

    fn act(&mut self, p: ActParameters);

    fn render(&mut self, p: RenderParameters) -> Result<()>;

    fn can_get_shot(&self) -> bool {
        false
    }

    fn shot(&mut self, _p: ShotParameters) -> ShotProcessing {
        ShotProcessing::Ignore
    }

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

    fn can_receive_message(
        &self,
        _message_type: ActorMessageType,
    ) -> bool {
        false
    }

    fn receive_message(&mut self, _p: ReceiveMessageParameters) {}

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::KeepEmpty
    }
}
