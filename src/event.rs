use crate::hero::InventoryItem;
use crate::HorizontalDirection;
use crate::UserEvent;
use crate::{HALFTILE_HEIGHT, HALFTILE_WIDTH};
use anyhow::{anyhow, Error, Result};
use std::convert::TryFrom;

#[must_use]
pub enum GameEvent {
    Escape,
    GetInventoryItem(InventoryItem),
    IncreaseLife,
    FinishLevel,
    ToggleFullscreen,
    MoveViewPoint { x: i32, y: i32 },
    HeroInteractionStart,
    HeroInteractionEnd,
    HeroSetWalkingDirectionEnabled((HorizontalDirection, bool)),
    RefreshScreen,
    HeroJump,
    HeroStartFiring,
    HeroStopFiring,
    TimerTriggered,
    HeroMoved,
    HeroScored,
    HeroFirepowerChanged,
    HeroInventoryChanged,
    HeroHealthChanged,
    HeroLanded,
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

pub trait WaitEvent: Sized {
    fn wait() -> Result<Self>;
}

impl<T: TryFrom<transdl::event::Event>> WaitEvent for T {
    fn wait() -> Result<Self> {
        loop {
            let event = transdl::event::Event::wait()
                .map_err(|e| anyhow!("{}", e))?;
            if let Ok(e) = T::try_from(event) {
                return Ok(e);
            }
        }
    }
}

impl TryFrom<transdl::event::Event> for GameEvent {
    type Error = Error;

    fn try_from(e: transdl::event::Event) -> Result<GameEvent> {
        use transdl::event::{
            Event as E, KeyCode as K, Modifier as M, MouseButton,
        };
        match e {
            E::Quit
            | E::KeyDown {
                key: Some(K::Escape),
                ..
            }
            | E::KeyDown {
                key: Some(K::Q), ..
            } => Ok(GameEvent::Escape),
            E::KeyDown {
                key: Some(K::Num1), ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::KeyRed)),
            E::KeyDown {
                key: Some(K::Num2), ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::KeyGreen)),
            E::KeyDown {
                key: Some(K::Num3), ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::KeyBlue)),
            E::KeyDown {
                key: Some(K::Num4), ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::KeyPink)),
            E::KeyDown {
                key: Some(K::Num5), ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Boot)),
            E::KeyDown {
                key: Some(K::Num6), ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Glove)),
            E::KeyDown {
                key: Some(K::Num7), ..
            } => Ok(GameEvent::GetInventoryItem(InventoryItem::Clamp)),
            E::KeyDown {
                key: Some(K::Num8), ..
            } => {
                Ok(GameEvent::GetInventoryItem(InventoryItem::AccessCard))
            }
            E::KeyDown {
                key: Some(K::Num9), ..
            } => Ok(GameEvent::IncreaseLife),
            E::KeyDown {
                key: Some(K::Num0), ..
            } => Ok(GameEvent::FinishLevel),
            E::KeyDown {
                key: Some(K::F), ..
            }
            | E::KeyDown {
                key: Some(K::F11), ..
            } => Ok(GameEvent::ToggleFullscreen),
            E::KeyDown {
                key: Some(K::Down),
                modifiers,
            } if modifiers.contains(&M::LeftShift)
                || modifiers.contains(&M::RightShift) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: HALFTILE_HEIGHT as i32,
                    y: 0,
                })
            }
            E::KeyDown {
                key: Some(K::Up),
                modifiers,
            } if modifiers.contains(&M::LeftShift)
                || modifiers.contains(&M::RightShift) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: -(HALFTILE_HEIGHT as i32),
                    y: 0,
                })
            }
            E::KeyDown {
                key: Some(K::Right),
                modifiers,
            } if modifiers.contains(&M::LeftShift)
                || modifiers.contains(&M::RightShift) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: 0,
                    y: HALFTILE_WIDTH as i32,
                })
            }
            E::KeyDown {
                key: Some(K::Left),
                modifiers,
            } if modifiers.contains(&M::LeftShift)
                || modifiers.contains(&M::RightShift) =>
            {
                Ok(GameEvent::MoveViewPoint {
                    x: 0,
                    y: -(HALFTILE_WIDTH as i32),
                })
            }
            E::KeyDown {
                key: Some(K::Up), ..
            }
            | E::MouseButtonDown {
                button: Some(MouseButton::Middle),
                ..
            } => Ok(GameEvent::HeroInteractionStart),
            E::KeyDown {
                key: Some(K::Right),
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled((
                HorizontalDirection::Right,
                true,
            ))),
            E::KeyDown {
                key: Some(K::Left), ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled((
                HorizontalDirection::Left,
                true,
            ))),
            E::KeyDown {
                key: Some(K::LeftCtrl),
                ..
            }
            | E::MouseButtonDown {
                button: Some(MouseButton::Right),
                ..
            } => Ok(GameEvent::HeroJump),
            E::KeyDown {
                key: Some(K::LeftAlt),
                ..
            }
            | E::MouseButtonDown {
                button: Some(MouseButton::Left),
                ..
            } => Ok(GameEvent::HeroStartFiring),
            E::KeyUp {
                key: Some(K::Up), ..
            }
            | E::MouseButtonUp {
                button: Some(MouseButton::Middle),
                ..
            } => Ok(GameEvent::HeroInteractionEnd),
            E::KeyUp {
                key: Some(K::Right),
                ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled((
                HorizontalDirection::Right,
                false,
            ))),
            E::KeyUp {
                key: Some(K::Left), ..
            } => Ok(GameEvent::HeroSetWalkingDirectionEnabled((
                HorizontalDirection::Left,
                false,
            ))),
            E::KeyUp {
                key: Some(K::LeftAlt),
                ..
            }
            | E::MouseButtonUp {
                button: Some(MouseButton::Left),
                ..
            } => Ok(GameEvent::HeroStopFiring),
            E::VideoExpose => Ok(GameEvent::RefreshScreen),
            E::UserEvent { code } if code == UserEvent::Timer as i32 => {
                Ok(GameEvent::TimerTriggered)
            }
            E::UserEvent { code }
                if code == UserEvent::HeroMoved as i32 =>
            {
                Ok(GameEvent::HeroMoved)
            }
            E::UserEvent { code }
                if code == UserEvent::HeroScored as i32 =>
            {
                Ok(GameEvent::HeroScored)
            }
            E::UserEvent { code }
                if code == UserEvent::HeroFirepowerChanged as i32 =>
            {
                Ok(GameEvent::HeroFirepowerChanged)
            }
            E::UserEvent { code }
                if code == UserEvent::HeroInventoryChanged as i32 =>
            {
                Ok(GameEvent::HeroInventoryChanged)
            }
            E::UserEvent { code }
                if code == UserEvent::HeroHealthChanged as i32 =>
            {
                Ok(GameEvent::HeroHealthChanged)
            }
            E::UserEvent { code }
                if code == UserEvent::HeroLanded as i32 =>
            {
                Ok(GameEvent::HeroLanded)
            }
            _ => Err(anyhow!("Event not handled")),
        }
    }
}

impl TryFrom<transdl::event::Event> for ConfirmEvent {
    type Error = Error;

    fn try_from(e: transdl::event::Event) -> Result<ConfirmEvent> {
        use transdl::event::{Event as E, KeyCode as K};
        match e {
            E::KeyDown {
                key: Some(K::Return),
                ..
            } => return Ok(ConfirmEvent::Confirmed),
            E::KeyDown {
                key: Some(K::Escape),
                ..
            }
            | E::Quit => {
                return Ok(ConfirmEvent::Aborted);
            }
            E::VideoExpose => {
                return Ok(ConfirmEvent::RefreshScreen);
            }
            _ => Err(anyhow!("Event not handled")),
        }
    }
}

impl TryFrom<transdl::event::Event> for InputEvent {
    type Error = Error;

    fn try_from(e: transdl::event::Event) -> Result<InputEvent> {
        use transdl::event::{Event as E, KeyCode as K, Modifier as M};
        match e {
            E::KeyDown { key, modifiers } => match key {
                Some(K::BackSpace) => Ok(InputEvent::DeleteLeft),
                Some(K::Delete) => Ok(InputEvent::DeleteRight),
                Some(K::Left) => Ok(InputEvent::MoveCursorLeft),
                Some(K::Right) => Ok(InputEvent::MoveCursorRight),
                Some(K::Return) => Ok(InputEvent::Confirm),
                Some(K::Escape) => Ok(InputEvent::Abort),
                Some(code)
                    if code == K::Space
                        || code == K::Exclaim
                        || code == K::QuoteDouble
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
                    if modifiers.contains(&M::LeftShift)
                        || modifiers.contains(&M::RightShift)
                    {
                        c.make_ascii_uppercase();
                    }
                    Ok(InputEvent::Letter(c))
                }
                _ => Err(anyhow!("Event not handled")),
            },
            E::VideoExpose => Ok(InputEvent::RefreshScreen),
            _ => Err(anyhow!("Event not handled")),
        }
    }
}
