// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::messagebox;
use crate::event::{MenuEvent, OnOffTracking, WaitEvent};
use crate::rendering::{CanvasRenderer, Renderer};
use crate::{
    RangedIterator, TileProvider, UserEvent, FONT_HEIGHT, FONT_WIDTH,
    GAME_INTERVAL, OBJECT_POINT,
};
use anyhow::{Error, Result};
use sdl2::{
    event::EventSender,
    rect::{Point, Rect},
    render::WindowCanvas,
    EventPump, TimerSubsystem,
};
use std::collections::BTreeSet;

pub struct MenuEntry {
    pub shortcut: char,
    pub name: String,
}

pub struct Menu {
    header: String,
    entries: Vec<MenuEntry>,
    current: usize,
    width: usize,
}

impl Menu {
    pub fn new(header: String) -> Self {
        Menu {
            header,
            entries: Vec::new(),
            current: 0usize,
            width: 0usize,
        }
    }

    pub fn append(&mut self, entry: MenuEntry) {
        self.width = std::cmp::max(self.width, entry.name.len());
        self.entries.push(entry);
    }

    pub fn get_choice(
        &mut self,
        canvas: &mut WindowCanvas,
        tileprovider: &dyn TileProvider,
        event_pump: &mut EventPump,
        event_sender: &EventSender,
        timer_subsystem: &TimerSubsystem,
    ) -> Result<char> {
        let texture_creator = canvas.texture_creator();
        let (headercols, headerrows) =
            messagebox::get_information(&self.header);

        let textcols = std::cmp::max(headercols, self.width);

        let contents = format!(
            "{}\n{}\n  {}",
            &self.header,
            " ".repeat(textcols + 2),
            self.entries
                .iter()
                .map(|entry| entry.name.to_string())
                .collect::<Vec<_>>()
                .join("\n  "),
        );

        let messagebox = messagebox::messagebox(
            &contents,
            tileprovider,
            &texture_creator,
        )?;

        let surface =
            canvas.window().surface(event_pump).map_err(Error::msg)?;
        let destrect = Rect::from_center(
            Point::new(
                surface.width() as i32 / 2,
                surface.height() as i32 / 2,
            ),
            messagebox.width(),
            messagebox.height(),
        );

        let timer = timer_subsystem.add_timer(
            GAME_INTERVAL,
            Box::new(move || {
                event_sender.push_custom_event(UserEvent::Timer).unwrap();
                GAME_INTERVAL
            }),
        );

        let mut changed = true;
        let mut animationframe = RangedIterator::new(4);

        let mut next_enabled = BTreeSet::new();
        let mut previous_enabled = BTreeSet::new();

        loop {
            if changed {
                canvas
                    .copy(
                        &messagebox.as_texture(&texture_creator)?,
                        None,
                        destrect,
                    )
                    .map_err(Error::msg)?;

                let mut renderer = CanvasRenderer {
                    canvas,
                    texture_creator: &texture_creator,
                    tileprovider,
                };

                let point_pos = Point::new(
                    destrect.left() + (FONT_WIDTH as i32 / 2) * 3,
                    destrect.top()
                        + FONT_HEIGHT as i32
                            * (self.current as i32
                                + headerrows as i32
                                + 2),
                );

                renderer.place_tile(
                    OBJECT_POINT + animationframe.current(),
                    point_pos,
                )?;
                canvas.present();
                changed = false;
            }

            let choice: Option<char> = match MenuEvent::wait(event_pump)? {
                MenuEvent::ChooseCurrentEntry | MenuEvent::ClickMouse => {
                    Some(self.entries[self.current].shortcut)
                }
                MenuEvent::Abort => Some('\0'),
                MenuEvent::NextEntry { context, enabled } => {
                    next_enabled.set_enabled(
                        context,
                        enabled,
                        &mut || {
                            self.current += 1;
                            self.current %= self.entries.len();
                        },
                        &mut || {},
                    );
                    None
                }
                MenuEvent::PreviousEntry { context, enabled } => {
                    previous_enabled.set_enabled(
                        context,
                        enabled,
                        &mut || {
                            if self.current == 0 {
                                self.current = self.entries.len();
                            }
                            self.current -= 1;
                        },
                        &mut || {},
                    );
                    None
                }
                MenuEvent::ChooseShortcutEntry(key) => {
                    let mut choice = None;
                    for entry in self.entries.iter() {
                        if key == entry.shortcut {
                            choice = Some(entry.shortcut);
                        }
                    }
                    choice
                }
                MenuEvent::MoveMouse { x, y } => {
                    let x = x - destrect.x() - FONT_WIDTH as i32 * 3;
                    let y = y
                        - destrect.y()
                        - FONT_HEIGHT as i32 * (headerrows as i32 + 2);

                    if x > 0
                        && x < (FONT_WIDTH * messagebox.width()) as i32
                    {
                        let menuitem = y / FONT_HEIGHT as i32;
                        if menuitem >= 0
                            && menuitem < self.entries.len() as i32
                        {
                            self.current = menuitem as usize;
                        }
                    }

                    None
                }
                MenuEvent::RefreshScreen => {
                    canvas.present();
                    None
                }
                MenuEvent::TimerTriggered => {
                    animationframe.next();
                    changed = true;
                    None
                }
            };
            if let Some(choice) = choice {
                drop(timer);
                return Ok(choice);
            }
        }
    }
}
