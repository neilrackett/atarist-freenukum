use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ActorType, HeroInteractStartParameters,
        RenderParameters,
    },
    hero::InventoryItem,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_GLOVE_SLOT, TILE_HEIGHT, TILE_WIDTH,
};

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
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = false;

        Specific {
            tile: OBJECT_GLOVE_SLOT,
            current_frame: 0,
            num_frames: 4,
            state: State::Idle,
            countdown: 0,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(&mut self, p: HeroInteractStartParameters) {
        match self.state {
            State::Idle => {
                if p.hero_data.inventory.is_set(InventoryItem::Glove) {
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
                        p.general.position.top_left(),
                    );
                } else if self.countdown % 4 == 2 {
                    p.actor_adder.add_actor(
                        ActorType::HostileShotLeft,
                        p.general.position.top_left(),
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
        let mut pos = p.general.position.top_left();
        p.renderer.place_tile(self.tile + adder, pos)?;

        pos.x -= TILE_WIDTH as i32;
        p.renderer.place_tile(self.tile + 2, pos)?;

        pos.x += 2 * TILE_WIDTH as i32;
        p.renderer.place_tile(self.tile + 3, pos)?;
        Ok(())
    }
}
