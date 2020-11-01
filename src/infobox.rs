use super::geometry::Geometry;
use super::messagebox::messagebox;
use super::texture::TextureCreationParams;
use super::tilecache::TileCache;
use transdl::event::Event;
use transdl::video::Surface;

pub fn show(
    screen: &mut Surface,
    tilecache: &TileCache,
    texture_creation_params: TextureCreationParams,
    text: &str,
) {
    let messagebox = messagebox(text, tilecache, texture_creation_params);
    let destrect = Geometry {
        x: (screen.width() as isize - messagebox.width() as isize) as i16
            / 2,
        y: (screen.height() as isize - messagebox.height() as isize)
            as i16
            / 2,
        w: messagebox.width(),
        h: messagebox.height(),
    };

    // backup the background
    let mut background_backup =
        texture_creation_params.create_surface(destrect.w, destrect.h);

    screen.blit(
        Some(destrect.as_sdl_rect()),
        &mut background_backup,
        None,
    );
    messagebox.blit_to_sdl_surface(None, screen, Some(destrect.clone()));
    screen.update_rect(0, 0, 0, 0);

    loop {
        match Event::wait() {
            Ok(Event::KeyDown { .. })
            | Ok(Event::MouseButtonDown { .. }) => {
                background_backup.blit(
                    None,
                    screen,
                    Some(destrect.as_sdl_rect()),
                );
                screen.update_rect(0, 0, 0, 0);
                return;
            }
            Ok(Event::VideoExpose) => {
                screen.update_rect(0, 0, 0, 0);
            }
            Ok(_) => {
                // Ignore other events
            }
            Err(e) => {
                panic!("Error getting SDL event: {:?}", e);
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
        params: TextureCreationParams,
    ) {
        for message in self.messages.drain(..) {
            show(screen, tilecache, params, &message);
        }
    }

    pub fn push_back(&mut self, msg: String) {
        self.messages.push(msg);
    }
}
