// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::messagebox::messagebox;
use crate::event::{ConfirmEvent, WaitEvent};
use crate::{Result, TileProvider};
use anyhow::Error;
use sdl2::{
    rect::{Point, Rect},
    render::WindowCanvas,
    surface::Surface,
    EventPump,
};

pub fn show(
    canvas: &mut WindowCanvas,
    tileprovider: &dyn TileProvider,
    text: &str,
    event_pump: &mut EventPump,
) -> Result<()> {
    let texture_creator = canvas.texture_creator();
    let messagebox = messagebox(text, tileprovider, &texture_creator)?;
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

    let mut background_backup = Surface::new(
        destrect.width(),
        destrect.height(),
        surface.pixel_format_enum(),
    )
    .map_err(Error::msg)?;
    surface
        .blit(destrect, &mut background_backup, None)
        .map_err(Error::msg)?;

    canvas
        .copy(&messagebox.as_texture(&texture_creator)?, None, destrect)
        .map_err(Error::msg)?;
    canvas.present();

    loop {
        match ConfirmEvent::wait(event_pump)? {
            ConfirmEvent::Confirmed | ConfirmEvent::Aborted => {
                canvas
                    .copy(
                        &background_backup.as_texture(&texture_creator)?,
                        None,
                        destrect,
                    )
                    .map_err(Error::msg)?;
                canvas.present();
                return Ok(());
            }
            ConfirmEvent::RefreshScreen => {
                canvas.present();
            }
        }
    }
}

#[derive(Default)]
pub struct InfoMessageQueue {
    messages: Vec<String>,
}

impl InfoMessageQueue {
    pub fn new() -> Self {
        InfoMessageQueue {
            messages: Vec::new(),
        }
    }

    pub fn process(
        &mut self,
        canvas: &mut WindowCanvas,
        tileprovider: &dyn TileProvider,
        event_pump: &mut EventPump,
    ) -> Result<()> {
        for message in self.messages.drain(..) {
            show(canvas, tileprovider, &message, event_pump)?;
        }
        Ok(())
    }

    pub fn push_back(&mut self, msg: String) {
        self.messages.push(msg);
    }
}
