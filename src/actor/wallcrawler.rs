use crate::actor::{
    ActParameters, ActorCreateInterface, ActorData, ActorInterface,
    ActorType, HeroTouchEndParameters, HeroTouchStartParameters,
    RenderParameters, ShotParameters,
};
use crate::level::solids::LevelSolids;
use crate::level::tiles::LevelTiles;
use crate::{
    HorizontalDirection, VerticalDirection, ANIMATION_WALLCRAWLERBOT_LEFT,
    ANIMATION_WALLCRAWLERBOT_RIGHT, TILE_HEIGHT, TILE_WIDTH,
};

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
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
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
    fn hero_touch_start(&mut self, p: HeroTouchStartParameters) {
        p.general.hurts_hero = true;
        self.touching_hero = true;
    }

    fn hero_touch_end(&mut self, p: HeroTouchEndParameters) {
        p.general.hurts_hero = false;
        self.touching_hero = false;
    }

    fn act(&mut self, p: ActParameters) {
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
            p.solids.get(
                p.general.position.x as usize / TILE_WIDTH,
                (p.general.position.y as usize - 1) / TILE_WIDTH) ||
            // bot has no more wall to stick upon
            !p.solids.get(
                (
                    p.general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize
                ) as usize / TILE_WIDTH,
                (p.general.position.y as usize - 1) / TILE_HEIGHT)
            {
                p.general.position.y += 1;
                self.direction = VerticalDirection::Down;
            } else {
                p.general.position.y -= 1;
            }
        } else {
            // going down
            if self.current_frame == 0 {
                self.current_frame = self.num_frames;
            }
            self.current_frame -= 1;

            if
            // bot collides with solid tile
            p.solids.get(
                    p.general.position.x as usize / TILE_WIDTH,
                    (
                        p.general.position.y as usize + TILE_HEIGHT
                    ) / TILE_HEIGHT) ||
            // bot has no more wall to stick upon
            !p.solids.get(
                (
                    p.general.position.x as isize +
                    orientation as isize *
                    TILE_WIDTH as isize) as usize /
                TILE_WIDTH,
                (p.general.position.y as usize + TILE_HEIGHT) / TILE_HEIGHT)
            {
                p.general.position.y -= 1;
                self.direction = VerticalDirection::Up;
            } else {
                p.general.position.y += 1;
            }
        }
    }

    fn render(&mut self, p: RenderParameters) {
        let tile = p
            .tilecache
            .get_tile(self.tile + self.current_frame)
            .unwrap();
        let destrect = p.general.position;
        tile.blit_to_sdl_surface(None, p.target, Some(destrect));
    }

    fn can_get_shot(&self, _general: &ActorData) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) {
        if !self.was_shot {
            if self.touching_hero {
                p.general.hurts_hero = false;
                self.touching_hero = false;
                self.current_frame = 0;
            }
            p.general.is_alive = false;

            p.hero_data.score.add(100);
            p.actor_adder.add_actor(
                ActorType::Steam,
                p.general.position.x as u16,
                p.general.position.y as u16,
            );
            p.actor_adder.add_actor(
                ActorType::Explosion,
                p.general.position.x as u16,
                p.general.position.y as u16,
            );
        }
    }
}
