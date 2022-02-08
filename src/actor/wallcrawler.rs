// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActorWithDetails,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Hero, HorizontalDirection, Result, Sizes, VerticalDirection,
    ANIMATION_WALLCRAWLERBOT_LEFT, ANIMATION_WALLCRAWLERBOT_RIGHT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct WallCrawler {
    direction: VerticalDirection,
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    is_alive: bool,
    position: Rect,
}

impl CreateActorWithDetails for WallCrawler {
    type Details = HorizontalDirection;

    fn create_with_details(
        orientation: HorizontalDirection,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        let tile = match orientation {
            HorizontalDirection::Left => ANIMATION_WALLCRAWLERBOT_LEFT,
            HorizontalDirection::Right => ANIMATION_WALLCRAWLERBOT_RIGHT,
        };

        Actor::WallCrawler(Self {
            direction: VerticalDirection::Up,
            orientation,
            tile,
            current_frame: 0,
            num_frames: 4,
            is_alive: true,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        })
    }
}

impl ActorExt for WallCrawler {
    fn act(&mut self, p: ActParameters) {
        let orientation = self.orientation.as_factor_i32();

        match self.direction {
            VerticalDirection::Up => {
                // going up
                self.current_frame += 1;
                self.current_frame %= self.num_frames;

                let solid_above = p
                    .tiles
                    .get(
                        self.position.x / p.sizes.width() as i32,
                        (self.position.top() - 1)
                            / p.sizes.height() as i32,
                    )
                    .map(|t| t.solid)
                    .unwrap_or(true);
                let solid_above_wall = p
                    .tiles
                    .get(
                        (self.position.x / p.sizes.width() as i32)
                            + orientation,
                        (self.position.top() - 1)
                            / p.sizes.height() as i32,
                    )
                    .map(|t| t.solid)
                    .unwrap_or(true);

                if solid_above || !solid_above_wall {
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

                let solid_below = p
                    .tiles
                    .get(
                        self.position.x / p.sizes.width() as i32,
                        (self.position.bottom() + 1)
                            / p.sizes.height() as i32,
                    )
                    .map(|t| t.solid)
                    .unwrap_or(true);
                let solid_below_wall = p
                    .tiles
                    .get(
                        (self.position.x / p.sizes.width() as i32)
                            + orientation,
                        (self.position.bottom() + 1)
                            / p.sizes.height() as i32,
                    )
                    .map(|t| t.solid)
                    .unwrap_or(true);

                if solid_below || !solid_below_wall {
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
        p.game_commands.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Steam),
            self.position.top_left(),
        );
        p.game_commands.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Explosion),
            self.position.top_left(),
        );
        p.game_commands.add_sound(SoundIndex::SMALLDEATH);

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

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        match self.orientation {
            HorizontalDirection::Left => {
                BackgroundTileStrategy::CopyFromRight
            }
            HorizontalDirection::Right => {
                BackgroundTileStrategy::CopyFromLeft
            }
        }
    }
}
