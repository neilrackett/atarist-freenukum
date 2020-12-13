use crate::{
    actor::{
        ActParameters, ActorCreateInterface, ActorData, ActorInterface,
        ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, OBJECT_HOSTILESHOT, TILE_HEIGHT, TILE_WIDTH,
};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    touching_hero: bool,
    current_frame: usize,
    num_frames: usize,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.resize(TILE_WIDTH, TILE_HEIGHT);
        general.acts_while_invisible = true;

        let tile = match general.actor_type {
            ActorType::HostileShotLeft => OBJECT_HOSTILESHOT,
            ActorType::HostileShotRight => OBJECT_HOSTILESHOT + 2,
            _ => unreachable!(
                "Passed actor type {:?} to hostile shot actor \
                which is not a shot actor id",
                general.actor_type
            ),
        };

        Specific {
            tile,
            touching_hero: false,
            current_frame: 0,
            num_frames: 2,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        self.touching_hero = true;
        p.general.hurts_hero = true;
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        self.touching_hero = false;
        p.general.hurts_hero = false;
    }

    fn act(&mut self, p: ActParameters) {
        let offset = match p.general.actor_type {
            ActorType::HostileShotLeft => -(TILE_WIDTH as i32),
            ActorType::HostileShotRight => TILE_WIDTH as i32,
            _ => unreachable!(),
        };
        p.general.position.offset(offset, 0);

        if p.solids.get(
            p.general.position.x() as u32 / TILE_WIDTH,
            p.general.position.y() as u32 / TILE_HEIGHT,
        ) {
            p.general.is_alive = false;
            if self.touching_hero {
                p.general.hurts_hero = false;
            }
        }

        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            p.general.position.top_left(),
        )?;
        Ok(())
    }
}
