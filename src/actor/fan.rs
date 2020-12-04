use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, RenderParameters, ShotParameters,
};
use crate::geometry::RectExt;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{ANIMATION_FAN, HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    running: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.y -= TILE_HEIGHT as i16;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16 * 2;

        Specific {
            tile: ANIMATION_FAN,
            current_frame: 0,
            num_frames: 4,
            running: 10,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        match self.running {
            0 => {}
            1 => {
                self.current_frame += 1;
            }
            2 => {}
            3 => {}
            4 => {}
            5 => {
                self.current_frame += 1;
            }
            6 => {}
            7 => {}
            8 => {
                self.current_frame += 1;
            }
            9 => {}
            10 => {
                self.current_frame += 1;
            }
            _ => unreachable!(),
        }
        self.current_frame %= self.num_frames;
        if self.running < 10 && self.running > 0 {
            self.running -= 1;
        } else if self.running == 10 {
            if p.hero_data
                .position
                .geometry
                .overlaps_vertically(p.general.position)
            {
                let mut hdistance = p
                    .hero_data
                    .position
                    .geometry
                    .horizontal_distance(p.general.position);

                let fan_direction = match p.general.actor_type {
                    ActorType::FanLeft => -1,
                    ActorType::FanRight => 1,
                    _ => unreachable!(),
                };
                if (fan_direction > 0 && hdistance > 0)
                    || (fan_direction < 0 && hdistance < 0)
                {
                    return;
                }

                if hdistance == 0 {
                    hdistance = HALFTILE_WIDTH as i32 * fan_direction;
                }

                if hdistance.abs() < 8 * HALFTILE_WIDTH as i32 {
                    p.hero_data.position.push_horizontally(
                        &p.solids,
                        fan_direction as i16 * TILE_WIDTH as i16,
                    );
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let mut destrect = p.general.position;
        p.renderer
            .place_tile(self.tile + self.current_frame * 2, destrect);
        destrect.y += TILE_HEIGHT as i16;
        p.renderer
            .place_tile(self.tile + self.current_frame * 2 + 1, destrect);
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        self.running = 9;
        p.actor_adder.add_actor(
            ActorType::Steam,
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
    }
}
