use std::time::SystemTime;
use sdl2::ttf;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsys = sdl.video().unwrap();
    let ttf = ttf::init().unwrap();
    let font = ttf.load_font("font/FiraMono-Regular.ttf", 40).unwrap();

    let window = video_subsys
        .window("rutikmer", 640, 480)
        .build()
        .unwrap();

    let mut timming = false;
    let mut time_start = SystemTime::now();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    timming = !timming;
                    if timming {
                        time_start = SystemTime::now();
                    }
                },
                _ => {}
            }
        }

        canvas.set_draw_color(if !timming { Color::RGB(0, 0, 0) } else { Color::RGB(30, 150, 30) });
        canvas.clear();
        if timming {
            let surface = font.render(&time_start.elapsed().unwrap().as_millis().to_string())
                              .solid(Color::RGB(255, 255, 255))
                              .unwrap();
            let tex = texture_creator.create_texture_from_surface(&surface).unwrap();
            canvas.copy(&tex, None, None).unwrap();
        }
        canvas.present();
    }
}
