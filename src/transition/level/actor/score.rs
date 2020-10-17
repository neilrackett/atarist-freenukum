use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
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
        _level_data: &mut LevelData,
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
        _level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
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

pub mod ffi {
    use super::super::ffi::{
        FnLevelActorActParams, FnLevelActorBlitParams,
        FnLevelActorCreateParams, FnLevelActorFreeParams,
    };

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_create(
        p: FnLevelActorCreateParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_free(
        p: FnLevelActorFreeParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_act(
        p: FnLevelActorActParams,
    ) {
        p.call_interface::<super::Specific>();
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_function_score_blit(
        p: FnLevelActorBlitParams,
    ) {
        p.call_interface::<super::Specific>();
    }
}
