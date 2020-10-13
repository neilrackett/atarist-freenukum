use super::geometry::Geometry;
use super::hero::{Firepower, Inventory, InventoryItem};
use super::text;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::{
    BORDER_GREY_START, FONT_HEIGHT, FONT_WIDTH, HALFTILE_HEIGHT,
    HALFTILE_WIDTH, MAX_FIREPOWER, MAX_LIFE, OBJECT_ACCESS_CARD,
    OBJECT_BOOT, OBJECT_CLAMP, OBJECT_GLOVE, OBJECT_GUN, OBJECT_HEALTH,
    OBJECT_KEY_BLUE, OBJECT_KEY_GREEN, OBJECT_KEY_PINK, OBJECT_KEY_RED,
    OBJECT_NONHEALTH, OBJECT_SHOT, SCORE_DIGITS, TILE_HEIGHT, TILE_WIDTH,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};
use transdl::video::Surface;

pub struct Borders {}

#[rustfmt::skip]
const BORDERS:
[i32; (2 * WINDOW_HEIGHT / TILE_HEIGHT) * (2 * WINDOW_WIDTH / TILE_WIDTH)] =
[
     4,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1, 2,-1,
     2,-1, 2,-1, 2,-1, 2,-1, 5,-1, 8,-1,38,-1,39,-1, 8,-1, 9,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,14,-1, 8,-1,36,-1,37,-1, 8,-1,15,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,14,-1, 8,33,-1,34,-1,35,-1, 8,15,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,14,-1, 8,30,-1,31,-1,32,-1, 8,15,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1,-1,-1,-1,10,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

     7,-1, 3,-1, 3,-1, 3, -1,3,-1, 3,26,-1,27,-1,28,-1,29,-1, 3,
     3,-1, 3,-1, 3,-1, 3,-1, 6,-1, 8,-1, 8,-1, 8,-1, 8,-1,11,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,

    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
];

impl Borders {
    fn blit_tile(
        &self,
        target: &mut Texture,
        tile: &Texture,
        x: i16,
        y: i16,
    ) {
        let geometry = Geometry {
            x,
            y,
            w: tile.width(),
            h: tile.height(),
        };

        tile.clone_to_texture(None, target, Some(geometry));
    }

    fn blit_iter<I>(
        &self,
        tilecache: &TileCache,
        target: &mut Texture,
        borders: I,
        columns: u8,
        grid_x: u16,
        grid_y: u16,
    ) where
        I: Iterator<Item = Option<usize>>,
    {
        for (i, border) in borders.enumerate() {
            if let Some(border) = border {
                self.blit_tile(
                    target,
                    tilecache.get_tile(border).unwrap(),
                    (i as i16 % columns as i16) * (grid_x as i16),
                    (i as i16 / columns as i16) * (grid_y as i16),
                )
            }
        }
    }

    pub fn blit(
        &self,
        screen: &mut Surface,
        texture_creation_params: TextureCreationParams,
        tilecache: &TileCache,
    ) {
        let mut texture = Texture::create_with_params(
            screen.width() as u16,
            screen.height() as u16,
            texture_creation_params,
        );

        self.blit_iter(
            tilecache,
            &mut texture,
            BORDERS.iter().map(|i| {
                if *i < 0 {
                    None
                } else {
                    Some(*i as usize + BORDER_GREY_START)
                }
            }),
            (2 * WINDOW_WIDTH / TILE_WIDTH) as u8,
            HALFTILE_WIDTH as u16,
            HALFTILE_HEIGHT as u16,
        );
        texture.blit_to_sdl_surface(None, screen, None);
    }

    pub fn blit_life(
        &self,
        screen: &mut Surface,
        texture_creation_params: TextureCreationParams,
        tilecache: &TileCache,
        health: u8,
    ) {
        let health = std::cmp::min(health as usize, MAX_LIFE);
        let mut lifesurface = Texture::create_with_params(
            HALFTILE_WIDTH as u16 * MAX_LIFE as u16,
            TILE_HEIGHT as u16,
            texture_creation_params,
        );
        let iter = (0..MAX_LIFE).map(|i| {
            if i < health {
                Some(OBJECT_HEALTH)
            } else {
                Some(OBJECT_NONHEALTH)
            }
        });
        self.blit_iter(
            tilecache,
            &mut lifesurface,
            iter,
            MAX_LIFE as u8,
            HALFTILE_WIDTH as u16,
            HALFTILE_HEIGHT as u16,
        );
        let destrect = Geometry {
            x: 30 * HALFTILE_WIDTH as i16,
            y: (15 * HALFTILE_HEIGHT as i16) / 2,
            w: MAX_LIFE as u16 * HALFTILE_WIDTH as u16,
            h: TILE_HEIGHT as u16,
        };
        lifesurface.blit_to_sdl_surface(None, screen, Some(destrect));
    }

    pub fn blit_score(
        &self,
        screen: &mut Surface,
        texture_creation_params: TextureCreationParams,
        tilecache: &TileCache,
        score: u128,
    ) {
        let score = std::cmp::min(99999999, score);
        let score_string = format!("{:08}", score);

        let mut scoresurface = Texture::create_with_params(
            FONT_WIDTH as u16 * SCORE_DIGITS as u16,
            FONT_HEIGHT as u16,
            texture_creation_params,
        );
        let sourcerect = Geometry {
            x: 0,
            y: 0,
            w: FONT_WIDTH as u16 * SCORE_DIGITS as u16,
            h: FONT_HEIGHT as u16,
        };
        text::print(
            &mut scoresurface,
            sourcerect,
            tilecache,
            &score_string,
        );
        let destrect = Geometry {
            x: 30 * FONT_WIDTH as i16,
            y: 3 * FONT_HEIGHT as i16,
            w: SCORE_DIGITS as u16 * FONT_WIDTH as u16,
            h: FONT_HEIGHT as u16,
        };
        scoresurface.blit_to_sdl_surface(None, screen, Some(destrect));
    }

    pub fn blit_firepower(
        &self,
        screen: &mut Surface,
        texture_creation_params: TextureCreationParams,
        tilecache: &TileCache,
        firepower: &Firepower,
    ) {
        const GUN: Option<usize> = Some(OBJECT_GUN);
        const SHOT: Option<usize> = Some(OBJECT_SHOT);

        let shots = firepower.num_shots();

        let shot0 = if shots > 0 { SHOT } else { None };
        let shot1 = if shots > 1 { SHOT } else { None };
        let shot2 = if shots > 2 { SHOT } else { None };
        let shot3 = if shots > 3 { SHOT } else { None };

        #[rustfmt::skip]
        let tiles = vec![
            None,  None, None,  GUN,  None,  None, None,  None,
            None,  None, None,  None, None,  None, None,  None,
            shot0, None, shot1, None, shot2, None, shot3, None,
            None,  None, None,  None, None,  None, None,  None,
        ];

        let mut shotsurface = Texture::create_with_params(
            TILE_WIDTH as u16 * MAX_FIREPOWER as u16,
            TILE_HEIGHT as u16 * 2,
            texture_creation_params,
        );

        self.blit_iter(
            tilecache,
            &mut shotsurface,
            tiles.into_iter(),
            MAX_FIREPOWER as u8 * 2,
            HALFTILE_WIDTH as u16,
            HALFTILE_HEIGHT as u16,
        );
        let destrect = Geometry {
            x: 15 * TILE_WIDTH as i16,
            y: 6 * TILE_HEIGHT as i16,
            w: MAX_FIREPOWER as u16 * TILE_WIDTH as u16,
            h: TILE_HEIGHT as u16 * 2,
        };
        shotsurface.blit_to_sdl_surface(None, screen, Some(destrect));
    }

    pub fn blit_inventory(
        &self,
        screen: &mut Surface,
        texture_creation_params: TextureCreationParams,
        tilecache: &TileCache,
        inventory: &Inventory,
    ) {
        let red_key = if inventory.is_set(InventoryItem::KeyRed) {
            Some(OBJECT_KEY_RED)
        } else {
            None
        };
        let green_key = if inventory.is_set(InventoryItem::KeyGreen) {
            Some(OBJECT_KEY_GREEN)
        } else {
            None
        };
        let blue_key = if inventory.is_set(InventoryItem::KeyBlue) {
            Some(OBJECT_KEY_BLUE)
        } else {
            None
        };
        let pink_key = if inventory.is_set(InventoryItem::KeyPink) {
            Some(OBJECT_KEY_PINK)
        } else {
            None
        };
        let boot = if inventory.is_set(InventoryItem::Boot) {
            Some(OBJECT_BOOT)
        } else {
            None
        };
        let glove = if inventory.is_set(InventoryItem::Glove) {
            Some(OBJECT_GLOVE)
        } else {
            None
        };
        let clamp = if inventory.is_set(InventoryItem::Clamp) {
            Some(OBJECT_CLAMP)
        } else {
            None
        };
        let access_card = if inventory.is_set(InventoryItem::AccessCard) {
            Some(OBJECT_ACCESS_CARD)
        } else {
            None
        };

        #[rustfmt::skip]
        let tiles = vec![
            red_key, green_key, blue_key, pink_key,
            boot, glove, clamp, access_card,
        ];

        let mut inventorysurface = Texture::create_with_params(
            TILE_WIDTH as u16 * 4,
            TILE_HEIGHT as u16 * 2,
            texture_creation_params,
        );

        self.blit_iter(
            tilecache,
            &mut inventorysurface,
            tiles.into_iter(),
            4,
            TILE_WIDTH as u16,
            TILE_HEIGHT as u16,
        );
        let destrect = Geometry {
            x: 15 * TILE_WIDTH as i16,
            y: 9 * TILE_HEIGHT as i16,
            w: 4 * TILE_WIDTH as u16,
            h: 2 * TILE_HEIGHT as u16,
        };
        inventorysurface.blit_to_sdl_surface(None, screen, Some(destrect));
    }
}

pub mod ffi {
    use super::super::hero::ffi::{FnHeroFirepower, FnHeroInventory};
    use super::super::texture::ffi::FnTextureCreationParams;
    use super::super::tilecache::ffi::FnTileCache;
    use super::Borders;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    #[no_mangle]
    pub extern "C" fn fn_borders_blit(
        screen: *mut SDL_Surface,
        params: FnTextureCreationParams,
        tilecache: *const FnTileCache,
    ) {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };

        let borders = Borders {};

        borders.blit(&mut screen, params, tilecache);
    }

    #[no_mangle]
    pub extern "C" fn fn_borders_blit_life(
        screen: *mut SDL_Surface,
        params: FnTextureCreationParams,
        tilecache: *const FnTileCache,
        health: u8,
    ) {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };

        let borders = Borders {};

        borders.blit_life(&mut screen, params, tilecache, health);
    }

    #[no_mangle]
    pub extern "C" fn fn_borders_blit_score(
        screen: *mut SDL_Surface,
        params: FnTextureCreationParams,
        tilecache: *const FnTileCache,
        score: usize,
    ) {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };

        let borders = Borders {};

        borders.blit_score(&mut screen, params, tilecache, score as u128);
    }

    #[no_mangle]
    pub extern "C" fn fn_borders_blit_firepower(
        screen: *mut SDL_Surface,
        params: FnTextureCreationParams,
        tilecache: *const FnTileCache,
        firepower: *const FnHeroFirepower,
    ) {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };
        let firepower = unsafe { &(*firepower) };

        let borders = Borders {};

        borders.blit_firepower(&mut screen, params, tilecache, firepower);
    }

    #[no_mangle]
    pub extern "C" fn fn_borders_blit_inventory(
        screen: *mut SDL_Surface,
        params: FnTextureCreationParams,
        tilecache: *const FnTileCache,
        inventory: *const FnHeroInventory,
    ) {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());
        assert!(!inventory.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };
        let inventory = unsafe { &(*inventory) };

        let borders = Borders {};

        borders.blit_inventory(&mut screen, params, tilecache, inventory);
    }
}
