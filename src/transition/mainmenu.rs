use super::geometry::Geometry;
use super::menu::{Menu, MenuEntry};
use super::messagebox;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::{FONT_HEIGHT, FONT_WIDTH, OBJECT_POINT};
use std::convert::{Into, TryFrom};
use transdl::event::{Event, KeyCode, MouseButton};
use transdl::video::Surface;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum MainMenuEntry {
    Start,
    Restore,
    Instructions,
    OrderingInfo,
    FullScreenToggle,
    EpisodeChange,
    HighScores,
    Previews,
    ViewUserDemo,
    TitleScreen,
    Credits,
    Quit,
    Invalid,
}

impl From<char> for MainMenuEntry {
    fn from(c: char) -> MainMenuEntry {
        use MainMenuEntry as M;
        match c {
            'S' | 's' => M::Start,
            'R' | 'r' => M::Restore,
            'I' | 'i' => M::Instructions,
            'O' | 'o' => M::OrderingInfo,
            'F' | 'f' => M::FullScreenToggle,
            'E' | 'e' => M::EpisodeChange,
            'H' | 'h' => M::HighScores,
            'P' | 'p' => M::Previews,
            'V' | 'v' => M::ViewUserDemo,
            'T' | 't' => M::TitleScreen,
            'C' | 'c' => M::Credits,
            'Q' | 'q' => M::Quit,
            c => M::Invalid,
        }
    }
}

impl Into<char> for MainMenuEntry {
    fn into(self) -> char {
        use MainMenuEntry as M;
        match self {
            M::Start => 's',
            M::Restore => 'r',
            M::Instructions => 'i',
            M::OrderingInfo => 'o',
            M::FullScreenToggle => 'f',
            M::EpisodeChange => 'e',
            M::HighScores => 'h',
            M::Previews => 'p',
            M::ViewUserDemo => 'v',
            M::TitleScreen => 't',
            M::Credits => 'c',
            M::Quit => 'q',
            M::Invalid => '\0',
        }
    }
}

impl Into<MenuEntry> for MainMenuEntry {
    fn into(self) -> MenuEntry {
        use MainMenuEntry as M;
        let shortcut: char = self.into();
        let name = match self {
            M::Start => "S)tart a new game",
            M::Restore => "R)estore an old game",
            M::Instructions => "I)nstructions",
            M::OrderingInfo => "O)rdering information",
            M::FullScreenToggle => "F)ullscreen toggle",
            M::EpisodeChange => "E)pisode change",
            M::HighScores => "H)igh score",
            M::Previews => "P)reviews / Main Demo!",
            M::ViewUserDemo => "V)iew user demo",
            M::TitleScreen => "T)itle screen",
            M::Credits => "C)redits",
            M::Quit => "Q)uit to OS",
            M::Invalid => "",
        }
        .to_string();
        MenuEntry { shortcut, name }
    }
}

pub fn mainmenu(
    screen: &mut Surface,
    tilecache: &TileCache,
    texture_creation_paramns: TextureCreationParams,
) -> MainMenuEntry {
    use MainMenuEntry as M;
    let msg = r"
  FREENUKUM MAIN MENU 
  -------------------";

    let mut menu = Menu::new(msg.to_string());
    menu.append(M::Start.into());
    menu.append(M::Restore.into());
    menu.append(M::Instructions.into());
    menu.append(M::OrderingInfo.into());
    menu.append(M::FullScreenToggle.into());
    menu.append(M::EpisodeChange.into());
    menu.append(M::HighScores.into());
    menu.append(M::Previews.into());
    menu.append(M::ViewUserDemo.into());
    menu.append(M::TitleScreen.into());
    menu.append(M::Credits.into());
    menu.append(M::Quit.into());

    MainMenuEntry::from(menu.get_choice(
        screen,
        tilecache,
        texture_creation_paramns,
    ))
}

pub mod ffi {
    use super::super::texture::ffi::FnTextureCreationParams;
    use super::super::tilecache::ffi::FnTileCache;
    use super::mainmenu;
    use transdl::ll::SDL_Surface;
    use transdl::video::Surface;

    pub type FnMainMenuEntry = super::MainMenuEntry;

    #[no_mangle]
    pub extern "C" fn fn_mainmenu(
        screen: *mut SDL_Surface,
        tilecache: *const FnTileCache,
        texture_creation_paramns: FnTextureCreationParams,
    ) -> FnMainMenuEntry {
        assert!(!screen.is_null());
        assert!(!tilecache.is_null());

        let mut screen = Surface { raw: screen };
        let tilecache = unsafe { &(*tilecache) };

        mainmenu(&mut screen, tilecache, texture_creation_paramns)
    }
}
