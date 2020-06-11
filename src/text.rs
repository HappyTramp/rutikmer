use sdl2::ttf;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WHITE: Color = Color::RGB(255, 255, 255);

pub struct Factory<'a, T> {
    font: &'a ttf::Font<'a, 'a>,
    creator: &'a TextureCreator<T>,
    bg: Color,
    font_size: u32,
}

impl<'a, T> Factory<'a, T> {
    pub fn new(font: &'a ttf::Font,
               creator: &'a TextureCreator<T>,
               bg: Color,
               font_size: u32
              ) -> Factory<'a, T>
    {
        Factory { font, creator, bg, font_size }
    }

    pub fn from_string(&self, s: &str) -> Texture
    {
        let surface = self.font.render(s).shaded(WHITE, self.bg).unwrap();
        self.creator.create_texture_from_surface(&surface).unwrap()
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }
}

struct Frame<'a> {
    width: u32,
    height: u32,
    texture: Texture<'a>,
}

impl<'a> Frame<'a> {
    fn new<T>(s: &'a str, factory: &'a Factory<T>) -> Frame<'a> {
        Frame{
            width: s.len() as u32 * factory.font_size,
            height: factory.font_size,
            texture: factory.from_string(s)
        }
    }

    fn to_rect(&self, x: i32, y: i32) -> Rect {
        Rect::new(x, y, self.width, self.height)
    }
}

trait Framable {
    fn to_frame<'a>(&'a self) -> Frame<'a>;

    fn put_canvas<T: sdl2::render::RenderTarget>(&self, canvas: &mut Canvas<T>, x: i32, y: i32) {
        let frame = self.to_frame();
        let rect = frame.to_rect(x, y);
        canvas.copy(&frame.texture, None, rect);
    }
}
