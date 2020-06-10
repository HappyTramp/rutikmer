use sdl2::ttf;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

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
    // let mut ui = ui::UI::new(WIDTH, HEIGHT);
    //
    let timer_rect = Rect::new(10, 10, 100, 40);

    'running: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    match timer.state {
                        time::State::Active   => timer.stop(),
                        time::State::Inactive => timer.idle(),
                        time::State::Idle     => {},
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
        canvas.clear();

        if timer.state != time::State::Idle {
            canvas.copy(&timer.to_texture(&font, &tex_creator, &bg_color),
                        None,
                        timer_rect)
                .unwrap();
        }

        let s = shuffle::Move::string_sequence(10);
        let shuff_tex = text::to_texture(&s, &font, &tex_creator, &bg_color);
        canvas.copy(&shuff_tex, None, Rect::new(10, 100, 500, 40)).unwrap();


        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 3_000_000));
    }
}
