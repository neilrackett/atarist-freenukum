use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::{
    ANIMATION_BOMBFIRE, ANIMATION_EXPLOSION, ANIMATION_ROBOT,
    OBJECT_DUSTCLOUD, OBJECT_STEAM, TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    can_hurt_hero: bool,
    replaced_by: Option<ActorType>,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.is_in_foreground = false;
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;

        let (tile, num_frames, can_hurt_hero, replaced_by) = match general
            .actor_type
        {
            ActorType::BombFire => (ANIMATION_BOMBFIRE, 6, true, None),
            ActorType::Explosion => (ANIMATION_EXPLOSION, 6, false, None),
            ActorType::DustCloud => (OBJECT_DUSTCLOUD, 5, false, None),
            ActorType::Steam => (OBJECT_STEAM, 5, false, None),
            ActorType::RobotDisappearing => {
                (ANIMATION_ROBOT + 3, 7, false, Some(ActorType::Explosion))
            }
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
            can_hurt_hero,
            replaced_by,
        }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        if self.current_frame == self.num_frames {
            general.is_alive = false;
            if let Some(successor) = self.replaced_by {
                actor_adder.add_actor(
                    successor,
                    general.position.x as u16,
                    general.position.y as u16,
                );
            }
        }
    }

    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        if self.can_hurt_hero {
            general.hurts_hero = true;
        }
    }

    fn hero_touch_end(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = false;
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = tilecache
            .get_tile((self.tile + self.current_frame) as usize)
            .unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}
