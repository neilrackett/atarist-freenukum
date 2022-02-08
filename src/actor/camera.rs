// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, ScoreType, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Result, Sizes, ANIMATION_CAMERA_CENTER, ANIMATION_CAMERA_LEFT,
    ANIMATION_CAMERA_RIGHT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Camera {
    tile: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for Camera {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::Camera(Self {
            tile: ANIMATION_CAMERA_CENTER,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
        })
    }
}

impl ActorExt for Camera {
    fn act(&mut self, p: ActParameters) {
        let x = p.hero.position.geometry.x;
        self.tile = if x - 1 > self.position.x {
            ANIMATION_CAMERA_RIGHT
        } else if x + 1 < self.position.x {
            ANIMATION_CAMERA_LEFT
        } else {
            ANIMATION_CAMERA_CENTER
        };
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        self.is_alive = false;
        p.hero.score.add(100);
        p.game_commands.add_actor(
            ActorType::Score(ScoreType::Score100),
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
        false
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::CopyFromBelow
    }
}
