use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageType, ReceiveMessageParameters, RenderParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{SOLID_EXPANDINGFLOOR, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug)]
pub(crate) struct Specific {
    expanding: bool,
    finished: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        Specific {
            expanding: false,
            finished: false,
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        if self.expanding {
            let x = (p.general.position.x as usize
                + p.general.position.w as usize)
                / TILE_WIDTH;
            let y = p.general.position.y as usize / TILE_HEIGHT;
            let can_expand = !p.solids.get(x, y);
            if can_expand {
                p.solids.set(x, y, true);
                p.general.position.w += TILE_WIDTH as u16;
            } else {
                self.expanding = false;
                self.finished = true;
            }
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let tile =
            p.tilecache.get_tile(SOLID_EXPANDINGFLOOR as usize).unwrap();
        let mut destrect = p.general.position;
        for _ in 0..p.general.position.w as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, p.target, Some(destrect));
            destrect.x += TILE_WIDTH as i16;
        }
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        if p.message != ActorMessageType::Expand {
            return;
        }
        if !self.expanding && !self.finished {
            self.expanding = true;
        }
    }
}
