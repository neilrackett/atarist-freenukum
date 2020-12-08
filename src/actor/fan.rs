use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters, ShotParameters,
    },
    geometry::RectExt,
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_FAN, HALFTILE_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};

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
        general.position.offset(0, -(TILE_HEIGHT as i32));
        general.position.resize(TILE_WIDTH, TILE_HEIGHT * 2);

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
        } else if self.running == 10
            && p.hero_data
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

            if (fan_direction * hdistance) < 0 {
                return;
            }

            if hdistance == 0 {
                hdistance = HALFTILE_WIDTH as i32 * fan_direction;
            }

            let range = HALFTILE_WIDTH as i32 * 8;
            if hdistance.abs() < range {
                p.hero_data.position.push_horizontally(
                    &p.solids,
                    fan_direction * TILE_WIDTH as i32,
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = p.general.position.top_left();
        p.renderer
            .place_tile(self.tile + self.current_frame * 2, pos)?;
        pos.y += TILE_HEIGHT as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 2 + 1, pos)?;
        Ok(())
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        self.running = 9;
        p.actor_adder
            .add_actor(ActorType::Steam, p.general.position.top_left());
    }
}
