use super::menu::{Menu, MenuEntry};
use super::texture::TextureCreationParams;
use super::tilecache::TileCache;
use anyhow::Result;
use std::convert::Into;
use transdl::video::Surface;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
            _ => M::Invalid,
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
) -> Result<MainMenuEntry> {
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

    Ok(MainMenuEntry::from(menu.get_choice(
        screen,
        tilecache,
        texture_creation_paramns,
    )?))
}
