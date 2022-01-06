use crate::data_dir;
use anyhow::Error;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget, TextureCreator, TextureQuery},
    ttf::Font,
};
use std::path::PathBuf;

pub fn required_file_names() -> Vec<&'static str> {
    vec![
        "anim0", "anim1", "anim2", "anim3", "anim4", "anim5", "back0",
        "back1", "back2", "back3", "badguy", "border", "credits", "dn",
        "drop0", "duke", "duke1-b", "duke1", "end", "font1", "font2",
        "man0", "man1", "man2", "man3", "man4", "numbers", "object0",
        "object1", "object2", "solid0", "solid1", "solid2", "solid3",
        "worldal1", "worldal2", "worldal3", "worldal4", "worldal5",
        "worldal6", "worldal7", "worldal8", "worldal9", "worldala",
        "worldalb", "worldalc",
    ]
}

pub fn original_data_dir() -> PathBuf {
    data_dir().join("data").join("original")
}

pub fn display_text<RT: RenderTarget, T>(
    canvas: &mut Canvas<RT>,
    x: i32,
    y: i32,
    font: &Font,
    message: &str,
    texture_creator: &TextureCreator<T>,
) -> crate::Result<()> {
    let mut destrect = Rect::new(x, y, 0, 0);
    let color = Color::RGB(255, 255, 255);

    canvas.set_draw_color(color);
    for line in message.lines() {
        let text = font.render(line).blended(color)?;
        let text = texture_creator.create_texture_from_surface(&text)?;
        let TextureQuery { width, height, .. } = text.query();
        destrect.set_y(destrect.y() + height as i32);
        destrect.set_width(width);
        destrect.set_height(height);
        canvas.copy(&text, None, destrect).map_err(Error::msg)?;
    }
    canvas.present();
    Ok(())
}
