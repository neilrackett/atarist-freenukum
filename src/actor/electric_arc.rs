use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, HeroTouchEndParameters,
        HeroTouchStartParameters, ReceiveMessageParameters,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_ELECTRIC_ARC, OBJECT_ELECTRIC_ARC_HURTING, TILE_HEIGHT,
    TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.is_in_foreground = true;
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);

        Specific {
            tile: OBJECT_ELECTRIC_ARC,
            current_frame: 0,
            num_frames: 4,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, _p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        self.tile = OBJECT_ELECTRIC_ARC_HURTING;
        p.general.hurts_hero = true;
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        self.tile = OBJECT_ELECTRIC_ARC;
        p.general.hurts_hero = false;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            p.general.position.top_left(),
        )?;
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::Remove {
            return;
        }
        p.general.is_alive = false;
    }
}
