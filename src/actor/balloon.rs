use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchStartParameters, RenderParameters, ShotParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{OBJECT_BALLOON, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug)]
pub(crate) struct Specific {
    destroyed: bool,
    current_frame: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16 * 2;

        Specific {
            destroyed: false,
            current_frame: 0,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        if !self.destroyed {
            p.general.is_alive = false;
            p.hero_data.score.add(10000);
            p.actor_adder.add_actor(
                ActorType::Score10000,
                p.general.position.x as u16,
                p.general.position.y as u16,
            );
        }
    }

    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= 9;

        if self.destroyed {
            p.general.is_alive = false;
        } else {
            p.general.position.y -= 1;
            if p.solids.get(
                p.general.position.x as usize / TILE_WIDTH,
                p.general.position.y as usize / TILE_WIDTH,
            ) {
                // balloon bumps against wall
                self.destroyed = true;
                p.actor_adder.add_actor(
                    ActorType::Steam,
                    p.general.position.x as u16,
                    p.general.position.y as u16,
                );
            }
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let mut destrect = p.general.position;

        let tile = if self.destroyed {
            OBJECT_BALLOON + 4
        } else {
            OBJECT_BALLOON
        };
        p.tilecache.get_tile(tile).unwrap().blit_to_sdl_surface(
            None,
            p.target,
            Some(destrect),
        );

        destrect.y += TILE_HEIGHT as i16;
        p.tilecache
            .get_tile(OBJECT_BALLOON + 1 + self.current_frame / 3)
            .unwrap()
            .blit_to_sdl_surface(None, p.target, Some(destrect));
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        self.destroyed = true;
        p.actor_adder.add_actor(
            ActorType::Steam,
            p.general.position.x as u16,
            p.general.position.y as u16,
        );
    }
}
