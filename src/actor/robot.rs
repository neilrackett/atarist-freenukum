use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
        RenderParameters, ShotParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    HorizontalDirection, Result, ANIMATION_ROBOT, HALFTILE_HEIGHT,
    HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    touching_hero: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = true;

        Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_ROBOT,
            current_frame: 0,
            num_frames: 3,
            touching_hero: false,
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
            p.general.position.x() as u32 / TILE_WIDTH,
            p.general.position.y() as u32 / TILE_HEIGHT + 1,
        ) {
            // In the air, falling down.
            p.general.position.offset(0, HALFTILE_HEIGHT as i32);
        } else {
            // On the floor, walking.
            if self.current_frame == 0 {
                let mut direction = match self.direction {
                    HorizontalDirection::Left => -1,
                    HorizontalDirection::Right => 2,
                    HorizontalDirection::Center => unreachable!(),
                };
                // Check if the place next to the bot is free
                if !p.solids.get(
                (
                    p.general.position.x() +
                    direction * HALFTILE_WIDTH as i32
                ) as u32/ TILE_WIDTH,
                p.general.position.y() as u32 / TILE_HEIGHT
            ) &&
            // Check if the tile below this free place is solid
            p.solids.get(
                (
                    p.general.position.x() +
                    direction * HALFTILE_WIDTH as i32
                ) as u32 / TILE_WIDTH,
                (p.general.position.y() as u32 + TILE_HEIGHT) / TILE_HEIGHT
            ) {
                    if direction == 2 {
                        direction = 1;
                    }
                    p.general
                        .position
                        .offset(direction * HALFTILE_WIDTH as i32, 0);
                } else {
                    self.direction =
                        if self.direction == HorizontalDirection::Left {
                            HorizontalDirection::Right
                        } else {
                            HorizontalDirection::Left
                        };
                    if direction == 2 {
                        direction = 1
                    };
                    direction *= -1;
                    p.general
                        .position
                        .offset(direction * HALFTILE_WIDTH as i32, 0);
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer
            .place_tile(self.tile, p.general.position.top_left())?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        p.hero_data.score.add(100);
        if self.touching_hero {
            p.general.hurts_hero = false;
            self.touching_hero = false;
        }
        p.actor_adder.add_actor(
            ActorType::RobotDisappearing,
            p.general.position.top_left(),
        );
        p.general.is_alive = false;
    }
}
