use crate::{
    actor::{
        ActParameters, Actor, CreateActorWithDetails, RenderParameters,
    },
    level::tiles::LevelTiles,
    Result, Sizes, ANIMATION_BROKENWALLBG, ANIMATION_STONEWINDOWBG,
    ANIMATION_WINDOWBG,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct BackgroundAnimation {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundAnimationType {
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

impl BackgroundAnimationType {
    fn tile_and_num_frames(&self) -> (usize, usize) {
        match self {
            BackgroundAnimationType::TextOnScreen => (0x0004, 4),
            BackgroundAnimationType::HighVoltageFlash => (0x0008, 4),
            BackgroundAnimationType::RedFlashlight => (0x000C, 4),
            BackgroundAnimationType::BlueFlashlight => (0x0010, 4),
            BackgroundAnimationType::Keypanel => (0x0014, 4),
            BackgroundAnimationType::RedRotationLight => (0x0018, 4),
            BackgroundAnimationType::UpArrow => (0x001C, 4),
            BackgroundAnimationType::BlueLight1 => (0x0020, 4),
            BackgroundAnimationType::BlueLight2 => (0x0021, 4),
            BackgroundAnimationType::BlueLight3 => (0x0022, 4),
            BackgroundAnimationType::BlueLight4 => (0x0023, 4),
            BackgroundAnimationType::GreenPoison => (0x0028, 4),
            BackgroundAnimationType::Lava => (0x002C, 4),
            BackgroundAnimationType::WindowLeft => (ANIMATION_WINDOWBG, 1),
            BackgroundAnimationType::WindowRight => {
                (ANIMATION_WINDOWBG + 1, 1)
            }
            BackgroundAnimationType::StoneWindow => {
                (ANIMATION_STONEWINDOWBG, 1)
            }
            BackgroundAnimationType::BrokenWall => {
                (ANIMATION_BROKENWALLBG, 1)
            }
        }
    }
}

impl CreateActorWithDetails for BackgroundAnimation {
    type Details = BackgroundAnimationType;

    fn create_with_details(
        animation_type: BackgroundAnimationType,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Self {
        let (tile, num_frames) = animation_type.tile_and_num_frames();

        Self {
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

impl Actor for BackgroundAnimation {
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
