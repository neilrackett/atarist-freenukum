use crate::{
    actor::{
        ActParameters, Actor, CreateActorWithDetails, RenderParameters,
    },
    level::{solids::LevelSolids, tiles::LevelTiles},
    Result, Sizes, OBJECT_SPARK_BLUE, OBJECT_SPARK_GREEN,
    OBJECT_SPARK_PINK, OBJECT_SPARK_WHITE,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleColor {
    Pink,
    Blue,
    White,
    Green,
}

#[derive(Debug)]
pub(crate) struct Particle {
    tile: usize,
    countdown: usize,
    hspeed: f32,
    vspeed: f32,
    position: Rect,
    is_alive: bool,
}

impl CreateActorWithDetails for Particle {
    type Details = ParticleColor;

    fn create_with_details(
        color: ParticleColor,
        pos: Point,
        sizes: &dyn Sizes,
        _solids: &mut LevelSolids,
        _tiles: &mut LevelTiles,
    ) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let vspeed = rng.gen_range(-0.75f32..0.3f32);
        let hspeed = rng.gen_range(-0.5f32..=0.5f32);

        let tile = match color {
            ParticleColor::Pink => OBJECT_SPARK_PINK,
            ParticleColor::Blue => OBJECT_SPARK_BLUE,
            ParticleColor::White => OBJECT_SPARK_WHITE,
            ParticleColor::Green => OBJECT_SPARK_GREEN,
        };

        Self {
            tile,
            countdown: 20,
            hspeed,
            vspeed,
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.half_width(),
                sizes.half_height(),
            ),
            is_alive: true,
        }
    }
}

impl Actor for Particle {
    fn act(&mut self, p: ActParameters) {
        if self.countdown > 0 {
            self.countdown -= 1;
            let vspeed = (p.sizes.height() as f32 * self.vspeed) as i32;
            let hspeed = (p.sizes.width() as f32 * self.hspeed) as i32;
            self.position.offset(hspeed, vspeed);
            self.vspeed += 0.1f32;
        } else {
            self.is_alive = false;
        }
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(self.tile, self.position.top_left())?;
        Ok(())
    }

    fn position(&self) -> Rect {
        self.position
    }

    fn is_in_foreground(&self) -> bool {
        true
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn acts_while_invisible(&self) -> bool {
        true
    }
}
