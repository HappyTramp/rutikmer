use sdl2::ttf;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub mod time;
pub mod scramble;
pub mod text;
pub mod history;

use scramble::Scramble;
use history::History;

const WIDTH:  u32  = 640;
const HEIGHT: u32  = 480;
const TITLE:  &str = "rutikmer";
const FONT_SIZE: u32 = 40;

const GREEN:  Color = Color::RGB(0x1B, 0x5E, 0x20);
const ORANGE: Color = Color::RGB(0xEF, 0x6C, 0x00);
const BLACK:  Color = Color::RGB(0x00, 0x00, 0x00);

fn main() {
    let sdl = sdl2::init().unwrap();
    let ttf = ttf::init().unwrap();
    let video_subsys = sdl.video().unwrap();
    let window = video_subsys
        .window(TITLE, WIDTH, HEIGHT)
        .resizable()
        .build()
        .unwrap();
    let font = ttf.load_font("font/FiraMono-Regular.ttf", FONT_SIZE as u16).unwrap();
    let hist = History::from_csv("history.csv");

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let tex_creator = canvas.texture_creator();

    let mut timer = time::Timer::new();
    let timer_rect = Rect::new(10, 10, 100, 40);
    let mut text_factory = text::Factory::new(&font, &tex_creator, BLACK, FONT_SIZE);

    let mut scramble_str = Scramble::new_rand(10).to_string();

    'running: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    match timer.state {
                        time::State::Idle     => {},
                        time::State::Active   => timer.stop(),
                        time::State::Inactive => {
                            timer.idle();
                            scramble_str = Scramble::new_rand(10).to_string();
                        },
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    if timer.state == time::State::Idle {
                        timer.start();
                    }
                },

                // Event::Window { win_event: WindowEvent::Resized(w, h), .. } =>
                //     ui.set_layout(w as u32, h as u32),
                _ => {}
            }
        }
        let bg_color = match timer.state {
            time::State::Idle     => ORANGE,
            time::State::Active   => GREEN,
            time::State::Inactive => BLACK,
        };
        canvas.set_draw_color(bg_color);
        text_factory.set_bg(bg_color);
        canvas.clear();

        if timer.state != time::State::Idle {
            canvas.copy(&text_factory.from_string(&timer.to_string()), None, timer_rect).unwrap();
        }

        if timer.state == time::State::Inactive {
            canvas.copy(&text_factory.from_string(&scramble_str), None, Rect::new(10, 100, 500, 40)).unwrap();
            let sum = hist.summary(3);
            for (i, s) in sum.iter().enumerate() {
                canvas.copy(&text_factory.from_string(&s), None, Rect::new(10, 200 + 40 * (i as i32), 200, 30)).unwrap();
            }
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 30_000_000));
    }
}
