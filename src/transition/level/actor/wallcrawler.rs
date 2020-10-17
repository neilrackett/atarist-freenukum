use super::super::super::hero::HeroData;
use super::super::super::tilecache::TileCache;
use super::super::super::{HorizontalDirection, VerticalDirection};
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorQueue, ActorType,
};
use crate::{
    ANIMATION_WALLCRAWLERBOT_LEFT, ANIMATION_WALLCRAWLERBOT_RIGHT,
    TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(Debug)]
pub(crate) struct Specific {
    direction: VerticalDirection,
    orientation: HorizontalDirection,
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    was_shot: bool,
    touching_hero: bool,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        let (tile, orientation) = match general.actor_type {
            ActorType::WallCrawlerBotLeft => {
                (ANIMATION_WALLCRAWLERBOT_LEFT, HorizontalDirection::Left)
            }
            ActorType::WallCrawlerBotRight => (
                ANIMATION_WALLCRAWLERBOT_RIGHT,
                HorizontalDirection::Right,
            ),
            _ => unreachable!(),
        };

        Specific {
            direction: VerticalDirection::Up,
            orientation,
            tile,
            current_frame: 0,
            num_frames: 4,
            was_shot: false,
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
        _actor_queue: &mut ActorQueue,
        _hero_data: &mut HeroData,
    ) {
        let direction = match self.direction {
            VerticalDirection::Up => 1,
            VerticalDirection::Down => -1,
            VerticalDirection::Center => unreachable!(),
        };
        let orientation = match self.orientation {
            HorizontalDirection::Left => -1,
            HorizontalDirection::Right => 1,
            HorizontalDirection::Center => unreachable!(),
        };

        if direction > 0 {
            // going up
            self.current_frame += 1;
            self.current_frame %= self.num_frames;

            if
            // bot collides with solid tile
            level_data.solids.get(
                general.position.x as usize / TILE_WIDTH,
                (general.position.y as usize - 1) / TILE_WIDTH) ||
            // bot has no more wall to stick upon
            !level_data.solids.get(
                (
                    general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (general.position.y as usize - 1) / TILE_HEIGHT)
            {
                general.position.y += 1;
                self.direction = VerticalDirection::Down;
            } else {
                general.position.y -= 1;
            }
        } else {
            // going down
            if self.current_frame == 0 {
                self.current_frame = self.num_frames;
            }
            self.current_frame -= 1;

            if
            // bot collides with solid tile
            level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    (
                        general.position.y as usize + TILE_HEIGHT
                    ) / TILE_HEIGHT) ||
            // bot has no more wall to stick upon
            !level_data.solids.get(
                (
                    general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize) as usize /
                TILE_WIDTH,
                (general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT)
            {
                general.position.y -= 1;
                self.direction = VerticalDirection::Up;
            } else {
                general.position.y += 1;
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
        let tile =
            tilecache.get_tile(self.tile + self.current_frame).unwrap();
        let destrect = general.position;
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        if !self.was_shot {
            if self.touching_hero {
                general.hurts_hero = false;
                self.touching_hero = false;
                self.current_frame = 0;
            }
            general.is_alive = false;

            hero_data.score.add(100);
            actor_queue.push_back(
                ActorType::Steam,
                general.position.x as u16,
                general.position.y as u16,
            );
            actor_queue.push_back(
                ActorType::Explosion,
                general.position.x as u16,
                general.position.y as u16,
            );
        }
    }
}
