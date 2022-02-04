use crate::{
    actor::{
        ActParameters, Actor, ActorExt, CreateActor, RenderParameters,
        ShotParameters, ShotProcessing,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    Result, Sizes, OBJECT_ROCKET,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Flying,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Rocket {
    state: State,
    position: Rect,
}

impl CreateActor for Rocket {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::Rocket(Self {
            state: State::Idle,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for Rocket {
    fn act(&mut self, p: ActParameters) {
        match self.state {
            State::Idle => {}
            State::Flying => {
                self.position.offset(0, -(p.sizes.half_height() as i32));
                if p.tiles.collides(p.sizes, self.position) {
                    let tile_x =
                        self.position.x() / p.sizes.width() as i32;
                    let tile_y =
                        self.position.y() / p.sizes.height() as i32;
                    if p.tiles.is_in_range(tile_x, tile_y + 1) {
                        if let Ok(ref mut t) =
                            p.tiles.get_mut(tile_x, tile_y + 1)
                        {
                            t.solid = false;
                        }
                    }
                    if p.tiles.is_in_range(tile_x, tile_y)
                        && p.tiles.is_in_range(tile_x, tile_y - 1)
                    {
                        p.tiles
                            .copy_effective_number_from_to(
                                tile_x,
                                tile_y - 1,
                                tile_x,
                                tile_y,
                            )
                            .expect("Can't copy effective number");
                    }
                }
            }
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let mut pos = self
            .position
            .top_left()
            .offset(0, -(p.sizes.height() as i32 * 3));

        let tile = OBJECT_ROCKET;
        p.renderer.place_tile(tile, pos)?;

        let tile = OBJECT_ROCKET + 1;
        for _ in 0..2 {
            pos.y += p.sizes.height() as i32;
            p.renderer.place_tile(tile, pos)?;
        }

        let tile = OBJECT_ROCKET + 2;
        pos.y += p.sizes.height() as i32;
        p.renderer.place_tile(tile, pos)?;

        let tile = OBJECT_ROCKET + 3;
        pos.x -= p.sizes.width() as i32;
        p.renderer.place_tile(tile, pos)?;

        let tile = OBJECT_ROCKET + 4;
        pos.x += 2 * p.sizes.width() as i32;
        p.renderer.place_tile(tile, pos)?;

        if self.state == State::Flying {
            let tile = OBJECT_ROCKET + 6;
            pos.x -= p.sizes.width() as i32;
            pos.y += p.sizes.height() as i32;
            p.renderer.place_tile(tile, pos)?;
        }
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        if self.state == State::Idle {
            // TODO: create animation
            self.state = State::Flying;
            let tile_x = self.position.x() / p.sizes.width() as i32;
            let tile_y = (self.position.y()
                + self.position.height() as i32)
                / p.sizes.height() as i32;

            let source = p
                .tiles
                .get(tile_x, tile_y + 1)
                .map(|t| t.effective_number)
                .unwrap_or(0);
            if let Ok(ref mut t) = p.tiles.get_mut(tile_x, tile_y) {
                t.solid = false;
                t.effective_number = source;
            }
        }
        ShotProcessing::Absorb
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        false
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromAbove
    }

    fn acts_while_invisible(&self) -> bool {
        self.state != State::Idle
    }
}
