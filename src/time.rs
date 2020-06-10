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

    pub fn to_texture<'a, T>(&'a self, font: &ttf::Font, texture_creator: &'a TextureCreator<T>) -> Texture {
        let rendered = if self.state == State::Active { self.time.elapsed().unwrap() } else { self.result };

        let surface = font.render(&rendered.as_millis().to_string())
                          .solid(Color::RGB(255, 255, 255))
                          .unwrap();
        texture_creator.create_texture_from_surface(&surface).unwrap()
    }
}
