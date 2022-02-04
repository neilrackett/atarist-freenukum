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
pub(crate) enum Actor {
    FireWheelBot(FireWheelBot),
    PlaceHolder(PlaceHolder),
    MineJumping(MineJumping),
    MineLying(MineLying),
    Robot(Robot),
    SingleAnimation(SingleAnimation),
    TankBot(TankBot),
    WallCrawler(WallCrawler),
    Camera(Camera),
    Particle(Particle),
    Rocket(Rocket),
    Bomb(Bomb),
    ExitDoor(ExitDoor),
    NoteBook(NoteBook),
    SurveillanceScreen(SurveillanceScreen),
    HostileShot(HostileShot),
    SodaFlying(SodaFlying),
    UnstableFloor(UnstableFloor),
    ExpandingFloor(ExpandingFloor),
    Conveyor(Conveyor),
    Fan(Fan),
    Teleporter(Teleporter),
    Item(Item),
    Balloon(Balloon),
    AccessCardSlot(AccessCardSlot),
    GloveSlot(GloveSlot),
    Key(Key),
    KeyHole(KeyHole),
    Door(Door),
    ShootableWall(ShootableWall),
    Elevator(Elevator),
    Acme(Acme),
    Fire(Fire),
    Mill(Mill),
    ElectricArc(ElectricArc),
    AccessCardDoor(AccessCardDoor),
    Spikes(Spikes),
    Score(Score),
    BackgroundAnimation(BackgroundAnimation),
}

impl Actor {
    fn as_actor_ext_mut(&mut self) -> &mut dyn ActorExt {
        match self {
            Actor::FireWheelBot(ref mut a) => a,
            Actor::PlaceHolder(ref mut a) => a,
            Actor::MineJumping(ref mut a) => a,
            Actor::MineLying(ref mut a) => a,
            Actor::Robot(ref mut a) => a,
            Actor::SingleAnimation(ref mut a) => a,
            Actor::TankBot(ref mut a) => a,
            Actor::WallCrawler(ref mut a) => a,
            Actor::Camera(ref mut a) => a,
            Actor::Particle(ref mut a) => a,
            Actor::Rocket(ref mut a) => a,
            Actor::Bomb(ref mut a) => a,
            Actor::ExitDoor(ref mut a) => a,
            Actor::NoteBook(ref mut a) => a,
            Actor::SurveillanceScreen(ref mut a) => a,
            Actor::HostileShot(ref mut a) => a,
            Actor::SodaFlying(ref mut a) => a,
            Actor::UnstableFloor(ref mut a) => a,
            Actor::ExpandingFloor(ref mut a) => a,
            Actor::Conveyor(ref mut a) => a,
            Actor::Fan(ref mut a) => a,
            Actor::Teleporter(ref mut a) => a,
            Actor::Item(ref mut a) => a,
            Actor::Balloon(ref mut a) => a,
            Actor::AccessCardSlot(ref mut a) => a,
            Actor::GloveSlot(ref mut a) => a,
            Actor::Key(ref mut a) => a,
            Actor::KeyHole(ref mut a) => a,
            Actor::Door(ref mut a) => a,
            Actor::ShootableWall(ref mut a) => a,
            Actor::Elevator(ref mut a) => a,
            Actor::Acme(ref mut a) => a,
            Actor::Fire(ref mut a) => a,
            Actor::Mill(ref mut a) => a,
            Actor::ElectricArc(ref mut a) => a,
            Actor::AccessCardDoor(ref mut a) => a,
            Actor::Spikes(ref mut a) => a,
            Actor::Score(ref mut a) => a,
            Actor::BackgroundAnimation(ref mut a) => a,
        }
    }

    fn as_actor_ext(&self) -> &dyn ActorExt {
        match self {
            Actor::FireWheelBot(ref a) => a,
            Actor::PlaceHolder(ref a) => a,
            Actor::MineJumping(ref a) => a,
            Actor::MineLying(ref a) => a,
            Actor::Robot(ref a) => a,
            Actor::SingleAnimation(ref a) => a,
            Actor::TankBot(ref a) => a,
            Actor::WallCrawler(ref a) => a,
            Actor::Camera(ref a) => a,
            Actor::Particle(ref a) => a,
            Actor::Rocket(ref a) => a,
            Actor::Bomb(ref a) => a,
            Actor::ExitDoor(ref a) => a,
            Actor::NoteBook(ref a) => a,
            Actor::SurveillanceScreen(ref a) => a,
            Actor::HostileShot(ref a) => a,
            Actor::SodaFlying(ref a) => a,
            Actor::UnstableFloor(ref a) => a,
            Actor::ExpandingFloor(ref a) => a,
            Actor::Conveyor(ref a) => a,
            Actor::Fan(ref a) => a,
            Actor::Teleporter(ref a) => a,
            Actor::Item(ref a) => a,
            Actor::Balloon(ref a) => a,
            Actor::AccessCardSlot(ref a) => a,
            Actor::GloveSlot(ref a) => a,
            Actor::Key(ref a) => a,
            Actor::KeyHole(ref a) => a,
            Actor::Door(ref a) => a,
            Actor::ShootableWall(ref a) => a,
            Actor::Elevator(ref a) => a,
            Actor::Acme(ref a) => a,
            Actor::Fire(ref a) => a,
            Actor::Mill(ref a) => a,
            Actor::ElectricArc(ref a) => a,
            Actor::AccessCardDoor(ref a) => a,
            Actor::Spikes(ref a) => a,
            Actor::Score(ref a) => a,
            Actor::BackgroundAnimation(ref a) => a,
        }
    }
}

impl ActorExt for Actor {
    fn hero_can_interact(&self, hero: &Hero) -> bool {
        self.as_actor_ext().hero_can_interact(hero)
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        self.as_actor_ext_mut().hero_interact_start(p)
    }

    fn hero_interact_end(&mut self, p: HeroInteractEndParameters) {
        self.as_actor_ext_mut().hero_interact_end(p)
    }

    fn act(&mut self, p: ActParameters) {
        self.as_actor_ext_mut().act(p)
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        self.as_actor_ext_mut().render(p)
    }

    fn can_get_shot(&self) -> bool {
        self.as_actor_ext().can_get_shot()
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.as_actor_ext_mut().shot(p)
    }

    fn position(&self) -> Rect {
        self.as_actor_ext().position()
    }

    fn is_in_foreground(&self) -> bool {
        self.as_actor_ext().is_in_foreground()
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.as_actor_ext().hurts_hero(hero)
    }

    fn is_alive(&self) -> bool {
        self.as_actor_ext().is_alive()
    }

    fn acts_while_invisible(&self) -> bool {
        self.as_actor_ext().acts_while_invisible()
    }

    fn can_receive_message(&self, message_type: ActorMessageType) -> bool {
        self.as_actor_ext().can_receive_message(message_type)
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        self.as_actor_ext_mut().receive_message(p)
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        self.as_actor_ext().background_tile_strategy()
    }
}

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
        actor_message_queue: &mut ActorMessageQueue,
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
                    actor_message_queue,
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
    ) -> Actor {
        match self {
            ActorType::FireWheelBot => FireWheelBot::create(p, sz, t),
            ActorType::FlameGnomeBot => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::FlyingBot => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::FootBot => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::HelicopterBot => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::RabbitoidBot => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::MineJumping => MineJumping::create(p, sz, t),
            ActorType::MineLying => MineLying::create(p, sz, t),
            ActorType::Robot => Robot::create(p, sz, t),
            ActorType::SingleAnimation(animation_type) => {
                SingleAnimation::create_with_details(
                    *animation_type,
                    p,
                    sz,
                    t,
                )
            }
            ActorType::SnakeBot => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::TankBot => TankBot::create(p, sz, t),
            ActorType::WallCrawler(direction) => {
                WallCrawler::create_with_details(*direction, p, sz, t)
            }
            ActorType::DrProton => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::Camera => Camera::create(p, sz, t),
            ActorType::Particle(color) => {
                Particle::create_with_details(*color, p, sz, t)
            }
            ActorType::Rocket => Rocket::create(p, sz, t),
            ActorType::Bomb => Bomb::create(p, sz, t),
            ActorType::Water => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::ExitDoor => ExitDoor::create(p, sz, t),
            ActorType::NoteBook => NoteBook::create(p, sz, t),
            ActorType::SurveillanceScreen => {
                SurveillanceScreen::create(p, sz, t)
            }
            ActorType::HostileShot(direction) => {
                HostileShot::create_with_details(*direction, p, sz, t)
            }
            ActorType::SodaFlying => SodaFlying::create(p, sz, t),
            ActorType::UnstableFloor => UnstableFloor::create(p, sz, t),
            ActorType::ExpandingFloor => ExpandingFloor::create(p, sz, t),
            ActorType::ConveyorRightEnd(direction) => {
                Conveyor::create_with_details(*direction, p, sz, t)
            }
            ActorType::Fan(direction) => {
                Fan::create_with_details(*direction, p, sz, t)
            }
            ActorType::StoneBackground => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::Teleporter(index) => {
                Teleporter::create_with_details(*index, p, sz, t)
            }
            ActorType::FenceBackground => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::Screen => {
                PlaceHolder::create_with_details(*self, p, sz, t)
            }
            ActorType::Item(item_type) => {
                Item::create_with_details(*item_type, p, sz, t)
            }
            ActorType::Balloon => Balloon::create(p, sz, t),
            ActorType::AccessCardSlot => AccessCardSlot::create(p, sz, t),
            ActorType::GloveSlot => GloveSlot::create(p, sz, t),
            ActorType::Key(color) => {
                Key::create_with_details(*color, p, sz, t)
            }
            ActorType::Keyhole(color) => {
                KeyHole::create_with_details(*color, p, sz, t)
            }
            ActorType::Door(color) => {
                Door::create_with_details(*color, p, sz, t)
            }
            ActorType::ShootableWall => ShootableWall::create(p, sz, t),
            ActorType::Elevator => Elevator::create(p, sz, t),
            ActorType::Acme => Acme::create(p, sz, t),
            ActorType::Fire(direction) => {
                Fire::create_with_details(*direction, p, sz, t)
            }
            ActorType::Mill => Mill::create(p, sz, t),
            ActorType::ElectricArc => ElectricArc::create(p, sz, t),
            ActorType::AccessCardDoor => AccessCardDoor::create(p, sz, t),
            ActorType::Spikes(spike_type) => {
                Spikes::create_with_details(*spike_type, p, sz, t)
            }
            ActorType::Score(score_type) => {
                Score::create_with_details(*score_type, p, sz, t)
            }
            ActorType::BackgroundAnimation(animation_type) => {
                BackgroundAnimation::create_with_details(
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
    ) -> Actor;
}

trait CreateActor: Sized {
    fn create(
        position: Point,
        sizes: &dyn Sizes,
        tiles: &mut LevelTiles,
    ) -> Actor;
}

pub struct ActParameters<'a> {
    pub tiles: &'a mut LevelTiles,
    pub hero: &'a mut Hero,
    pub actor_adder: &'a mut dyn ActorAdder,
    pub actor_message_queue: &'a mut ActorMessageQueue,
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

pub(crate) trait ActorExt: std::fmt::Debug {
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
