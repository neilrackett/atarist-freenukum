use crate::{
    actor::{
        ActParameters, ActorInterface, ActorType, CreateActor,
        RenderParameters, ScoreType, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_BALLOON, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    destroyed: bool,
    current_frame: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for Specific {
    fn create(
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        Specific {
            destroyed: false,
            current_frame: 0,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT * 2),
            is_alive: true,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= 9;

        if self.position.has_intersection(p.hero.position.geometry) {
            self.is_alive = false;
            p.hero.score.add(10000);
            p.actor_adder.add_actor(
                ActorType::Score(ScoreType::Score10000),
                self.position.top_left(),
            );
        } else {
            self.position.y -= 1;
            if p.solids.get(
                self.position.x() as u32 / TILE_WIDTH,
                self.position.y() as u32 / TILE_WIDTH,
            ) {
                // balloon bumps against wall
                self.destroyed = true;
                p.actor_adder.add_actor(
                    ActorType::SingleAnimation(SingleAnimationType::Steam),
                    self.position.top_left(),
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self.position.top_left();

        let tile = if self.destroyed {
            OBJECT_BALLOON + 4
        } else {
            OBJECT_BALLOON
        };
        p.renderer.place_tile(tile, pos)?;

        pos.y += TILE_HEIGHT as i32;
        p.renderer.place_tile(
            OBJECT_BALLOON + 1 + self.current_frame / 3,
            pos,
        )?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.destroyed = true;
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
}
