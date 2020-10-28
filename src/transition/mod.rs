pub mod backdrop;
pub mod borders;
pub mod data;
pub mod episodes;
pub mod file;
pub mod game;
pub mod geometry;
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

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum HorizontalDirection {
    Center,
    Left,
    Right,
}

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub enum VerticalDirection {
    Center,
    Up,
    Down,
}

#[repr(C)]
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

pub fn sdl_surface_flags(fullscreen: bool) -> u32 {
    let init = if fullscreen {
        transdl::ll::SDL_FULLSCREEN
    } else {
        0u32
    };
    init | transdl::ll::SDL_HWSURFACE
        | transdl::ll::SDL_HWACCEL
        | transdl::ll::SDL_ANYFORMAT
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

pub fn sdl_create_screen(
    w: i32,
    h: i32,
    fullscreen: bool,
) -> transdl::video::Surface {
    let depth = 0;
    transdl::video::Surface::set_video_mode(
        w,
        h,
        depth,
        sdl_surface_flags(fullscreen),
    )
}

pub mod ffi {
    pub type FnHorizontalDirection = super::HorizontalDirection;
    pub type FnVerticalDirection = super::VerticalDirection;
    pub type FnUserEvent = super::UserEvent;

    // Just a placeholder so the enum gets exported
    // without extra creating a cbindgen config.
    #[no_mangle]
    pub extern "C" fn fn_horizontal_direction_print(
        direction: FnHorizontalDirection,
    ) {
        println!("{:?}", direction);
    }

    // Just a placeholder so the enum gets exported
    // without extra creating a cbindgen config.
    #[no_mangle]
    pub extern "C" fn fn_vertical_direction_print(
        direction: FnVerticalDirection,
    ) {
        println!("{:?}", direction);
    }

    // Just a placeholder so the enum gets exported
    // without extra creating a cbindgen config.
    #[no_mangle]
    pub extern "C" fn fn_user_event_print(e: FnUserEvent) {
        println!("{:?}", e);
    }

    #[no_mangle]
    pub extern "C" fn fn_sdl_surface_flags(fullscreen: bool) -> u32 {
        super::sdl_surface_flags(fullscreen)
    }

    #[no_mangle]
    pub extern "C" fn fn_sdl_surface_transparent(
        surface: *mut transdl::ll::SDL_Surface,
    ) -> u32 {
        assert!(!surface.is_null());
        let surface = transdl::video::Surface { raw: surface };
        super::sdl_surface_transparent(&surface)
    }

    #[no_mangle]
    pub extern "C" fn fn_sdl_surface_creation_params(
        surface: *mut transdl::ll::SDL_Surface,
    ) -> super::texture::ffi::FnTextureCreationParams {
        assert!(!surface.is_null());
        let surface = transdl::video::Surface { raw: surface };
        super::sdl_surface_creation_params(&surface)
    }

    #[no_mangle]
    pub extern "C" fn fn_sdl_create_screen(
        w: i32,
        h: i32,
        fullscreen: bool,
    ) -> *mut transdl::ll::SDL_Surface {
        let surface = super::sdl_create_screen(w, h, fullscreen);
        surface.raw
    }
}
