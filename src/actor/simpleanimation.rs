use crate::{
    actor::{
        ActParameters, Actor, CreateActorWithDetails, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, Sizes, ANIMATION_BROKENWALLBG, ANIMATION_STONEWINDOWBG,
    ANIMATION_WINDOWBG,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleAnimationType {
    TextOnScreen,
    HighVoltageFlash,
    RedFlashlight,
    BlueFlashlight,
    Keypanel,
    RedRotationLight,
    UpArrow,
    BlueLight1,
    BlueLight2,
    BlueLight3,
    BlueLight4,
    GreenPoison,
    Lava,
    WindowLeft,
    WindowRight,
    StoneWindow,
    BrokenWall,
}

impl SimpleAnimationType {
    fn tile_and_num_frames(&self) -> (usize, usize) {
        match self {
            SimpleAnimationType::TextOnScreen => (0x0004, 4),
            SimpleAnimationType::HighVoltageFlash => (0x0008, 4),
            SimpleAnimationType::RedFlashlight => (0x000C, 4),
            SimpleAnimationType::BlueFlashlight => (0x0010, 4),
            SimpleAnimationType::Keypanel => (0x0014, 4),
            SimpleAnimationType::RedRotationLight => (0x0018, 4),
            SimpleAnimationType::UpArrow => (0x001C, 4),
            SimpleAnimationType::BlueLight1 => (0x0020, 4),
            SimpleAnimationType::BlueLight2 => (0x0021, 4),
            SimpleAnimationType::BlueLight3 => (0x0022, 4),
            SimpleAnimationType::BlueLight4 => (0x0023, 4),
            SimpleAnimationType::GreenPoison => (0x0028, 4),
            SimpleAnimationType::Lava => (0x002C, 4),
            SimpleAnimationType::WindowLeft => (ANIMATION_WINDOWBG, 1),
            SimpleAnimationType::WindowRight => {
                (ANIMATION_WINDOWBG + 1, 1)
            }
            SimpleAnimationType::StoneWindow => {
                (ANIMATION_STONEWINDOWBG, 1)
            }
            SimpleAnimationType::BrokenWall => (ANIMATION_BROKENWALLBG, 1),
        }
    }
}

impl CreateActorWithDetails for Specific {
    type Details = SimpleAnimationType;

    fn create_with_details(
        animation_type: SimpleAnimationType,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let (tile, num_frames) = animation_type.tile_and_num_frames();

        Specific {
            tile,
            current_frame: 0,
            num_frames,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        }
    }
}

impl Actor for Specific {
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

    fn is_in_foreground(&self) -> bool {
        false
    }
}
