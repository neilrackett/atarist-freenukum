use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Hero, HorizontalDirection, Result, Sizes, ANIMATION_ROBOT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Robot {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for Robot {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::Robot(Self {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_ROBOT,
            current_frame: 0,
            num_frames: 3,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
        })
    }
}

impl ActorExt for Robot {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if p.tiles
            .get(
                self.position.x() / p.sizes.width() as i32,
                self.position.y() / p.sizes.height() as i32 + 1,
            )
            .map(|t| !t.solid)
            .unwrap_or(false)
        {
            // In the air, falling down.
            self.position.offset(0, p.sizes.half_height() as i32);
        } else {
            // On the floor, walking.
            if self.current_frame == 0 {
                let mut direction = match self.direction {
                    HorizontalDirection::Left => -1,
                    HorizontalDirection::Right => 2,
                };
                // Check if the place next to the bot is free
                if p.tiles.get(
                (
                    self.position.x() +
                    direction * p.sizes.half_width() as i32
                ) / p.sizes.width() as i32,
                self.position.y() / p.sizes.height() as i32
            ).map(|t|!t.solid).unwrap_or(false) &&
            // Check if the tile below this free place is solid
            p.tiles.get(
                (
                    self.position.x() +
                    direction * p.sizes.half_width() as i32
                ) / p.sizes.width() as i32,
                (self.position.y() + p.sizes.height() as i32) / p.sizes.height() as i32
            ).map(|t|t.solid).unwrap_or(false)
                {
                    if direction == 2 {
                        direction = 1;
                    }
                    self.position.offset(
                        direction * p.sizes.half_width() as i32,
                        0,
                    );
                } else {
                    self.direction.reverse();
                    if direction == 2 {
                        direction = 1
                    };
                    direction *= -1;
                    self.position.offset(
                        direction * p.sizes.half_width() as i32,
                        0,
                    );
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.hero.score.add(100);
        p.game_commands.add_actor(
            ActorType::SingleAnimation(
                SingleAnimationType::RobotDisappearing,
            ),
            self.position.top_left(),
        );
        p.game_commands.add_sound(SoundIndex::SMALLDEATH);
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

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromLeft
    }
}
