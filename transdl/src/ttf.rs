use super::video::{Color, Surface};
use crate::ll;

pub fn init() -> Result<(), String> {
    match unsafe { ll::TTF_Init() } {
        0 => Ok(()),
        _ => match super::get_error() {
            Some(text) => Err(text),
            None => Err("Unknown error".to_string()),
        },
    }
}

#[repr(transparent)]
pub struct Font {
    pub raw: *mut ll::TTF_Font,
}

impl Font {
    pub fn render_utf8_shaded(
        &self,
        text: &str,
        fg_color: Color,
        bg_color: Color,
    ) -> Surface {
        use std::os::raw::c_char;

        let text = text.as_ptr() as *const c_char;

        Surface {
            raw: unsafe {
                ll::TTF_RenderUTF8_Shaded(
                    self.raw, text, fg_color, bg_color,
                )
            },
        }
    }
}
