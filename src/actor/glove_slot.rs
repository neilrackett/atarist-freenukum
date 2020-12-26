use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ActorType, HeroInteractStartParameters,
        RenderParameters,
    },
    hero::InventoryItem,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, OBJECT_GLOVE_SLOT, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
enum State {
    Idle,
    Shooting,
    Expanded,
}

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    state: State,
    countdown: usize,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        _general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            tile: OBJECT_GLOVE_SLOT,
            current_frame: 0,
            num_frames: 4,
            state: State::Idle,
            countdown: 0,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self, _hero: &Hero) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        match self.state {
            State::Idle => {
                if p.hero.inventory.is_set(InventoryItem::Glove) {
                    p.actor_message_queue.push_back(
                        ActorType::ExpandingFloor,
                        ActorMessageType::Expand,
                    );
                    self.state = State::Expanded;
                } else {
                    self.state = State::Shooting;
                    self.countdown = 20;
                }
            }
            State::Shooting => {}
            State::Expanded => {}
        }
    }

    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Idle => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
            }
            State::Shooting => {
                self.current_frame += 1;
                self.current_frame %= self.num_frames;
                self.countdown -= 1;
                if self.countdown % 4 == 0 {
                    p.actor_adder.add_actor(
                        ActorType::HostileShotRight,
                        self.position.top_left(),
                    );
                } else if self.countdown % 4 == 2 {
                    p.actor_adder.add_actor(
                        ActorType::HostileShotLeft,
                        self.position.top_left(),
                    );
                }
                if self.countdown == 0 {
                    self.state = State::Idle;
                }
            }
            State::Expanded => {}
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let adder = if self.current_frame == 0 { 0 } else { 1 };
        let mut pos = self.position.top_left();
        p.renderer.place_tile(self.tile + adder, pos)?;

        pos.x -= TILE_WIDTH as i32;
        p.renderer.place_tile(self.tile + 2, pos)?;

        pos.x += 2 * TILE_WIDTH as i32;
        p.renderer.place_tile(self.tile + 3, pos)?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }
}
