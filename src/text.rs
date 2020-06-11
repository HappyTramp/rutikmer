use sdl2::ttf;
use sdl2::render::{Texture, TextureCreator};
use sdl2::pixels::Color;

const WHITE: Color = Color::RGB(255, 255, 255);

pub struct Factory<'a, T> {
    font: &'a ttf::Font<'a, 'a>,
    creator: &'a TextureCreator<T>,
    bg: Color,
}

impl<'a, T> Factory<'a, T> {
    pub fn new(font: &'a ttf::Font,
            creator: &'a TextureCreator<T>,
            bg: Color
              ) -> Factory<'a, T>
    {
        Factory { font, creator, bg }
    }

    pub fn from_string(&self, s: &String) -> Texture
    {
        let surface = self.font.render(s).shaded(WHITE, self.bg).unwrap();
        self.creator.create_texture_from_surface(&surface).unwrap()
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }
}


// pub trait TextTexture {
//     fn to_texture<T>(&self, factory: &Factory<T>) -> Texture;
// }
//
// impl TextTexture for String {
//     fn to_texture<T>(&self, factory: &Factory<T>) -> Texture {
//         let surface = factory.font.render(self).shaded(WHITE, factory.bg).unwrap();
//         factory.creator.create_texture_from_surface(&surface).unwrap()
//     }
// }
