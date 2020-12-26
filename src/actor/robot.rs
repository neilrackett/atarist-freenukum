use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
        RenderParameters, ShotParameters, ShotProcessing,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    HorizontalDirection, Result, ANIMATION_ROBOT, HALFTILE_HEIGHT,
    HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    touching_hero: bool,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.is_in_foreground = true;

        Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_ROBOT,
            current_frame: 0,
            num_frames: 3,
            touching_hero: false,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.general.hurts_hero = true;
        self.touching_hero = true;
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        p.general.hurts_hero = false;
        self.touching_hero = false;
    }

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

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.hero.score.add(100);
        if self.touching_hero {
            p.general.hurts_hero = false;
            self.touching_hero = false;
        }
        p.actor_adder.add_actor(
            ActorType::RobotDisappearing,
            self.position.top_left(),
        );
        p.general.is_alive = false;
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }
}
