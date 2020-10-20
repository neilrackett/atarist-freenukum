use super::super::super::hero::HeroData;
use super::super::super::level::solids::LevelSolids;
use super::super::super::level::tiles::LevelTiles;
use super::super::super::tilecache::TileCache;
use super::super::super::HorizontalDirection;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{
    ANIMATION_CARBOT, HALFTILE_HEIGHT, HALFTILE_WIDTH, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: usize,
    touching_hero: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        Specific {
            orientation: HorizontalDirection::Left,
            tile: ANIMATION_CARBOT,
            current_frame: 0,
            num_frames: 4,
            was_shot: 0,
            touching_hero: false,
        }
    }
}

impl ActorInterface for Specific {
    fn hero_touch_start(
        &mut self,
        general: &mut ActorData,
        _actor_queue: &mut ActorQueue,
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
        level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;

        if self.was_shot == 2 {
            general.is_alive = false;
            actor_queue.push_back(
                ActorType::Explosion,
                general.position.x as u16 + HALFTILE_WIDTH as u16,
                general.position.y as u16,
            );
            actor_queue.push_particle_firework(
                general.position.x as u16,
                general.position.y as u16,
                4,
            );
            hero_data.score.add(2500);
        } else {
            if level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                general.position.y as usize / TILE_HEIGHT + 1,
            ) && !level_data.solids.get(
                general.position.x as usize / TILE_WIDTH + 1,
                general.position.y as usize / TILE_HEIGHT + 1,
            ) {
                // still in the air, falling down
                general.position.y += HALFTILE_HEIGHT as i16;
            } else {
                // on the floor, walking
                let mut direction = match self.orientation {
                    HorizontalDirection::Left => -1,
                    HorizontalDirection::Right => 4,
                    HorizontalDirection::Center => unreachable!(),
                };

                if !level_data.solids.get(
                    // check if the place next ot the bot is free
                    (general.position.x as isize
                        + direction * HALFTILE_WIDTH as isize)
                        as usize
                        / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT,
                ) && level_data.solids.get(
                    // check if the tile below is solid
                    (general.position.x as isize
                        + direction * HALFTILE_WIDTH as isize)
                        as usize
                        / TILE_WIDTH,
                    (general.position.y as usize + TILE_HEIGHT)
                        / TILE_HEIGHT,
                ) {
                    if direction > 0 {
                        direction = 1;
                    }
                    general.position.x +=
                        (direction as f64 * HALFTILE_WIDTH as f64 * 0.7)
                            as i16;
                } else {
                    // reached the end, turning around
                    self.orientation = match self.orientation {
                        HorizontalDirection::Left => {
                            HorizontalDirection::Right
                        }
                        HorizontalDirection::Right => {
                            HorizontalDirection::Left
                        }
                        HorizontalDirection::Center => unreachable!(),
                    };
                    if direction > 0 {
                        direction = 1;
                    }
                    direction *= -1;
                    general.position.x +=
                        direction as i16 * HALFTILE_WIDTH as i16;
                    let tile = self.tile as isize + 4 * direction;
                    self.tile = tile as usize;

                    if direction > 0 {
                        actor_queue.push_back(
                            ActorType::HostileShotRight,
                            general.position.x as u16,
                            general.position.y as u16 - 6,
                        );
                    } else {
                        actor_queue.push_back(
                            ActorType::HostileShotLeft,
                            general.position.x as u16,
                            general.position.y as u16 - 6,
                        );
                    }
                }
            }
        }
        if self.was_shot == 1 {
            // create steam clouds
            if self.current_frame == 0 {
                actor_queue.push_back(
                    ActorType::Steam,
                    general.position.x as u16 + HALFTILE_WIDTH as u16,
                    general.position.y as u16 - TILE_HEIGHT as u16,
                );
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
        let mut destrect = general.position;
        let tile = tilecache
            .get_tile(self.tile + (self.current_frame / 2) * 2)
            .unwrap();
        tile.blit_to_sdl_surface(None, target, Some(destrect));

        let tile = tilecache
            .get_tile(self.tile + (self.current_frame / 2) * 2 + 1)
            .unwrap();
        destrect.x += TILE_WIDTH as i16;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }

    fn can_get_shot(&self, _gerenal: &ActorData) -> bool {
        true
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        if self.was_shot == 1 && self.touching_hero {
            general.hurts_hero = false;
            self.touching_hero = false;
        }
        if self.was_shot != 2 {
            self.was_shot += 1;
        }
    }
}
