use super::super::super::hero::HeroData;
use super::super::super::level::solids::LevelSolids;
use super::super::super::level::tiles::LevelTiles;
use super::super::super::tilecache::TileCache;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::{OBJECT_ROTATINGCYLINDER, TILE_HEIGHT, TILE_WIDTH};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    lives: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = false;

        while general.position.y > 0
            && !solids.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT - 1,
            )
        {
            general.position.y -= TILE_HEIGHT as i16;
            general.position.h += TILE_HEIGHT as u16;
        }

        Specific {
            tile: OBJECT_ROTATINGCYLINDER,
            current_frame: 0,
            num_frames: 5,
            lives: 10,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        _general: &mut ActorData,
        _actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
    ) {
        hero_data.health.kill();
    }

    fn act(
        &mut self,
        _general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        if self.lives > 0 {
            self.current_frame += 1;
            self.current_frame %= self.num_frames;
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let mut destrect = general.position;
        let tile =
            tilecache.get_tile(self.tile + self.current_frame).unwrap();

        for _ in 0..general.position.h as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, target, Some(destrect));
            destrect.y += TILE_HEIGHT as i16;
        }
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_solids: &mut LevelSolids,
        _level_tiles: &mut LevelTiles,
        actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
    ) {
        self.lives -= 1;
        if self.lives > 0 {
            actor_adder.add_particle_firework(
                general.position.x as u16 + general.position.w / 2,
                general.position.y as u16 + general.position.h / 2,
                4,
            );
        } else {
            // TODO: add removal animation (destroyed body)
            general.is_alive = false;
            hero_data.score.add(20000);
            actor_adder.add_particle_firework(
                general.position.x as u16 + general.position.w / 2,
                general.position.y as u16 + general.position.h / 2,
                20,
            );
            actor_adder.add_actor(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16 + general.position.h / 2
                    - TILE_HEIGHT as u16,
            );
            actor_adder.add_actor(
                ActorType::Score10000,
                general.position.x as u16,
                general.position.y as u16 + general.position.h / 2,
            );
        }
    }
}
