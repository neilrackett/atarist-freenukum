use super::geometry::Geometry;
use super::tilecache::TileCache;
use crate::{
    ANIMATION_CARBOT, ANIMATION_FOOTBOT, ANIMATION_WALLCRAWLERBOT_LEFT,
    ANIMATION_WALLCRAWLERBOT_RIGHT, HALFTILE_HEIGHT, HALFTILE_WIDTH,
    TILE_HEIGHT, TILE_WIDTH,
};
use transdl::video::Surface;

#[repr(C)]
pub enum BotType {
    FireWheel,
    FlameGnome,
    FlyingBot,
    FootBot,
    Helicopter,
    Rabbitoid,
    RedBallJumping,
    RedBallLying,
    SnakeBot,
    TankBot,
    WallCrawlerLeft,
    WallCrawlerRight,
    DrProton,
}

pub struct Bot {
    bot_type: BotType,
    x: usize,
    y: usize,
}

impl Bot {
    pub fn blit(&self, surface: &mut Surface, tilecache: &TileCache) {
        let mut dstrect = Geometry {
            x: self.x as i16 * HALFTILE_WIDTH as i16,
            y: self.y as i16 * HALFTILE_HEIGHT as i16,
            w: TILE_WIDTH as u16,
            h: TILE_HEIGHT as u16,
        };

        match self.bot_type {
            BotType::FireWheel => { /* TODO */ }
            BotType::FlameGnome => { /* TODO */ }
            BotType::FlyingBot => { /* TODO */ }
            BotType::FootBot => {
                let tile =
                    tilecache.get_tile(ANIMATION_FOOTBOT + 2).unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));

                dstrect.x += TILE_WIDTH as i16;

                let tile =
                    tilecache.get_tile(ANIMATION_FOOTBOT + 3).unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));

                dstrect.x -= TILE_WIDTH as i16;
                dstrect.y -= TILE_HEIGHT as i16;

                let tile =
                    tilecache.get_tile(ANIMATION_FOOTBOT + 0).unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));

                dstrect.x += TILE_WIDTH as i16;

                let tile =
                    tilecache.get_tile(ANIMATION_FOOTBOT + 1).unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));
            }
            BotType::Helicopter => { /* TODO */ }
            BotType::Rabbitoid => { /* TODO */ }
            BotType::RedBallJumping => { /* TODO */ }
            BotType::RedBallLying => { /* TODO */ }
            BotType::SnakeBot => { /* TODO */ }
            BotType::TankBot => {
                /* TODO */
                let tile =
                    tilecache.get_tile(ANIMATION_CARBOT + 0).unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));

                dstrect.x += TILE_WIDTH as i16;
                let tile =
                    tilecache.get_tile(ANIMATION_CARBOT + 1).unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));
            }
            BotType::WallCrawlerLeft => {
                let tile = tilecache
                    .get_tile(ANIMATION_WALLCRAWLERBOT_LEFT + 0)
                    .unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));
                /* TODO */
            }
            BotType::WallCrawlerRight => {
                let tile = tilecache
                    .get_tile(ANIMATION_WALLCRAWLERBOT_RIGHT + 0)
                    .unwrap();
                tile.blit_to_sdl_surface(None, surface, Some(dstrect));
                /* TODO */
            }
            BotType::DrProton => { /* TODO */ }
        }
    }
}

pub mod ffi {
    use super::super::tilecache::ffi::FnTileCache;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    pub type FnBotType = super::BotType;
    pub type FnBot = super::Bot;

    #[no_mangle]
    pub extern "C" fn fn_bot_create(
        bot_type: FnBotType,
        x: usize,
        y: usize,
    ) -> *mut FnBot {
        Box::into_raw(Box::new(FnBot { bot_type, x, y }))
    }

    #[no_mangle]
    pub extern "C" fn fn_bot_free(ptr: *mut FnBot) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_bot_blit(
        bot: *const FnBot,
        surface: *mut SDL_Surface,
        tilecache: *const FnTileCache,
    ) {
        assert!(!bot.is_null());
        assert!(!surface.is_null());
        assert!(!tilecache.is_null());

        let bot = unsafe { &(*bot) };
        let mut surface = Surface { raw: surface };
        let tilecache = unsafe { &(*tilecache) };
        bot.blit(&mut surface, tilecache);
    }

    #[no_mangle]
    pub extern "C" fn fn_bot_get_x(bot: *const FnBot) -> usize {
        assert!(!bot.is_null());
        let bot = unsafe { &(*bot) };
        bot.x
    }

    #[no_mangle]
    pub extern "C" fn fn_bot_get_y(bot: *const FnBot) -> usize {
        assert!(!bot.is_null());
        let bot = unsafe { &(*bot) };
        bot.y
    }
}
