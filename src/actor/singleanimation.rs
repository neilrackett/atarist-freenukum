use crate::{
    actor::{
        ActParameters, ActorInterface, ActorType, CreateActorWithDetails,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, ANIMATION_BOMBFIRE, ANIMATION_EXPLOSION,
    ANIMATION_ROBOT, OBJECT_DUSTCLOUD, OBJECT_STEAM, TILE_HEIGHT,
    TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    can_hurt_hero: bool,
    replaced_by: Option<ActorType>,
    position: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleAnimationType {
    BombFire,
    Explosion,
    DustCloud,
    Steam,
    RobotDisappearing,
}

impl CreateActorWithDetails for Specific {
    type Details = SingleAnimationType;

    fn create_with_details(
        animation_type: SingleAnimationType,
        pos: Point,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        let (tile, num_frames, can_hurt_hero, replaced_by) =
            match animation_type {
                SingleAnimationType::BombFire => {
                    (ANIMATION_BOMBFIRE, 6, true, None)
                }
                SingleAnimationType::Explosion => {
                    (ANIMATION_EXPLOSION, 6, false, None)
                }
                SingleAnimationType::DustCloud => {
                    (OBJECT_DUSTCLOUD, 5, false, None)
                }
                SingleAnimationType::Steam => {
                    (OBJECT_STEAM, 5, false, None)
                }
                SingleAnimationType::RobotDisappearing => (
                    ANIMATION_ROBOT + 3,
                    7,
                    false,
                    Some(ActorType::SingleAnimation(
                        SingleAnimationType::Explosion,
                    )),
                ),
            };

        Specific {
            tile,
            current_frame: 0,
            num_frames,
            can_hurt_hero,
            replaced_by,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
        }
    }
}

impl ActorInterface for Specific {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        if !self.is_alive() {
            if let Some(successor) = self.replaced_by {
                p.actor_adder
                    .add_actor(successor, self.position.top_left());
            }
        }
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

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.can_hurt_hero
            && self.position.has_intersection(hero.position.geometry)
    }

    fn is_alive(&self) -> bool {
        self.current_frame < self.num_frames
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }
}
