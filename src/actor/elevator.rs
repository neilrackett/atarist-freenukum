use crate::actor::{
    ActorAdder, ActorCreateInterface, ActorData, ActorInterface,
    ActorMessageQueue,
};
use crate::hero::HeroData;
use crate::infobox::InfoMessageQueue;
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::tilecache::TileCache;
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
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
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
        solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
        _actor_adder: &mut dyn ActorAdder,
        hero_data: &mut HeroData,
        _do_play: &mut bool,
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
                if solids.get(
                    general.position.x as usize / TILE_WIDTH,
                    general.position.y as usize / TILE_HEIGHT - 3,
                ) {
                    // hero touches solid with head
                    self.state = State::Idle;
                } else {
                    let offset = hero_data
                        .position
                        .push_vertically(&solids, -(TILE_HEIGHT as i16));
                    if -offset < TILE_HEIGHT as i16 {
                        hero_data
                            .position
                            .push_vertically(&solids, -offset);
                        self.state = State::Idle;
                    } else {
                        general.position.h += (-offset) as u16;
                        general.position.y += offset as i16;

                        solids.set(
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
                        solids.set(
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
        _level_passed: &mut bool,
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
        _level_passed: &mut bool,
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
