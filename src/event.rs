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
