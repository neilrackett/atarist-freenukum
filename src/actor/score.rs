use crate::{
    actor::{
        ActParameters, Actor, CreateActorWithDetails, RenderParameters,
    },
    level::tiles::LevelTiles,
    Result, Sizes, NUMBER_100, NUMBER_1000, NUMBER_10000, NUMBER_200,
    NUMBER_2000, NUMBER_500, NUMBER_5000, NUMBER_BONUS_1_LEFT,
    NUMBER_BONUS_1_RIGHT, NUMBER_BONUS_2_LEFT, NUMBER_BONUS_2_RIGHT,
    NUMBER_BONUS_3_LEFT, NUMBER_BONUS_3_RIGHT, NUMBER_BONUS_4_LEFT,
    NUMBER_BONUS_4_RIGHT, NUMBER_BONUS_5_LEFT, NUMBER_BONUS_5_RIGHT,
    NUMBER_BONUS_6_LEFT, NUMBER_BONUS_6_RIGHT, NUMBER_BONUS_7_LEFT,
    NUMBER_BONUS_7_RIGHT,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Score {
    tile: usize,
    countdown: usize,
    position: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreType {
    Score100,
    Score200,
    Score500,
    Score1000,
    Score2000,
    Score5000,
    Score10000,
    Bonus1Left,
    Bonus1Right,
    Bonus2Left,
    Bonus2Right,
    Bonus3Left,
    Bonus3Right,
    Bonus4Left,
    Bonus4Right,
    Bonus5Left,
    Bonus5Right,
    Bonus6Left,
    Bonus6Right,
    Bonus7Left,
    Bonus7Right,
}

impl CreateActorWithDetails for Score {
    type Details = ScoreType;

    fn create_with_details(
        score_type: ScoreType,
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Self {
        let tile = match score_type {
            ScoreType::Score100 => NUMBER_100,
            ScoreType::Score200 => NUMBER_200,
            ScoreType::Score500 => NUMBER_500,
            ScoreType::Score1000 => NUMBER_1000,
            ScoreType::Score2000 => NUMBER_2000,
            ScoreType::Score5000 => NUMBER_5000,
            ScoreType::Score10000 => NUMBER_10000,
            ScoreType::Bonus1Left => NUMBER_BONUS_1_LEFT,
            ScoreType::Bonus1Right => NUMBER_BONUS_1_RIGHT,
            ScoreType::Bonus2Left => NUMBER_BONUS_2_LEFT,
            ScoreType::Bonus2Right => NUMBER_BONUS_2_RIGHT,
            ScoreType::Bonus3Left => NUMBER_BONUS_3_LEFT,
            ScoreType::Bonus3Right => NUMBER_BONUS_3_RIGHT,
            ScoreType::Bonus4Left => NUMBER_BONUS_4_LEFT,
            ScoreType::Bonus4Right => NUMBER_BONUS_4_RIGHT,
            ScoreType::Bonus5Left => NUMBER_BONUS_5_LEFT,
            ScoreType::Bonus5Right => NUMBER_BONUS_5_RIGHT,
            ScoreType::Bonus6Left => NUMBER_BONUS_6_LEFT,
            ScoreType::Bonus6Right => NUMBER_BONUS_6_RIGHT,
            ScoreType::Bonus7Left => NUMBER_BONUS_7_LEFT,
            ScoreType::Bonus7Right => NUMBER_BONUS_7_RIGHT,
        };

        Self {
            tile,
            countdown: 40,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
        }
    }
}

impl Actor for Score {
    fn act(&mut self, _p: ActParameters) {
        self.countdown -= 1;
        self.position.y -= 1;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn is_alive(&self) -> bool {
        self.countdown > 0
            && self.position.y() > -(self.position.height() as i32)
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }
}
