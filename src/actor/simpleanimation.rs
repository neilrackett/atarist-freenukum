use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, ANIMATION_BROKENWALLBG, ANIMATION_STONEWINDOWBG,
    ANIMATION_WINDOWBG, TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.is_in_foreground = false;

        let (tile, num_frames) = match general.actor_type {
            ActorType::TextOnScreenBackground => (0x0004, 4),
            ActorType::HighVoltageFlashBackground => (0x0008, 4),
            ActorType::RedFlashlightBackground => (0x000C, 4),
            ActorType::BlueFlashlightBackground => (0x0010, 4),
            ActorType::KeypanelBackground => (0x0014, 4),
            ActorType::RedRotationLightBackground => (0x0018, 4),
            ActorType::UpArrowBackground => (0x001C, 4),
            ActorType::BlueLightBackground1 => (0x0020, 4),
            ActorType::BlueLightBackground2 => (0x0021, 4),
            ActorType::BlueLightBackground3 => (0x0022, 4),
            ActorType::BlueLightBackground4 => (0x0023, 4),
            ActorType::GreenPoisonBackground => (0x0028, 4),
            ActorType::LavaBackground => (0x002C, 4),
            ActorType::WindowLeftBackground => (ANIMATION_WINDOWBG, 1),
            ActorType::WindowRightBackground => {
                (ANIMATION_WINDOWBG + 1, 1)
            }
            ActorType::StoneWindowBackground => {
                (ANIMATION_STONEWINDOWBG, 1)
            }
            ActorType::BrokenWallBackground => (ANIMATION_BROKENWALLBG, 1),
            _ => {
                unreachable!(
                    "Actor type {:?} added as an animation \
                    which is not an animation id",
                    general.actor_type
                );
            }
        };

        Specific {
            tile,
            current_frame: 0,
            num_frames,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
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
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }
}
