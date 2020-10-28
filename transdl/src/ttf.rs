use super::video::{Color, Surface};
use crate::ll;
use std::ffi::CString;
use std::path::PathBuf;

fn base_directories() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/usr/share/fonts/truetype/noto/"),
        PathBuf::from("/usr/share/fonts"),
        PathBuf::from("/usr/share/fonts/truetype"),
        PathBuf::from("/usr/local/share/fonts"),
        PathBuf::from("/usr/local/share/fonts/truetype"),
        PathBuf::from("/usr/share/fonts/TTF"),
    ]
}

fn font_names() -> Vec<String> {
    vec![
        "NotoSansDisplay-Black.ttf".to_string(),
        "DejaVuSans.ttf".to_string(),
        "DejavuSerif.ttf".to_string(),
        "linux-libertine/LinLibertine.ttf".to_string(),
        "linux-libertine/LinLibertine_Re.ttf".to_string(),
        "freefont/FreeMono.ttf".to_string(),
        "freefont/FreeSans.ttf".to_string(),
        "freefont/FreeSerif.ttf".to_string(),
        "dustin/Balker.ttf".to_string(),
        "dustin/Dustismo.ttf".to_string(),
        "kochi/kochi-gothic.ttf".to_string(),
        "kochi/kochi-mincho.ttf".to_string(),
        "msttcorefonts/arial.ttf".to_string(),
        "msttcorefonts/Arial.ttf".to_string(),
        "msttcorefonts/Arial_Black.ttf".to_string(),
        "msttcorefonts/times.ttf".to_string(),
        "msttcorefonts/verdana.ttf".to_string(),
        "ttf-bitstream-vera/Vera.ttf".to_string(),
        "ttf-dejavu/DejaVuSans.ttf".to_string(),
        "ttf-dejavu/DejaVuSansMono.ttf".to_string(),
        "unfonts/UnBatang.ttf".to_string(),
        "unfonts/UnDotum.ttf".to_string(),
    ]
}

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
    pub fn load(font_size: u16) -> Result<Self, String> {
        init()?;
        for name in font_names() {
            for path in base_directories() {
                let path = path.join(&name);
                let path = path.to_string_lossy();
                let filename = CString::new(path.as_bytes()).unwrap();
                let raw = unsafe {
                    ll::TTF_OpenFont(filename.as_ptr(), font_size as i32)
                };
                if !raw.is_null() {
                    return Ok(Font { raw });
                }
            }
        }
        return Err("Couldn't find any font".to_string());
    }

    pub fn render_utf8_shaded(
        &self,
        text: &str,
        fg_color: Color,
        bg_color: Color,
    ) -> Surface {
        let text = CString::new(text).unwrap();

        Surface {
            raw: unsafe {
                ll::TTF_RenderUTF8_Shaded(
                    self.raw,
                    text.as_ptr(),
                    fg_color,
                    bg_color,
                )
            },
        }
    }
}
