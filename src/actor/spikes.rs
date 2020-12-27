use crate::{
    actor::{
        ActParameters, Actor, CreateActorWithDetails,
        RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Hero, Result, OBJECT_SPIKE, OBJECT_SPIKES_DOWN, OBJECT_SPIKES_UP,
    TILE_HEIGHT, TILE_WIDTH,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct Specific {
    touching_hero: bool,
    position: Rect,
    spike_type: SpikeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpikeType {
    SpikesUp,
    SpikesDown,
    SingleSpikeUp,
}

impl CreateActorWithDetails for Specific {
    type Details = SpikeType;

    fn create_with_details(
        spike_type: SpikeType,
        pos: Point,
        _solids: &mut LevelSolids,
        tiles: &mut LevelTiles,
    ) -> Specific {
        let x = pos.x as u32 / TILE_WIDTH;
        let y = pos.y as u32 / TILE_HEIGHT;

        match spike_type {
            SpikeType::SpikesUp | SpikeType::SingleSpikeUp => {
                tiles.copy_from_to(x, y - 1, x, y);
            }
            SpikeType::SpikesDown => {
                tiles.copy_from_to(x, y + 1, x, y);
            }
        }

        Specific {
            touching_hero: false,
            position: Rect::new(pos.x, pos.y, TILE_WIDTH, TILE_HEIGHT),
            spike_type,
        }
    }
}

impl Actor for Specific {
    fn act(&mut self, p: ActParameters) {
        self.touching_hero =
            self.position.has_intersection(p.hero.position.geometry);
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        let tile = match self.spike_type {
            SpikeType::SpikesUp => OBJECT_SPIKES_UP,
            SpikeType::SpikesDown => OBJECT_SPIKES_DOWN,
            SpikeType::SingleSpikeUp if self.touching_hero => {
                OBJECT_SPIKE + 1
            }
            SpikeType::SingleSpikeUp => OBJECT_SPIKE,
        };
        p.renderer.place_tile(tile, self.position.top_left())?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn hurts_hero(&self, hero: &Hero) -> bool {
        self.position.has_intersection(hero.position.geometry)
    }
}
