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
    HorizontalDirection, KeyColor, Result, Sizes,
};
pub(crate) use item::{
    BoxBlueContent, BoxGreyContent, BoxRedContent, ItemType,
};
use particle::ParticleColor;
use score::ScoreType;
use sdl2::rect::{Point, Rect};
pub(crate) use simpleanimation::SimpleAnimationType;
pub(crate) use singleanimation::SingleAnimationType;
pub(crate) use spikes::SpikeType;
pub(crate) use teleporter::TeleporterIndex;

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
        solids: &mut LevelSolids,
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
                solids,
            };
            actor.receive_message(p);
        }
    }

    pub fn process_shot(
        &mut self,
        shot_position: Rect,
        sizes: &dyn Sizes,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero: &mut Hero,
        actor_message_queue: &mut ActorMessageQueue,
    ) -> bool {
        for actor in self.actors.iter_mut() {
            let p = ShotParameters {
                sizes,
                solids,
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
                let p = ActParameters {
                    sizes,
                    solids,
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
            solids,
            sizes,
            tiles,
            actors: self,
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
    RedBallJumping,
    RedBallLying,
    Robot,
    SingleAnimation(SingleAnimationType),
    SnakeBot,
    TankBot,
    WallCrawlerBot(HorizontalDirection),
    DrProton,
    Camera,
    Particle(ParticleColor),
    Rocket,
    Bomb,
    Water,
    ExitDoor,
    Notebook,
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
    Lift,
    Acme,
    Fire(HorizontalDirection),
    Mill,
    ElectricArc,
    AccessCardDoor,
    Spikes(SpikeType),
    Score(ScoreType),
    SimpleAnimation(SimpleAnimationType),
}

impl ActorType {
    pub(crate) fn create_actor_boxed(
        &self,
        p: Point,
        sz: &dyn Sizes,
        s: &mut LevelSolids,
        t: &mut LevelTiles,
    ) -> Box<dyn Actor> {
        match self {
            ActorType::FireWheelBot => {
                firewheelbot::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::FlameGnomeBot => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::FlyingBot => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::FootBot => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::HelicopterBot => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::RabbitoidBot => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::RedBallJumping => {
                redball_jumping::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::RedBallLying => {
                redball_lying::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Robot => robot::Specific::create_boxed(p, sz, s, t),
            ActorType::SingleAnimation(animation_type) => {
                singleanimation::Specific::create_boxed_with_details(
                    *animation_type,
                    p,
                    sz,
                    s,
                    t,
                )
            }
            ActorType::SnakeBot => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::TankBot => {
                tankbot::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::WallCrawlerBot(direction) => {
                wallcrawler::Specific::create_boxed_with_details(
                    *direction, p, sz, s, t,
                )
            }
            ActorType::DrProton => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::Camera => {
                camera::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Particle(color) => {
                particle::Specific::create_boxed_with_details(
                    *color, p, sz, s, t,
                )
            }
            ActorType::Rocket => {
                rocket::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Bomb => bomb::Specific::create_boxed(p, sz, s, t),
            ActorType::Water => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::ExitDoor => {
                exitdoor::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Notebook => {
                notebook::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::SurveillanceScreen => {
                surveillancescreen::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::HostileShot(direction) => {
                hostileshot::Specific::create_boxed_with_details(
                    *direction, p, sz, s, t,
                )
            }
            ActorType::SodaFlying => {
                soda_flying::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::UnstableFloor => {
                unstablefloor::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::ExpandingFloor => {
                expandingfloor::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::ConveyorRightEnd(direction) => {
                conveyor::Specific::create_boxed_with_details(
                    *direction, p, sz, s, t,
                )
            }
            ActorType::Fan(direction) => {
                fan::Specific::create_boxed_with_details(
                    *direction, p, sz, s, t,
                )
            }
            ActorType::StoneBackground => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::Teleporter(index) => {
                teleporter::Specific::create_boxed_with_details(
                    *index, p, sz, s, t,
                )
            }
            ActorType::FenceBackground => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::Screen => {
                placeholder::Specific::create_boxed_with_details(
                    *self, p, sz, s, t,
                )
            }
            ActorType::Item(item_type) => {
                item::Specific::create_boxed_with_details(
                    *item_type, p, sz, s, t,
                )
            }
            ActorType::Balloon => {
                balloon::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::AccessCardSlot => {
                accesscard_slot::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::GloveSlot => {
                glove_slot::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Key(color) => {
                key::Specific::create_boxed_with_details(
                    *color, p, sz, s, t,
                )
            }
            ActorType::Keyhole(color) => {
                keyhole::Specific::create_boxed_with_details(
                    *color, p, sz, s, t,
                )
            }
            ActorType::Door(color) => {
                door::Specific::create_boxed_with_details(
                    *color, p, sz, s, t,
                )
            }
            ActorType::ShootableWall => {
                shootable_wall::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Lift => {
                elevator::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Acme => acme::Specific::create_boxed(p, sz, s, t),
            ActorType::Fire(direction) => {
                fire::Specific::create_boxed_with_details(
                    *direction, p, sz, s, t,
                )
            }
            ActorType::Mill => mill::Specific::create_boxed(p, sz, s, t),
            ActorType::ElectricArc => {
                electric_arc::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::AccessCardDoor => {
                accesscard_door::Specific::create_boxed(p, sz, s, t)
            }
            ActorType::Spikes(spike_type) => {
                spikes::Specific::create_boxed_with_details(
                    *spike_type,
                    p,
                    sz,
                    s,
                    t,
                )
            }
            ActorType::Score(score_type) => {
                score::Specific::create_boxed_with_details(
                    *score_type,
                    p,
                    sz,
                    s,
                    t,
                )
            }
            ActorType::SimpleAnimation(animation_type) => {
                simpleanimation::Specific::create_boxed_with_details(
                    *animation_type,
                    p,
                    sz,
                    s,
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
    pub solids: &'a mut LevelSolids,
    pub sizes: &'a dyn Sizes,
    pub tiles: &'a mut LevelTiles,
    pub actors: &'a mut ActorsList,
}
impl<'a> ActorAdder for LevelActorAdder<'a> {
    fn add_actor(&mut self, actor_type: ActorType, pos: Point) {
        let actor = actor_type.create_actor_boxed(
            pos,
            self.sizes,
            self.solids,
            self.tiles,
        );
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
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self;

    fn create_boxed_with_details(
        details: Self::Details,
        position: Point,
        sizes: &dyn Sizes,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Box<Self> {
        Box::new(Self::create_with_details(
            details, position, sizes, solids, tiles,
        ))
    }
}

trait CreateActor: Sized {
    fn create(
        position: Point,
        sizes: &dyn Sizes,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Self;

    fn create_boxed(
        position: Point,
        sizes: &dyn Sizes,
        solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Box<Self> {
        Box::new(Self::create(position, sizes, solids, tiles))
    }
}

pub struct ActParameters<'a> {
    pub solids: &'a mut LevelSolids,
    pub tiles: &'a mut LevelTiles,
    pub hero: &'a mut Hero,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub play_state: &'a mut PlayState,
    pub sizes: &'a dyn Sizes,
}

pub struct ShotParameters<'a> {
    pub solids: &'a mut LevelSolids,
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
    pub solids: &'a mut LevelSolids,
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
}
