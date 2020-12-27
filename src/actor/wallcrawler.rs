use crate::{
    actor::{
        ActParameters, Actor, ActorType, CreateActorWithDetails,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, HorizontalDirection, Result, VerticalDirection,
    ANIMATION_WALLCRAWLERBOT_LEFT, ANIMATION_WALLCRAWLERBOT_RIGHT,
    LEVEL_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    direction: VerticalDirection,
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    is_alive: bool,
    position: Rect,
}

impl CreateActorWithDetails for Specific {
    type Details = HorizontalDirection;

    fn create_with_details(
        orientation: HorizontalDirection,
        pos: Point,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Specific {
        let x = pos.x as u32 / TILE_WIDTH;
        let y = pos.y as u32 / TILE_HEIGHT;

        let tile = match orientation {
            HorizontalDirection::Left => {
                if x < LEVEL_WIDTH + 1 {
                    tiles.copy_from_to(x + 1, y, x, y);
                }
                ANIMATION_WALLCRAWLERBOT_LEFT
            }
            HorizontalDirection::Right => {
                if x > 0 {
                    tiles.copy_from_to(x - 1, y, x, y);
                }
                ANIMATION_WALLCRAWLERBOT_RIGHT
            }
        };

        Specific {
            direction: VerticalDirection::Up,
            orientation,
            tile,
            current_frame: 0,
            num_frames: 4,
            is_alive: true,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, p: ActParameters) {
        let orientation = self.orientation.as_factor_i32();

        match self.direction {
            VerticalDirection::Up => {
                // going up
                self.current_frame += 1;
                self.current_frame %= self.num_frames;

                if
                // bot collides with solid tile
                p.solids.get(
                self.position.x as u32 / TILE_WIDTH,
                (self.position.y as u32 - 1) / TILE_WIDTH) ||
            // bot has no more wall to stick upon
            !p.solids.get(
                (
                    self.position.x +
                    orientation *
                    TILE_WIDTH as i32
                ) as u32 / TILE_WIDTH,
                (self.position.y - 1) as u32 / TILE_HEIGHT)
                {
                    self.position.y += 1;
                    self.direction = VerticalDirection::Down;
                } else {
                    self.position.y -= 1;
                }
            }
            VerticalDirection::Down => {
                // going down
                if self.current_frame == 0 {
                    self.current_frame = self.num_frames;
                }
                self.current_frame -= 1;

                if
                // bot collides with solid tile
                p.solids.get(
                    self.position.x as u32 / TILE_WIDTH,
                    (
                        self.position.y as u32 + TILE_HEIGHT
                    ) / TILE_HEIGHT) ||
            // bot has no more wall to stick upon
            !p.solids.get(
                (
                    self.position.x +
                    orientation *
                    TILE_WIDTH as i32) as u32 /
                TILE_WIDTH,
                (self.position.y as u32 + TILE_HEIGHT) / TILE_HEIGHT)
                {
                    self.position.y -= 1;
                    self.direction = VerticalDirection::Up;
                } else {
                    self.position.y += 1;
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.is_alive = false;

        p.hero.score.add(100);
        p.actor_adder.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Steam),
            self.position.top_left(),
        );
        p.actor_adder.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Explosion),
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

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.position.has_intersection(hero.position.geometry)
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}
