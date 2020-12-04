use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorMessageType, ReceiveMessageParameters, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_LASERBEAM, TILE_HEIGHT, TILE_WIDTH,
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
    ) -> Self {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.is_in_foreground = false;

        Specific {
            tile: OBJECT_LASERBEAM,
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

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            p.general.position.top_left(),
        )?;
        Ok(())
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::OpenDoor {
            return;
        }
        let x = p.general.position.x() as u32 / TILE_WIDTH;
        let y = p.general.position.y() as u32 / TILE_HEIGHT;
        p.solids.set(x, y, false);
        p.general.is_alive = false;
    }
}
