use crate::{
    actor::{
        ActParameters, ActorInterface, ActorType, CreateActor,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, HorizontalDirection, Result, ANIMATION_ROBOT, HALFTILE_HEIGHT,
    HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_ROBOT,
            current_frame: 0,
            num_frames: 3,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            is_alive: true,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if !p.solids.get(
            self.position.x() as u32 / TILE_WIDTH,
            self.position.y() as u32 / TILE_HEIGHT + 1,
        ) {
            // In the air, falling down.
            self.position.offset(0, HALFTILE_HEIGHT as i32);
        } else {
            // On the floor, walking.
            if self.current_frame == 0 {
                let mut direction = match self.direction {
                    HorizontalDirection::Left => -1,
                    HorizontalDirection::Right => 2,
                };
                // Check if the place next to the bot is free
                if !p.solids.get(
                (
                    self.position.x() +
                    direction * HALFTILE_WIDTH as i32
                ) as u32/ TILE_WIDTH,
                self.position.y() as u32 / TILE_HEIGHT
            ) &&
            // Check if the tile below this free place is solid
            p.solids.get(
                (
                    self.position.x() +
                    direction * HALFTILE_WIDTH as i32
                ) as u32 / TILE_WIDTH,
                (self.position.y() as u32 + TILE_HEIGHT) / TILE_HEIGHT
            ) {
                    if direction == 2 {
                        direction = 1;
                    }
                    self.position
                        .offset(direction * HALFTILE_WIDTH as i32, 0);
                } else {
                    self.direction.reverse();
                    if direction == 2 {
                        direction = 1
                    };
                    direction *= -1;
                    self.position
                        .offset(direction * HALFTILE_WIDTH as i32, 0);
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
        p.actor_adder.add_actor(
            ActorType::SingleAnimation(
                SingleAnimationType::RobotDisappearing,
            ),
            self.position.top_left(),
        );
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
}
