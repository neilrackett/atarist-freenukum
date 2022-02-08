// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType, CreateActor,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Hero, Result, Sizes, OBJECT_ELECTRIC_ARC, OBJECT_ELECTRIC_ARC_HURTING,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct ElectricArc {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for ElectricArc {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::ElectricArc(Self {
            tile: OBJECT_ELECTRIC_ARC,
            current_frame: 0,
            num_frames: 4,
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

impl ActorExt for ElectricArc {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
        p.game_commands.add_sound(SoundIndex::FORCEFIELD);
        self.tile =
            if p.hero.position.geometry.has_intersection(self.position) {
                OBJECT_ELECTRIC_ARC_HURTING
            } else {
                OBJECT_ELECTRIC_ARC
            }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn can_receive_message(&self, message: ActorMessageType) -> bool {
        message == ActorMessageType::RemoveElectricArc
    }

    fn receive_message(&mut self, _p: ReceiveMessageParameters) {
        self.is_alive = false;
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
        BackgroundTileStrategy::CopyFromAbove
    }
}
