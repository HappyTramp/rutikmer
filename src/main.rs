use sdl2::ttf;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub mod time;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsys = sdl.video().unwrap();
    let ttf = ttf::init().unwrap();
    let font = ttf.load_font("font/FiraMono-Regular.ttf", 40).unwrap();

    let window = video_subsys
        .window("rutikmer", 640, 480)
        .build()
        .unwrap();

    let mut timer = time::Timer::new();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    match timer.state {
                        time::State::Active   => timer.stop(),
                        time::State::Inactive => timer.idle(),
                        time::State::Idle     => {},
                    }
                }
                Event::KeyUp   { keycode: Some(Keycode::Space), .. } => {
                    if timer.state == time::State::Idle {
                        timer.start();
                    }
                },
                _ => {}
            }
        }
        match timer.state {
            time::State::Idle     => canvas.set_draw_color(Color::RGB(100, 100, 0)),
            time::State::Active   => canvas.set_draw_color(Color::RGB(0, 100, 0)),
            time::State::Inactive => canvas.set_draw_color(Color::RGB(0, 0, 0)),
        }
        canvas.clear();

        if timer.state != time::State::Idle {
            canvas.copy(&timer.to_texture(&font, &texture_creator), None, None).unwrap();
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 3_000_000));
    }
}
