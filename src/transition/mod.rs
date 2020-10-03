pub mod backdrop;
pub mod borders;
pub mod bot;
pub mod file;
pub mod geometry;
pub mod infobox;
pub mod inputbox;
pub mod inputfield;
pub mod level;
pub mod mainmenu;
pub mod menu;
pub mod messagebox;
pub mod picture;
pub mod settings;
pub mod text;
pub mod texture;
pub mod tile;
pub mod tilecache;

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
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

pub mod ffi {
    pub type FnHorizontalDirection = super::HorizontalDirection;
    pub type FnVerticalDirection = super::VerticalDirection;

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
}
