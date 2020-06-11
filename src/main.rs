use sdl2::ttf;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub mod time;
pub mod shuffle;
pub mod text;

const WIDTH:  u32  = 640;
const HEIGHT: u32  = 480;
const TITLE:  &str = "rutikmer";

fn main() {
    let sdl = sdl2::init().unwrap();
    let ttf = ttf::init().unwrap();
    let video_subsys = sdl.video().unwrap();
    let window = video_subsys
        .window(TITLE, WIDTH, HEIGHT)
        .resizable()
        .build()
        .unwrap();
    let font = ttf.load_font("font/FiraMono-Regular.ttf", 40).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let tex_creator = canvas.texture_creator();

    let mut timer = time::Timer::new();
    let timer_rect = Rect::new(10, 10, 100, 40);
    let mut text_factory = text::Factory::new(&font, &tex_creator, Color::RGB(0, 0, 0));

    let mut shuffle_str = shuffle::Move::string_sequence(10);

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
                            shuffle_str = shuffle::Move::string_sequence(10);
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
            time::State::Idle     => Color::RGB(100, 100, 0),
            time::State::Active   => Color::RGB(0, 100, 0),
            time::State::Inactive => Color::RGB(0, 0, 0),
        };
        canvas.set_draw_color(bg_color);
        text_factory.set_bg(bg_color);
        canvas.clear();

        if timer.state != time::State::Idle {
            canvas.copy(&text_factory.from_string(&timer.to_string()), None, timer_rect).unwrap();
        }

        if timer.state == time::State::Inactive {
            canvas.copy(&text_factory.from_string(&shuffle_str), None, Rect::new(10, 100, 500, 40)).unwrap();
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 30_000_000));
    }
}
