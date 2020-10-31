use super::geometry::Geometry;
use super::messagebox;
use super::texture::{Texture, TextureCreationParams};
use super::tilecache::TileCache;
use crate::{FONT_HEIGHT, FONT_WIDTH, OBJECT_POINT};
use transdl::event::{Event, KeyCode, MouseButton};
use transdl::video::Surface;

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
        screen: &mut Surface,
        tilecache: &TileCache,
        texture_creation_paramns: TextureCreationParams,
    ) -> char {
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

        let msgbox = messagebox::messagebox(
            &contents,
            tilecache,
            texture_creation_paramns,
        );

        let mut target = Texture::create_with_params(
            msgbox.width(),
            msgbox.height(),
            texture_creation_paramns,
        );

        let destrect = Geometry {
            x: (screen.width() - msgbox.width() as usize) as i16 / 2,
            y: (screen.height() - msgbox.height() as usize) as i16 / 2,
            w: msgbox.width(),
            h: msgbox.height(),
        };

        transdl::event::enable_key_repeat();

        let timer = transdl::timer::Timer::add(80, || {
            transdl::event::push_user_event(0)
        });

        let pointrect = Geometry {
            x: (FONT_WIDTH * 2 + destrect.x as usize) as i16,
            y: (FONT_HEIGHT + destrect.y as usize) as i16,
            w: FONT_WIDTH as u16,
            h: FONT_HEIGHT as u16,
        };

        let mut changed = true;
        let mut animationframe = 0;
        let mut update_whole_screen = true;
        let mut update_whole_menu = false;

        loop {
            if changed || true {
                msgbox.clone_to_texture(None, &mut target, None);

                let marker = tilecache
                    .get_tile(OBJECT_POINT + animationframe)
                    .unwrap();

                let targetrect = Geometry {
                    x: (FONT_WIDTH * 3) as i16 / 2,
                    y: (FONT_HEIGHT * (self.current + headerrows + 2))
                        as i16,
                    w: FONT_WIDTH as u16,
                    h: FONT_HEIGHT as u16,
                };

                marker.clone_to_texture(
                    None,
                    &mut target,
                    Some(targetrect),
                );

                target.blit_to_sdl_surface(
                    None,
                    screen,
                    Some(destrect.clone()),
                );

                if update_whole_screen {
                    screen.update_rect(0, 0, 0, 0);
                    update_whole_screen = false;
                    update_whole_menu = false;
                } else if update_whole_menu {
                    screen.update_rect(
                        destrect.x as i32,
                        destrect.y as i32,
                        destrect.w as u32,
                        destrect.h as u32,
                    );
                    update_whole_menu = false;
                } else {
                    screen.update_rect(
                        pointrect.x as i32,
                        pointrect.y as i32,
                        pointrect.w as u32,
                        pointrect.h as u32,
                    );
                }
                changed = false;
            }

            let choice: Option<char> = match Event::wait() {
                Ok(Event::KeyDown {
                    key: Some(KeyCode::Return),
                    ..
                }) => Some(self.entries[self.current].shortcut),
                Ok(Event::KeyDown {
                    key: Some(KeyCode::Escape),
                    ..
                }) => Some('\0'),
                Ok(Event::KeyDown {
                    key: Some(KeyCode::Down),
                    ..
                }) => {
                    self.current += 1;
                    self.current %= self.entries.len();
                    update_whole_menu = true;
                    None
                }
                Ok(Event::KeyDown {
                    key: Some(KeyCode::Up),
                    ..
                }) => {
                    if self.current == 0 {
                        self.current = self.entries.len();
                    }
                    self.current -= 1;
                    update_whole_menu = true;
                    None
                }
                Ok(Event::KeyDown { key: Some(key), .. }) => {
                    use std::convert::TryFrom;
                    let mut choice = None;
                    for entry in self.entries.iter() {
                        let c = KeyCode::try_from(entry.shortcut as isize)
                            .ok();
                        if let Some(c) = c {
                            if key == c {
                                choice = Some(entry.shortcut);
                                break;
                            }
                        }
                    }
                    choice
                }
                Ok(Event::MouseMotion { x, y, .. }) => {
                    let x = x as i16 - destrect.x - FONT_WIDTH as i16 * 3;
                    let y = y as i16
                        - destrect.y
                        - FONT_HEIGHT as i16 * (headerrows as i16 + 2);

                    if x > 0
                        && x < FONT_WIDTH as i16 * msgbox.width() as i16
                    {
                        let menuitem = y / FONT_HEIGHT as i16;
                        if menuitem >= 0
                            && menuitem < self.entries.len() as i16
                        {
                            self.current = menuitem as usize;
                            update_whole_menu = true;
                        }
                    }

                    None
                }
                Ok(Event::MouseButtonDown { button, .. }) => {
                    if button == Some(MouseButton::Left) {
                        Some(self.entries[self.current].shortcut)
                    } else {
                        None
                    }
                }
                Ok(Event::VideoExpose) => {
                    screen.update_rect(0, 0, 0, 0);
                    None
                }
                Ok(Event::UserEvent { code: 0 }) => {
                    animationframe += 1;
                    animationframe %= 4;
                    changed = true;
                    update_whole_menu = true;
                    None
                }
                Ok(_) => {
                    // Ignore other events
                    None
                }
                Err(e) => {
                    eprintln!("Error handling SDL event: {:?}", e);
                    None
                }
            };
            if let Some(choice) = choice {
                timer.remove();
                transdl::event::disable_key_repeat();
                return choice;
            }
        }
    }
}
