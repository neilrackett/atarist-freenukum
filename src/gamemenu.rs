// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::menu::{Menu, MenuEntry};
use crate::TileProvider;
use anyhow::Result;
use sdl2::{
    event::EventSender, render::WindowCanvas, EventPump, TimerSubsystem,
};
use std::convert::Into;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameMenuEntry {
    Save,
    Instructions,
    GameSetup,
    HighScores,
    RestartLevel,
    Invalid,
}

impl From<char> for GameMenuEntry {
    fn from(c: char) -> GameMenuEntry {
        use GameMenuEntry as M;
        match c {
            'S' | 's' => M::Save,
            'I' | 'i' => M::Instructions,
            'G' | 'g' => M::GameSetup,
            'H' | 'h' => M::HighScores,
            'R' | 'r' => M::RestartLevel,
            _ => M::Invalid,
        }
    }
}

impl Into<char> for GameMenuEntry {
    fn into(self) -> char {
        use GameMenuEntry as M;
        match self {
            M::Save => 's',
            M::Instructions => 'i',
            M::GameSetup => 'g',
            M::HighScores => 'h',
            M::RestartLevel => 'r',
            M::Invalid => '\0',
        }
    }
}

impl Into<MenuEntry> for GameMenuEntry {
    fn into(self) -> MenuEntry {
        use GameMenuEntry as M;
        let shortcut: char = self.into();
        let name = match self {
            M::Save => "S)ave a game",
            M::Instructions => "I)nstructions",
            M::GameSetup => "G)ame Setup",
            M::HighScores => "H)igh scores",
            M::RestartLevel => "R)estart Level",
            M::Invalid => "",
        }
        .to_string();
        MenuEntry { shortcut, name }
    }
}

pub fn gamemenu(
    canvas: &mut WindowCanvas,
    tileprovider: &dyn TileProvider,
    event_pump: &mut EventPump,
    event_sender: &EventSender,
    timer_subsystem: &TimerSubsystem,
) -> Result<GameMenuEntry> {
    use GameMenuEntry as M;
    let msg = r"
  FREENUKUM HELP MENU 
  -------------------";

    let mut menu = Menu::new(msg.to_string());
    menu.append(M::Save.into());
    menu.append(M::Instructions.into());
    menu.append(M::GameSetup.into());
    menu.append(M::HighScores.into());
    menu.append(M::RestartLevel.into());

    Ok(GameMenuEntry::from(menu.get_choice(
        canvas,
        tileprovider,
        event_pump,
        event_sender,
        timer_subsystem,
    )?))
}
