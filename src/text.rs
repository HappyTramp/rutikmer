use sdl2::ttf;
use sdl2::render::{Texture, TextureCreator};
use sdl2::pixels::Color;

struct Factory<'a, T> {
    font: ttf::Font<'a, 'a>,
    creator: &'a TextureCreator<T>,
    bg: Color,
}

impl Factory<'a, T> {
    pub fn new(font: &ttf::Font<'a, 'a>, creator: &'a TextureCreator<T>, bg: Color) -> Factory {
        Factory { font, creator, bg }
    }
}

pub fn to_texture<'a, T>(
    s: &'a String,
    font: &ttf::Font,
    tex_creator: &'a TextureCreator<T>,
    bg: &Color
) -> Texture<'a>
{
    let surface = font.render(s).shaded(Color::RGB(255, 255, 255), *bg).unwrap();
    tex_creator.create_texture_from_surface(&surface).unwrap()
}

// pub fn width(s: &String
