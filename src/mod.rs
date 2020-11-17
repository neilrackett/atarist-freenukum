pub mod backdrop;
pub mod borders;
pub mod data;
pub mod episodes;
pub mod file;
pub mod game;
pub mod geometry;
pub mod graphics;
pub mod hero;
pub mod infobox;
pub mod inputbox;
pub mod inputfield;
pub mod level;
pub mod mainmenu;
pub mod menu;
pub mod messagebox;
pub mod picture;
pub mod settings;
pub mod shot;
pub mod text;
pub mod texture;
pub mod tile;
pub mod tilecache;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub enum HorizontalDirection {
    Center,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
pub enum VerticalDirection {
    Center,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UserEvent {
    Timer,
    HeroMoved,
    HeroScored,
    HeroFirepowerChanged,
    HeroInventoryChanged,
    HeroHealthChanged,
    HeroLanded,
}

pub fn sdl_surface_transparent(surface: &transdl::video::Surface) -> u32 {
    transdl::video::map_rgb(&surface.format(), 100, 1, 1)
}

pub fn sdl_surface_creation_params(
    surface: &transdl::video::Surface,
) -> texture::TextureCreationParams {
    texture::TextureCreationParams {
        flags: surface.flags(),
        bits_per_pixel: surface.format().bits_per_pixel(),
        transparent: sdl_surface_transparent(surface),
    }
}
