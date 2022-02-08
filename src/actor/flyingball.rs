use crate::{
    actor::{
        ActParameters, Actor, ActorExt, CreateActorWithDetails,
        RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Hero, Result, Sizes, ANIMATION_BALL,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct FlyingBall {
    base_position: Rect,
    tile: usize,
    num_frames: usize,
    counter: usize,
    position_counter: usize,
    position: Rect,
    is_alive: bool,
}

const POSITION_OFFSETS: [(i32, i32); 40] = [
    // resting
    (128, 0),
    (128, 0),
    (128, 0),
    (128, 0),
    (128, 0),
    (128, 0),
    (128, 0),
    (128, 0),
    (128, 0),
    // moving to bottom left
    (128, 8),
    (120, 16),
    (112, 24),
    (96, 32),
    (80, 32),
    (64, 24),
    (56, 16),
    (48, 8),
    // moving to top left
    (48, 0),
    (56, -8),
    (64, -16),
    (80, -24),
    (96, -24),
    (112, -16),
    (120, -8),
    (128, 0),
    // moving to bottom right
    (128, 8),
    (136, 16),
    (144, 24),
    (160, 32),
    (176, 32),
    (192, 24),
    (200, 16),
    (208, 8),
    // moving to top right
    (208, 0),
    (200, -8),
    (192, -16),
    (176, -24),
    (160, -24),
    (144, -16),
    (136, -8),
];

impl CreateActorWithDetails for FlyingBall {
    type Details = usize;

    fn create_with_details(
        start_index: usize,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        let num_frames = 8;
        let counter = start_index % num_frames;
        let position_counter = start_index % POSITION_OFFSETS.len();
        let position = POSITION_OFFSETS[position_counter];
        Actor::FlyingBall(Self {
            base_position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            tile: ANIMATION_BALL,
            num_frames,
            counter,
            position_counter,
            position: Rect::new(
                pos.x + position.0,
                pos.y + position.1,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
        })
    }
}

impl ActorExt for FlyingBall {
    fn act(&mut self, _p: ActParameters) {
        let relative_position = POSITION_OFFSETS[self.position_counter];
        self.position
            .set_x(self.base_position.x + relative_position.0);
        self.position
            .set_y(self.base_position.y + relative_position.1);

        self.counter += 1;
        self.counter %= self.num_frames * 3;

        self.position_counter += 1;
        self.position_counter %= POSITION_OFFSETS.len();
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.counter / 3,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.hero.score.add(1000);
        p.actor_adder
            .add_particle_firework(self.position.top_left(), 4);
        // TODO: add explosion animation (small and then large white circle)
        self.is_alive = false;
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.position.has_intersection(hero.position.geometry)
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}
