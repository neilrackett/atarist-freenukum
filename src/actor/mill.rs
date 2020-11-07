use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchStartParameters, RenderParameters, ShotParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{OBJECT_ROTATINGCYLINDER, TILE_HEIGHT, TILE_WIDTH};

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
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.hero_data.health.kill();
    }

    fn act(&mut self, _p: ActParameters) {
        if self.lives > 0 {
            self.current_frame += 1;
            self.current_frame %= self.num_frames;
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let mut destrect = p.general.position;
        let tile = p
            .tilecache
            .get_tile(self.tile + self.current_frame)
            .unwrap();

        for _ in 0..p.general.position.h as usize / TILE_WIDTH {
            tile.blit_to_sdl_surface(None, p.target, Some(destrect));
            destrect.y += TILE_HEIGHT as i16;
        }
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        self.lives -= 1;
        if self.lives > 0 {
            p.actor_adder.add_particle_firework(
                p.general.position.x as u16 + p.general.position.w / 2,
                p.general.position.y as u16 + p.general.position.h / 2,
                4,
            );
        } else {
            // TODO: add removal animation (destroyed body)
            p.general.is_alive = false;
            p.hero_data.score.add(20000);
            p.actor_adder.add_particle_firework(
                p.general.position.x as u16 + p.general.position.w / 2,
                p.general.position.y as u16 + p.general.position.h / 2,
                20,
            );
            p.actor_adder.add_actor(
                ActorType::Score10000,
                p.general.position.x as u16,
                p.general.position.y as u16 + p.general.position.h / 2
                    - TILE_HEIGHT as u16,
            );
            p.actor_adder.add_actor(
                ActorType::Score10000,
                p.general.position.x as u16,
                p.general.position.y as u16 + p.general.position.h / 2,
            );
        }
    }
}
