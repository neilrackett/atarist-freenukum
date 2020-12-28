use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActorWithDetails,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    geometry::RectExt,
    level::{
        solids::LevelSolids, tiles::LevelTiles, BackgroundTileStrategy,
    },
    HorizontalDirection, Result, Sizes, ANIMATION_FAN,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Fan {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    running: usize,
    position: Rect,
    direction: HorizontalDirection,
}

impl CreateActorWithDetails for Fan {
    type Details = HorizontalDirection;

    fn create_with_details(
        direction: HorizontalDirection,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        Self {
            tile: ANIMATION_FAN,
            current_frame: 0,
            num_frames: 4,
            running: 10,
            position: Rect::new(
                pos.x,
                pos.y - sizes.height() as i32,
                sizes.width(),
                sizes.height() * 2,
            ),
            direction,
        }
    }
}

impl Actor for Fan {
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
            && p.hero.position.geometry.overlaps_vertically(self.position)
        {
            let mut hdistance = p
                .hero
                .position
                .geometry
                .horizontal_distance(self.position);

            let fan_direction = self.direction.as_factor_i32();

            if (fan_direction * hdistance) < 0 {
                return;
            }

            if hdistance == 0 {
                hdistance = p.sizes.half_width() as i32 * fan_direction;
            }

            let range = p.sizes.half_width() as i32 * 8;
            if hdistance.abs() < range {
                p.hero.position.push_horizontally(
                    p.sizes,
                    &p.solids,
                    fan_direction * p.sizes.width() as i32,
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();
        p.renderer
            .place_tile(self.tile + self.current_frame * 2, pos)?;
        pos.y += p.sizes.height() as i32;
        p.renderer
            .place_tile(self.tile + self.current_frame * 2 + 1, pos)?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.running = 9;
        p.actor_adder.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Steam),
            self.position.top_left(),
        );
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromAbove
    }
}
