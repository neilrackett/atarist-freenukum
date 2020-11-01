use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::hero::HeroData;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
use crate::{
    NUMBER_100, NUMBER_1000, NUMBER_10000, NUMBER_200, NUMBER_2000,
    NUMBER_500, NUMBER_5000, NUMBER_BONUS_1_LEFT, NUMBER_BONUS_1_RIGHT,
    NUMBER_BONUS_2_LEFT, NUMBER_BONUS_2_RIGHT, NUMBER_BONUS_3_LEFT,
    NUMBER_BONUS_3_RIGHT, NUMBER_BONUS_4_LEFT, NUMBER_BONUS_4_RIGHT,
    NUMBER_BONUS_5_LEFT, NUMBER_BONUS_5_RIGHT, NUMBER_BONUS_6_LEFT,
    NUMBER_BONUS_6_RIGHT, NUMBER_BONUS_7_LEFT, NUMBER_BONUS_7_RIGHT,
    TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    countdown: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.is_in_foreground = true;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let tile = match general.actor_type {
            ActorType::Score100 => NUMBER_100,
            ActorType::Score200 => NUMBER_200,
            ActorType::Score500 => NUMBER_500,
            ActorType::Score1000 => NUMBER_1000,
            ActorType::Score2000 => NUMBER_2000,
            ActorType::Score5000 => NUMBER_5000,
            ActorType::Score10000 => NUMBER_10000,
            ActorType::ScoreBonus1Left => NUMBER_BONUS_1_LEFT,
            ActorType::ScoreBonus1Right => NUMBER_BONUS_1_RIGHT,
            ActorType::ScoreBonus2Left => NUMBER_BONUS_2_LEFT,
            ActorType::ScoreBonus2Right => NUMBER_BONUS_2_RIGHT,
            ActorType::ScoreBonus3Left => NUMBER_BONUS_3_LEFT,
            ActorType::ScoreBonus3Right => NUMBER_BONUS_3_RIGHT,
            ActorType::ScoreBonus4Left => NUMBER_BONUS_4_LEFT,
            ActorType::ScoreBonus4Right => NUMBER_BONUS_4_RIGHT,
            ActorType::ScoreBonus5Left => NUMBER_BONUS_5_LEFT,
            ActorType::ScoreBonus5Right => NUMBER_BONUS_5_RIGHT,
            ActorType::ScoreBonus6Left => NUMBER_BONUS_6_LEFT,
            ActorType::ScoreBonus6Right => NUMBER_BONUS_6_RIGHT,
            ActorType::ScoreBonus7Left => NUMBER_BONUS_7_LEFT,
            ActorType::ScoreBonus7Right => NUMBER_BONUS_7_RIGHT,
            _ => {
                unreachable!(
                    "Actor type {:?} added as an score \
                    which is not a score id",
                    general.actor_type
                );
            }
        };

        Specific {
            tile,
            countdown: 40,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        self.countdown -= 1;
        general.position.y -= 1;
        if self.countdown == 0
            || general.position.y == -(TILE_HEIGHT as i16)
        {
            general.is_alive = false;
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = tilecache.get_tile(self.tile as usize).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}
