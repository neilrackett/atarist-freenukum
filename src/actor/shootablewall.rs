use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorType, CreateActor,
        RenderParameters, ShotParameters, ShotProcessing,
        SingleAnimationType,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Result, Sizes, BACKGROUND_LIGHT_GREY, SOLID_SHOOTABLE_WALL_BRICKS,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct ShootableWall {
    position: Rect,
    is_alive: bool,
}

impl CreateActor for ShootableWall {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::ShootableWall(Self {
            position: Rect::new(
                pos.x,
                pos.y,
                sizes.width(),
                sizes.height(),
            ),
            is_alive: true,
        })
    }
}

impl ActorExt for ShootableWall {
    fn act(&mut self, _p: ActParameters) {}

    fn can_get_shot(&self) -> bool {
        true
    }

    fn shot(&mut self, p: ShotParameters) -> ShotProcessing {
        p.hero.score.add(10);
        p.game_commands.add_actor(
            ActorType::SingleAnimation(SingleAnimationType::Explosion),
            self.position.top_left(),
        );
        p.game_commands.add_sound(SoundIndex::HITABREAKER);
        self.is_alive = false;
        if let Ok(ref mut t) = p.tiles.get_mut(
            self.position.x() / p.sizes.width() as i32,
            self.position.y() / p.sizes.height() as i32,
        ) {
            t.solid = false;
        }
        ShotProcessing::Absorb
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer
            .place_tile(BACKGROUND_LIGHT_GREY, self.position.top_left())?;
        p.renderer.place_tile(
            SOLID_SHOOTABLE_WALL_BRICKS,
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

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn background_tile_strategy(&self) -> BackgroundTileStrategy {
        BackgroundTileStrategy::SetTile(0x17e0 / 0x20)
    }
}
