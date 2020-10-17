use super::super::super::hero::HeroData;
use super::super::super::infobox::InfoMessageQueue;
use super::super::super::tilecache::TileCache;
use super::super::LevelData;
use super::{
    ActorCreateInterface, ActorData, ActorInterface, ActorMessageQueue,
    ActorQueue,
};
use crate::{
    HALFTILE_HEIGHT, OBJECT_ELEVATOR_TOP, SOLID_ELEVATOR, TILE_HEIGHT,
    TILE_WIDTH,
};
use transdl::video::Surface;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Idle,
    Ascending,
    Descending,
}

#[derive(Debug)]
pub(crate) struct Specific {
    state: State,
}

impl ActorCreateInterface for Specific {
    fn create(
        general: &mut ActorData,
        _level_data: &mut LevelData,
    ) -> Specific {
        general.position.w = TILE_WIDTH as u16 * 2;
        general.position.h = TILE_HEIGHT as u16;
        general.is_in_foreground = true;

        Specific { state: State::Idle }
    }
}

impl ActorInterface for Specific {
    fn act(
        &mut self,
        general: &mut ActorData,
        level_data: &mut LevelData,
        _actor_queue: &mut ActorQueue,
        hero_data: &mut HeroData,
    ) {
        let hero_geometry = hero_data.position.geometry;

        if self.state == State::Ascending
            || self.state == State::Idle
                && general.position.h as usize > TILE_HEIGHT
        {
            // check if hero leaves elevator
            if !hero_geometry.touches(general.position)
                || general.position.x != hero_geometry.x
            {
                self.state = State::Descending;
            }
        }

        match self.state {
            State::Ascending => {
                if level_data.solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT - 3,
                ) {
                    // hero touches solid with head
                    self.state = State::Idle;
                } else {
                    let offset = hero_data.position.push_vertically(
                        &level_data.solids,
                        -(TILE_HEIGHT as i16),
                    );
                    if -offset < TILE_HEIGHT as i16 {
                        hero_data
                            .position
                            .push_vertically(&level_data.solids, -offset);
                        self.state = State::Idle;
                    } else {
                        general.position.h += (-offset) as u16;
                        general.position.y += offset as i16;

                        level_data.solids.set(
                            general.position.x as usize / TILE_WIDTH,
                            general.position.y as usize / TILE_HEIGHT,
                            true,
                        );
                    }
                }
            }
            State::Descending => {
                for _ in 0..2 {
                    if general.position.h as usize > TILE_HEIGHT {
                        level_data.solids.set(
                            general.position.x as usize / TILE_WIDTH,
                            general.position.y as usize / TILE_HEIGHT,
                            false,
                        );
                        general.position.y += TILE_HEIGHT as i16;
                        general.position.h -= TILE_HEIGHT as u16;
                    } else {
                        self.state = State::Idle;
                    }
                }
            }
            State::Idle => {}
        }
    }

    fn hero_can_interact(&self) -> bool {
        true
    }

    fn hero_interact_start(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        hero_data: &mut HeroData,
        _info_message_queue: &mut InfoMessageQueue,
        _actor_message_queue: &mut ActorMessageQueue,
    ) {
        if hero_data.position.geometry.touches(general.position)
            && hero_data.position.geometry.y
                + hero_data.position.geometry.h as i16
                == general.position.y
        {
            self.state = State::Ascending;
        }
    }

    fn hero_interact_end(
        &mut self,
        general: &mut ActorData,
        _level_data: &mut LevelData,
        hero_data: &mut HeroData,
    ) {
        if hero_data.position.geometry.touches(general.position)
            && hero_data.position.geometry.x == general.position.x
        {
            self.state = State::Idle;
        } else {
            self.state = State::Descending;
        }
    }

    fn blit(
        &mut self,
        general: &mut ActorData,
        _hero_data: &mut HeroData,
        tilecache: &TileCache,
        target: &mut Surface,
    ) {
        let tile = tilecache.get_tile(SOLID_ELEVATOR).unwrap();
        let mut destrect = general.position;
        for _ in 0..(general.position.h as usize / TILE_HEIGHT - 1) * 2 {
            destrect.y += HALFTILE_HEIGHT as i16;
            tile.blit_to_sdl_surface(None, target, Some(destrect));
        }
        destrect = general.position;
        let tile = tilecache.get_tile(OBJECT_ELEVATOR_TOP).unwrap();
        tile.blit_to_sdl_surface(None, target, Some(destrect));
    }
}
