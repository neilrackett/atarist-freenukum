use crate::{
    actor::{
        ActParameters, Actor, ActorExt, ActorMessageType, CreateActor,
        ReceiveMessageParameters, RenderParameters,
    },
    level::{tiles::LevelTiles, BackgroundTileStrategy},
    sound::SoundIndex,
    Result, Sizes, OBJECT_LASERBEAM,
};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub(crate) struct AccessCardDoor {
    tile: usize,
    current_frame: usize,
    num_frames: usize,
    position: Rect,
    is_alive: bool,
}

impl CreateActor for AccessCardDoor {
    fn create(
        pos: Point,
        sizes: &dyn Sizes,
        _tiles: &mut LevelTiles,
    ) -> Actor {
        Actor::AccessCardDoor(Self {
            tile: OBJECT_LASERBEAM,
            current_frame: 0,
            num_frames: 4,
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

impl ActorExt for AccessCardDoor {
    fn act(&mut self, p: ActParameters) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
        p.game_commands.add_sound(SoundIndex::FORCEFIELD);
    }

    fn render(&mut self, p: RenderParameters) -> Result<()> {
        p.renderer.place_tile(
            self.tile + self.current_frame,
            self.position.top_left(),
        )?;
        Ok(())
    }

    fn can_receive_message(&self, message: ActorMessageType) -> bool {
        message == ActorMessageType::OpenDoorAccessCard
    }

    fn receive_message(&mut self, p: ReceiveMessageParameters) {
        let x = self.position.x() / p.sizes.width() as i32;
        let y = self.position.y() / p.sizes.height() as i32;
        if let Ok(ref mut t) = p.tiles.get_mut(x, y) {
            t.solid = false;
        }
        self.is_alive = false;
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
        BackgroundTileStrategy::CopyFromLeft
    }
}
