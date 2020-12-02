use super::geometry::Geometry;
use super::messagebox::messagebox;
use super::tilecache::TileCache;
use crate::event::{ConfirmEvent, WaitEvent};
use crate::graphics::{SurfaceCreator, SurfaceCreatorProvider};
use anyhow::Result;
use transdl::video::Surface;

pub fn show(
    screen: &mut Surface,
    tilecache: &TileCache,
    text: &str,
) -> Result<()> {
    let messagebox =
        messagebox(text, tilecache, &mut screen.surface_creator());
    let destrect = Geometry {
        x: (screen.width() as isize - messagebox.width() as isize) as i16
            / 2,
        y: (screen.height() as isize - messagebox.height() as isize)
            as i16
            / 2,
        w: messagebox.width() as u16,
        h: messagebox.height() as u16,
    };

    // backup the background
    let mut background_backup = screen
        .surface_creator()
        .create(destrect.w as u32, destrect.h as u32);

    screen.blit(
        Some(destrect.as_sdl_rect()),
        &mut background_backup,
        None,
    );
    messagebox.blit(None, screen, Some(destrect.as_sdl_rect()));
    screen.update_rect(0, 0, 0, 0);

    loop {
        match ConfirmEvent::wait()? {
            ConfirmEvent::Confirmed | ConfirmEvent::Aborted => {
                background_backup.blit(
                    None,
                    screen,
                    Some(destrect.as_sdl_rect()),
                );
                screen.update_rect(0, 0, 0, 0);
                return Ok(());
            }
            ConfirmEvent::RefreshScreen => {
                screen.update_rect(0, 0, 0, 0);
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
        screen: &mut Surface,
        tilecache: &TileCache,
    ) -> Result<()> {
        for message in self.messages.drain(..) {
            show(screen, tilecache, &message)?;
        }
        Ok(())
    }

    pub fn push_back(&mut self, msg: String) {
        self.messages.push(msg);
    }
}
