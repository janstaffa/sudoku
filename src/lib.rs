pub mod board;
pub mod cell;

use std::io::{self, Write};

use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    ttf::Font,
    video::Window,
};

pub fn get_input() -> Result<String, std::io::Error> {
    print!("Enter path to your game: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn draw_text(
    canvas: &mut Canvas<Window>,
    font: &Font,
    text: &str,
    color: Color,
    x: i32,
    y: i32,
    wrap: Option<u32>,
) -> Result<(), String> {
    let surface = font.render(text);
    let surface = if let Some(wl) = wrap {
        surface
            .blended_wrapped(color, wl)
            .map_err(|e| e.to_string())?
    } else {
        surface.blended(color).map_err(|e| e.to_string())?
    };

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(x, y, width, height);

    canvas.copy(&texture, None, target)?;

    return Ok(());
}

pub fn draw_custom_rect(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    border_width: u32,
) -> Result<(), String> {
    let rect: [Rect; 4] = [
        Rect::new(x, y, w, border_width),
        Rect::new(x, y, border_width, h),
        Rect::new(x, (y + h as i32) - border_width as i32, w, border_width),
        Rect::new((x + w as i32) - border_width as i32, y, border_width, h),
    ];
    for r in rect {
        canvas.fill_rect(r)?;
    }
    return Ok(());
}
