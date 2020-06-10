use std::time::{SystemTime, Duration};
use sdl2::ttf;
use sdl2::render::{Texture, TextureCreator};
use sdl2::pixels::Color;

#[derive(PartialEq)]
pub enum State {
    Idle,
    Active,
    Inactive,
}

pub struct Timer {
    pub state: State,
    time: SystemTime,
    result: Duration,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            state: State::Inactive,
            time: SystemTime::now(),
            result: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.time = SystemTime::now();
        self.state = State::Active;
    }

    pub fn stop(&mut self) {
        self.result = self.time.elapsed().unwrap();
        self.state = State::Inactive;
    }

    pub fn idle(&mut self) {
        self.state = State::Idle;
    }

    pub fn to_texture<'a, T>(
        &'a self,
        font: &ttf::Font,
        tex_creator: &'a TextureCreator<T>,
        bg: &Color
    ) -> Texture
    {
        let current = if self.state == State::Active {
            self.time.elapsed().unwrap()
        } else {
            self.result
        }.as_millis();

        let s = format!("{}.{}", current / 1000, current % 1000);

        let surface = font.render(&s).shaded(Color::RGB(255, 255, 255), *bg).unwrap();
        tex_creator.create_texture_from_surface(&surface).unwrap()
    }
}
