use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
    RenderParameters, ShotParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    HorizontalDirection, ANIMATION_ROBOT, HALFTILE_HEIGHT, HALFTILE_WIDTH,
    TILE_HEIGHT, TILE_WIDTH,
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
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
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
            p.general.position.x as usize / TILE_WIDTH,
            p.general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            // In the air, falling down.
            p.general.position.y += HALFTILE_HEIGHT as i16;
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
                    p.general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize/ TILE_WIDTH,
                p.general.position.y as usize / TILE_HEIGHT
            ) &&
            // Check if the tile below this free place is solid
            p.solids.get(
                (
                    p.general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (p.general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT
            ) {
                    if direction == 2 {
                        direction = 1;
                    }
                    p.general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
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
                    p.general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) {
        p.renderer.place_tile(self.tile, p.general.position);
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
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
        p.general.is_alive = false;
    }
}
