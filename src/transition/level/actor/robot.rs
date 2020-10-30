use super::super::super::hero::HeroData;
use super::super::super::level::solids::LevelSolids;
use super::super::super::level::tiles::LevelTiles;
use super::super::super::tilecache::TileCache;
use super::super::super::HorizontalDirection;
use super::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface, ActorType,
};
use crate::{
    ANIMATION_ROBOT, HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    direction: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    touching_hero: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        Specific {
            direction: HorizontalDirection::Left,
            tile: ANIMATION_ROBOT,
            current_frame: 0,
            num_frames: 3,
            touching_hero: false,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = true;
        self.touching_hero = true;
    }

    fn hero_touch_end(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
    ) {
        general.hurts_hero = false;
        self.touching_hero = false;
    }

    fn act(
        &mut self,
        general: &mut ActorData,
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        _hero_data: &mut HeroData,
        _do_play: &mut bool,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if !solids.get(
            general.position.x as usize / TILE_WIDTH,
            general.position.y as usize / TILE_HEIGHT + 1,
        ) {
            // In the air, falling down.
            general.position.y += HALFTILE_HEIGHT as i16;
        } else {
            // On the floor, walking.
            if self.current_frame == 0 {
                let mut direction = match self.direction {
                    HorizontalDirection::Left => -1,
                    HorizontalDirection::Right => 2,
                    HorizontalDirection::Center => unreachable!(),
                };
                // Check if the place next to the bot is free
                if !solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize/ TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT
            ) &&
            // Check if the tile below this free place is solid
            solids.get(
                (
                    general.position.x as isize +
                    direction * HALFTILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT
            ) {
                    if direction == 2 {
                        direction = 1;
                    }
                    general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                } else {
                    self.direction =
                        if self.direction == HorizontalDirection::Left {
                            HorizontalDirection::Right
                        } else {
                            HorizontalDirection::Left
                        };
                    if direction == 2 {
                        direction = 1
                    };
                    direction *= -1;
                    general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                }
            }
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
        hero_data.score.add(100);
        if self.touching_hero {
            general.hurts_hero = false;
            self.touching_hero = false;
        }
        actor_adder.add_actor(
            ActorType::RobotDisappearing,
            general.position.x as u16,
            general.position.y as u16,
        );
        general.is_alive = false;
    }
}
