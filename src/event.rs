// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    hero::InventoryItem, HorizontalDirection, KeyColor, UserEvent,
    HALFTILE_HEIGHT, HALFTILE_WIDTH,
};
use anyhow::{anyhow, Error, Result};
use sdl2::{
    controller::{Axis, Button},
    event::{Event, WindowEvent},
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
    EventPump,
};
use std::{collections::BTreeSet, convert::TryFrom, iter::FromIterator};

#[must_use]
pub enum GameEvent {
    Escape,
    GameMenu,
    GetInventoryItem(InventoryItem),
    IncreaseLife,
    FinishLevel,
    ToggleFullscreen,
    MoveViewPoint {
        x: i32,
        y: i32,
    },
    HeroInteractionStart,
    HeroInteractionEnd,
    HeroSetWalkingDirectionEnabled {
        directions: BTreeSet<HorizontalDirection>,
        context: InputContext,
        enabled: bool,
    },
    RefreshScreen,
    HeroJump,
    HeroStartFiring,
    HeroStopFiring,
    TimerTriggered,
}

#[must_use]
pub enum ConfirmEvent {
    Confirmed,
    Aborted,
    RefreshScreen,
}

#[must_use]
pub enum InputEvent {
    DeleteLeft,
    DeleteRight,
    MoveCursorLeft,
    MoveCursorRight,
    Confirm,
    Abort,
    Letter(char),
    RefreshScreen,
}

#[must_use]
pub enum MenuEvent {
    ChooseCurrentEntry,
    Abort,
    NextEntry {
        context: InputContext,
        enabled: bool,
    },
    PreviousEntry {
        context: InputContext,
        enabled: bool,
    },
    ChooseShortcutEntry(char),
    MoveMouse {
        x: i32,
        y: i32,
    },
    ClickMouse,
    RefreshScreen,
    TimerTriggered,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputContext {
    ControllerDPad,
    ControllerAxis,
    Keyboard,
}

pub trait OnOffTracking {
    type Context;

    fn set_enabled(
        &mut self,
        context: Self::Context,
        enable: bool,
        off_to_on: &mut dyn FnMut(),
        on_to_off: &mut dyn FnMut(),
    );
}

impl OnOffTracking for BTreeSet<InputContext> {
    type Context = InputContext;

    fn set_enabled(
        &mut self,
        context: Self::Context,
        enable: bool,
        off_to_on: &mut dyn FnMut(),
        on_to_off: &mut dyn FnMut(),
    ) {
        let was_enabled = !self.is_empty();
        if enable {
            self.insert(context);
        } else {
            self.remove(&context);
        }
        let is_enabled = !self.is_empty();

        match (was_enabled, is_enabled) {
            (false, true) => off_to_on(),
            (true, false) => on_to_off(),
            (false, false) | (true, true) => {}
        }
    }
}

pub trait WaitEvent: Sized {
    fn wait(event_pump: &mut EventPump) -> Result<Self>;
}

impl<T: TryFrom<Event>> WaitEvent for T {
    fn wait(event_pump: &mut EventPump) -> Result<Self> {
        loop {
            let event = event_pump.wait_event();
            if let Ok(e) = T::try_from(event) {
                return Ok(e);
            }
        }
    }
}

impl TryFrom<Event> for GameEvent {
    type Error = Error;

    fn try_from(e: Event) -> Result<GameEvent> {
        use Button as B;
        use Event as E;
        use Keycode as K;
        use WindowEvent as W;
        match e {
            E::KeyDown {
                keycode: Some(K::F1),
                ..
            } => Ok(GameEvent::GameMenu),
            E::Quit { .. }
            | E::KeyDown {
                keycode: Some(K::Escape),
                ..
            }
            | E::KeyDown {
                keycode: Some(K::Q),
                ..
            }
            | E::ControllerButtonDown {
                button: B::Start, ..
            } => Ok(GameEvent::Escape),
            E::KeyDown {
                keycode: Some(K::Num1),
                ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Key(
                KeyColor::Red,
            ))),
            E::KeyDown {
                keycode: Some(K::Num2),
                ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Key(
                KeyColor::Green,
            ))),
            E::KeyDown {
                keycode: Some(K::Num3),
                ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Key(
                KeyColor::Blue,
            ))),
            E::KeyDown {
                keycode: Some(K::Num4),
                ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Key(
                KeyColor::Pink,
            ))),
            E::KeyDown {
                keycode: Some(K::Num5),
                ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Boot)),
            E::KeyDown {
                keycode: Some(K::Num6),
                ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Glove)),
            E::KeyDown {
                keycode: Some(K::Num7),
                ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Clamp)),
            E::KeyDown {
                keycode: Some(K::Num8),
                ..
            } => {
                Ok(GameEvent::GetInventoryItem(InventoryItem::AccessCard))
            }
            E::KeyDown {
                keycode: Some(K::Num9),
                ..
            } => Ok(GameEvent::IncreaseLife),
            E::KeyDown {
                keycode: Some(K::Num0),
                ..
            } => Ok(GameEvent::FinishLevel),
            E::KeyDown {
                keycode: Some(K::F),
                ..
            }
            | E::KeyDown {
                keycode: Some(K::F11),
                ..
            } => Ok(GameEvent::ToggleFullscreen),
            E::KeyDown {
                keycode: Some(K::Down),
                keymod,
                ..
            } if keymod.contains(Mod::LSHIFTMOD)
                || keymod.contains(Mod::RSHIFTMOD) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: HALFTILE_HEIGHT as i32,
                    y: 0,
                })
            }
            E::KeyDown {
                keycode: Some(K::Up),
                keymod,
                ..
            } if keymod.contains(Mod::LSHIFTMOD)
                || keymod.contains(Mod::RSHIFTMOD) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: -(HALFTILE_HEIGHT as i32),
                    y: 0,
                })
            }
            E::KeyDown {
                keycode: Some(K::Right),
                keymod,
                ..
            } if keymod.contains(Mod::LSHIFTMOD)
                || keymod.contains(Mod::RSHIFTMOD) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: 0,
                    y: HALFTILE_WIDTH as i32,
                })
            }
            E::KeyDown {
                keycode: Some(K::Left),
                keymod,
                ..
            } if keymod.contains(Mod::LSHIFTMOD)
                || keymod.contains(Mod::RSHIFTMOD) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: 0,
                    y: -(HALFTILE_WIDTH as i32),
                })
            }
            E::KeyDown {
                keycode: Some(K::Up),
                ..
            }
            | E::ControllerButtonDown { button: B::Y, .. }
            | E::MouseButtonDown {
                mouse_btn: MouseButton::Middle,
                ..
            } => Ok(GameEvent::HeroInteractionStart),
            E::ControllerAxisMotion {
                axis: Axis::LeftX,
                value,
                ..
            } if value < -20000 => {
                Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                    directions: BTreeSet::from_iter(
                        Some(HorizontalDirection::Left).into_iter(),
                    ),
                    context: InputContext::ControllerAxis,
                    enabled: true,
                })
            }
            E::ControllerAxisMotion {
                axis: Axis::LeftX,
                value,
                ..
            } if value > 20000 => {
                Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                    directions: BTreeSet::from_iter(
                        Some(HorizontalDirection::Right).into_iter(),
                    ),
                    context: InputContext::ControllerAxis,
                    enabled: true,
                })
            }
            E::ControllerAxisMotion {
                axis: Axis::LeftX, ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    vec![
                        HorizontalDirection::Left,
                        HorizontalDirection::Right,
                    ]
                    .into_iter(),
                ),
                context: InputContext::ControllerAxis,
                enabled: false,
            }),
            E::KeyDown {
                keycode: Some(K::Right),
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Right).into_iter(),
                ),
                context: InputContext::Keyboard,
                enabled: true,
            }),
            E::ControllerButtonDown {
                button: B::DPadRight,
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Right).into_iter(),
                ),
                context: InputContext::ControllerDPad,
                enabled: true,
            }),
            E::KeyDown {
                keycode: Some(K::Left),
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Left).into_iter(),
                ),
                context: InputContext::Keyboard,
                enabled: true,
            }),
            E::ControllerButtonDown {
                button: B::DPadLeft,
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Left).into_iter(),
                ),
                context: InputContext::ControllerDPad,
                enabled: true,
            }),
            E::KeyDown {
                keycode: Some(K::LCtrl),
                ..
            }
            | E::ControllerButtonDown { button: B::A, .. }
            | E::MouseButtonDown {
                mouse_btn: MouseButton::Right,
                ..
            } => Ok(GameEvent::HeroJump),
            E::KeyDown {
                keycode: Some(K::LAlt),
                ..
            }
            | E::ControllerButtonDown {
                button: B::LeftShoulder,
                ..
            }
            | E::ControllerButtonDown {
                button: B::RightShoulder,
                ..
            }
            | E::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => Ok(GameEvent::HeroStartFiring),
            E::KeyUp {
                keycode: Some(K::Up),
                ..
            }
            | E::ControllerButtonUp { button: B::Y, .. }
            | E::MouseButtonUp {
                mouse_btn: MouseButton::Middle,
                ..
            } => Ok(GameEvent::HeroInteractionEnd),
            E::KeyUp {
                keycode: Some(K::Right),
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Right).into_iter(),
                ),
                context: InputContext::Keyboard,
                enabled: false,
            }),
            E::ControllerButtonUp {
                button: B::DPadRight,
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Right).into_iter(),
                ),
                context: InputContext::ControllerDPad,
                enabled: false,
            }),
            E::KeyUp {
                keycode: Some(K::Left),
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Left).into_iter(),
                ),
                context: InputContext::Keyboard,
                enabled: false,
            }),
            E::ControllerButtonUp {
                button: B::DPadLeft,
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled {
                directions: BTreeSet::from_iter(
                    Some(HorizontalDirection::Left).into_iter(),
                ),
                context: InputContext::ControllerDPad,
                enabled: false,
            }),
            E::KeyUp {
                keycode: Some(K::LAlt),
                ..
            }
            | E::ControllerButtonUp {
                button: B::LeftShoulder,
                ..
            }
            | E::ControllerButtonUp {
                button: B::RightShoulder,
                ..
            }
            | E::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => Ok(GameEvent::HeroStopFiring),
            E::Window {
                win_event: W::Exposed,
                ..
            }
            | E::Window {
                win_event: W::Shown,
                ..
            } => Ok(GameEvent::RefreshScreen),
            e if e.is_user_event() => {
                match e.as_user_event_type::<UserEvent>() {
                    Some(UserEvent::Timer) => {
                        Ok(GameEvent::TimerTriggered)
                    }
                    Some(UserEvent::Redraw) => {
                        Ok(GameEvent::RefreshScreen)
                    }
                    None => unreachable!("Unknown user event"),
                }
            }

            _ => Err(anyhow!("Event not handled")),
        }
    }
}

impl TryFrom<Event> for ConfirmEvent {
    type Error = Error;

    fn try_from(e: Event) -> Result<ConfirmEvent> {
        use Button as B;
        use Event as E;
        use Keycode as K;
        use WindowEvent as W;
        match e {
            E::KeyDown {
                keycode: Some(K::Return),
                ..
            }
            | E::ControllerButtonDown { button: B::A, .. } => {
                Ok(ConfirmEvent::Confirmed)
            }
            E::KeyDown {
                keycode: Some(K::Escape),
                ..
            }
            | E::ControllerButtonDown { button: B::B, .. }
            | E::Quit { .. }
            | E::Window {
                win_event: W::Close,
                ..
            } => Ok(ConfirmEvent::Aborted),
            E::Window {
                win_event: W::Exposed,
                ..
            }
            | E::Window {
                win_event: W::Shown,
                ..
            } => Ok(ConfirmEvent::RefreshScreen),
            _ => Err(anyhow!("Event not handled")),
        }
    }
}

impl TryFrom<Event> for InputEvent {
    type Error = Error;

    fn try_from(e: Event) -> Result<InputEvent> {
        use Button as B;
        use Event as E;
        use Keycode as K;
        use Mod as M;
        use WindowEvent as W;
        match e {
            E::ControllerButtonDown {
                button: B::DPadLeft,
                ..
            } => Ok(InputEvent::MoveCursorLeft),
            E::ControllerButtonDown {
                button: B::DPadRight,
                ..
            } => Ok(InputEvent::MoveCursorRight),
            E::ControllerButtonDown { button: B::A, .. } => {
                Ok(InputEvent::Confirm)
            }
            E::ControllerButtonDown { button: B::B, .. } => {
                Ok(InputEvent::Abort)
            }

            E::KeyDown {
                keycode, keymod, ..
            } => match keycode {
                Some(K::Backspace) => Ok(InputEvent::DeleteLeft),
                Some(K::Delete) => Ok(InputEvent::DeleteRight),
                Some(K::Left) => Ok(InputEvent::MoveCursorLeft),
                Some(K::Right) => Ok(InputEvent::MoveCursorRight),
                Some(K::Return) => Ok(InputEvent::Confirm),
                Some(K::Escape) => Ok(InputEvent::Abort),
                Some(code)
                    if code == K::Space
                        || code == K::Exclaim
                        || code == K::Quotedbl
                        || code == K::Hash
                        || code == K::Dollar
                        || code == K::Ampersand
                        || code == K::Quote
                        || code == K::LeftParen
                        || code == K::RightParen
                        || code == K::Asterisk
                        || code == K::Plus
                        || code == K::Comma
                        || code == K::Minus
                        || code == K::Period
                        || code == K::Slash
                        || code == K::Num0
                        || code == K::Num1
                        || code == K::Num2
                        || code == K::Num3
                        || code == K::Num4
                        || code == K::Num5
                        || code == K::Num6
                        || code == K::Num7
                        || code == K::Num8
                        || code == K::Num9
                        || code == K::Colon
                        || code == K::Semicolon
                        || code == K::Less
                        || code == K::Equals
                        || code == K::Greater
                        || code == K::Question
                        || code == K::At
                        || code == K::A
                        || code == K::B
                        || code == K::C
                        || code == K::D
                        || code == K::E
                        || code == K::F
                        || code == K::G
                        || code == K::H
                        || code == K::I
                        || code == K::J
                        || code == K::K
                        || code == K::L
                        || code == K::M
                        || code == K::N
                        || code == K::O
                        || code == K::P
                        || code == K::Q
                        || code == K::R
                        || code == K::S
                        || code == K::T
                        || code == K::U
                        || code == K::V
                        || code == K::W
                        || code == K::X
                        || code == K::Y
                        || code == K::Z =>
                {
                    let mut c = code as u8 as char;
                    if keymod.contains(M::LSHIFTMOD)
                        || keymod.contains(M::RSHIFTMOD)
                    {
                        c.make_ascii_uppercase();
                    }
                    Ok(InputEvent::Letter(c))
                }
                _ => Err(anyhow!("Event not handled")),
            },
            E::Window {
                win_event: W::Exposed,
                ..
            }
            | E::Window {
                win_event: W::Shown,
                ..
            } => Ok(InputEvent::RefreshScreen),
            _ => Err(anyhow!("Event not handled")),
        }
    }
}

impl TryFrom<Event> for MenuEvent {
    type Error = Error;

    fn try_from(e: Event) -> Result<MenuEvent> {
        use Button as B;
        use Event as E;
        use Keycode as K;
        use MouseButton as M;
        use WindowEvent as W;
        match e {
            E::KeyDown {
                keycode: Some(K::Return),
                ..
            }
            | E::ControllerButtonDown { button: B::A, .. } => {
                Ok(MenuEvent::ChooseCurrentEntry)
            }
            E::KeyDown {
                keycode: Some(K::Escape),
                ..
            }
            | E::ControllerButtonDown { button: B::B, .. } => {
                Ok(MenuEvent::Abort)
            }
            E::ControllerAxisMotion {
                axis: Axis::LeftY,
                value,
                ..
            } if value < 0 => Ok(MenuEvent::PreviousEntry {
                context: InputContext::ControllerAxis,
                enabled: value < -20000,
            }),
            E::ControllerAxisMotion {
                axis: Axis::LeftY,
                value,
                ..
            } if value > 0 => Ok(MenuEvent::NextEntry {
                context: InputContext::ControllerAxis,
                enabled: value > 20000,
            }),
            E::KeyDown {
                keycode: Some(K::Down),
                ..
            } => Ok(MenuEvent::NextEntry {
                context: InputContext::Keyboard,
                enabled: true,
            }),
            E::ControllerButtonDown {
                button: B::DPadDown,
                ..
            } => Ok(MenuEvent::NextEntry {
                context: InputContext::ControllerDPad,
                enabled: true,
            }),
            E::KeyUp {
                keycode: Some(K::Down),
                ..
            } => Ok(MenuEvent::NextEntry {
                context: InputContext::Keyboard,
                enabled: false,
            }),
            E::ControllerButtonUp {
                button: B::DPadDown,
                ..
            } => Ok(MenuEvent::NextEntry {
                context: InputContext::ControllerDPad,
                enabled: false,
            }),
            E::ControllerButtonUp {
                button: B::DPadUp, ..
            } => Ok(MenuEvent::PreviousEntry {
                context: InputContext::ControllerDPad,
                enabled: false,
            }),
            E::KeyDown {
                keycode: Some(K::Up),
                ..
            } => Ok(MenuEvent::PreviousEntry {
                context: InputContext::Keyboard,
                enabled: true,
            }),
            E::KeyUp {
                keycode: Some(K::Up),
                ..
            } => Ok(MenuEvent::PreviousEntry {
                context: InputContext::Keyboard,
                enabled: false,
            }),
            E::ControllerButtonDown {
                button: B::DPadUp, ..
            } => Ok(MenuEvent::PreviousEntry {
                context: InputContext::ControllerDPad,
                enabled: true,
            }),
            E::KeyDown {
                keycode: Some(key), ..
            } => {
                let c = key as u8 as char;
                Ok(MenuEvent::ChooseShortcutEntry(c))
            }
            E::MouseMotion { x, y, .. } => {
                Ok(MenuEvent::MoveMouse { x, y })
            }
            E::MouseButtonDown { mouse_btn, .. } => {
                if mouse_btn == M::Left {
                    Ok(MenuEvent::ClickMouse)
                } else {
                    Err(anyhow!("Event not handled"))
                }
            }
            E::Window {
                win_event: W::Exposed,
                ..
            }
            | E::Window {
                win_event: W::Shown,
                ..
            } => Ok(MenuEvent::RefreshScreen),
            e if e.is_user_event() => {
                if e.as_user_event_type::<UserEvent>()
                    == Some(UserEvent::Timer)
                {
                    Ok(MenuEvent::TimerTriggered)
                } else {
                    // Ignore other events
                    Err(anyhow!("Event not handled"))
                }
            }
            _ => {
                // Ignore other events
                Err(anyhow!("Event not handled"))
            }
        }
    }
}
