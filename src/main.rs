/* ************************************************************************** */
/*                                                                            */
/*                                                            .               */
/*   main.rs                                                 / \              */
/*                                                          /   \             */
/*   By: charles <charles.cabergs@gmail.com>               /o  o \            */
/*                                                        /  v    \           */
/*   Created: 2020/06/25 14:38:48 by charles             /    _    \          */
/*   Updated: 2020/06/25 15:35:58 by charles            '-----------'         */
/*                                                                            */
/* ************************************************************************** */

use sdl2::ttf;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

pub mod time;
pub mod scramble;
pub mod history;

use scramble::Scramble;
use history::History;

const WIDTH:  u32  = 640;
const HEIGHT: u32  = 480;
const TITLE:  &str = "rutikmer";
const FONT_SIZE: u32 = 20;

const CENTER_Y: u32 = HEIGHT / 2;
const CENTER_X: u32 = WIDTH / 2;

const WHITE:  Color = Color::RGB(0xff, 0xff, 0xff);
const GREEN:  Color = Color::RGB(0x1B, 0x5E, 0x20);
const ORANGE: Color = Color::RGB(0xEF, 0x6C, 0x00);
const BLACK:  Color = Color::RGB(0x00, 0x00, 0x00);

const SCRAMBLE_LEN: usize = 14;
const SUMMARY_LEN:  usize = 12;
const HISTORY_FILE_PATH: &str = "history.csv";

fn str_to_tex<'a, T>(s: &str, font: &ttf::Font, creator: &'a TextureCreator<T>, bg: Color) -> Texture<'a> {
    let surface = font.render(s).shaded(WHITE, bg).unwrap();
    creator.create_texture_from_surface(&surface).unwrap()
}

fn pad_rect(rect: &mut Rect, pad: u32) {
    rect.resize(rect.width() - pad, rect.height() - pad);
    rect.offset(pad as i32 / 2, pad as i32 / 2);
}

fn rect_text(frame: &Rect, len: usize) -> Rect {
    let mut rect = *frame;
    rect.set_width(len as u32 * FONT_SIZE);
    rect.set_height(2 * FONT_SIZE);
    if rect.width() > frame.width() {
        rect.set_height(rect.height() * (rect.width() / frame.width()));
        rect.set_width(frame.width());
    }
    rect.center_on(frame.center());
    rect
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let ttf = ttf::init().unwrap();
    let video_subsys = sdl.video().unwrap();
    let window = video_subsys
        .window(TITLE, WIDTH, HEIGHT)
        .build()
        .unwrap();
    std::fs::copy(HISTORY_FILE_PATH, HISTORY_FILE_PATH.to_owned() + ".backup").unwrap();
    let font = ttf.load_font("font/FiraMono-Regular.ttf", FONT_SIZE as u16).unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let tex_creator = canvas.texture_creator();

    let mut hist = History::from_csv(HISTORY_FILE_PATH);
    let mut timer = time::Timer::new();
    let mut scramble = Scramble::new_rand(SCRAMBLE_LEN);
    let mut scramble_str = scramble.to_string();
    let mut scramble_tex = str_to_tex(&scramble_str, &font, &tex_creator, BLACK);
    let mut timer_tex = str_to_tex(&timer.to_string(), &font, &tex_creator, BLACK);

    'running: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    match timer.state {
                        time::State::Inactive => {
                            timer.idle();
                        },
                        time::State::Idle     => {},
                        time::State::Active   => {
                            timer.stop();
                            timer_tex = str_to_tex(&timer.to_string(), &font, &tex_creator, BLACK);
                            hist.push(&timer, &scramble);
                            scramble = Scramble::new_rand(SCRAMBLE_LEN);
                            scramble_str = scramble.to_string();
                            scramble_tex = str_to_tex(&scramble_str, &font, &tex_creator, BLACK);
                        },
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    if timer.state == time::State::Idle {
                        timer.start();
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    if timer.state == time::State::Inactive {
                        hist.pop();
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::U), repeat: false, .. } => {
                    if timer.state == time::State::Inactive {
                        hist.undo_pop();
                    }
                },
                _ => {}
            }
        }
        let bg_color = match timer.state {
            time::State::Inactive => BLACK,
            time::State::Idle     => ORANGE,
            time::State::Active   => GREEN,
        };
        canvas.set_draw_color(bg_color);
        canvas.clear();

        match timer.state {
            time::State::Inactive => {
                let mut history_rect  = Rect::new(0, 0, WIDTH/ 3, HEIGHT);
                let mut scramble_rect = Rect::new(WIDTH as i32 / 3, 0, 2 * WIDTH / 3, HEIGHT / 3);
                let mut timer_rect    = Rect::new(WIDTH as i32 / 3, HEIGHT as i32 / 3,
                                                  2 * WIDTH / 3, 2 * HEIGHT / 3);
                canvas.set_draw_color(WHITE);
                canvas.draw_rect(history_rect).unwrap();
                canvas.draw_rect(scramble_rect).unwrap();
                canvas.draw_rect(timer_rect).unwrap();

                pad_rect(&mut history_rect, 10);
                pad_rect(&mut scramble_rect, 20);
                pad_rect(&mut timer_rect, 40);

                let scramble_text_rect = rect_text(&scramble_rect, scramble_str.len());
                let timer_text_rect = rect_text(&timer_rect, timer.to_string().len());

                canvas.copy(&scramble_tex, None, scramble_text_rect).unwrap();
                canvas.copy(&timer_tex, None, timer_text_rect).unwrap();

                let mut entry_rect = history_rect;
                entry_rect.set_height(history_rect.height() / SUMMARY_LEN as u32);
                for entry in hist.summarize(SUMMARY_LEN) {
                    let entry_tex = str_to_tex(&entry.to_string(), &font, &tex_creator, BLACK);
                    canvas.copy(&entry_tex, None, entry_rect).unwrap();
                    entry_rect.set_y(entry_rect.y() + history_rect.height() as i32 / SUMMARY_LEN as i32);
                }
            },
            time::State::Idle => {},
            time::State::Active => {
                let s = timer.to_string();
                timer_tex = str_to_tex(&s, &font, &tex_creator, GREEN);
                let h: u32 = FONT_SIZE * 2;
                let w: u32 = s.len() as u32 * FONT_SIZE;
                let dest = Rect::new((CENTER_X - w / 2) as i32, (CENTER_Y - h / 2) as i32, w, h);
                canvas.copy(&timer_tex, None, dest).unwrap();
            },
        }

        canvas.present();
        if timer.state != time::State::Active {
            std::thread::sleep(std::time::Duration::new(0, 40_000_000));
        } else {
            std::thread::sleep(std::time::Duration::new(0, 10_000_000));
        }
    }
    hist.save_csv(HISTORY_FILE_PATH);
}

/* inactive (all black)
 * +------------+---------------------+
 * | history    |   scramble          |
 * |            |                     |
 * |            |                     |
 * |            |                     |
 * |            |---------------------|
 * |            |                     |
 * |            |   timer             |
 * |            |                     |
 * |            |                     |
 * +------------+---------------------+
 * idle (all yellow)
 * +------------+---------------------+
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * +------------+---------------------+
 * active (all green)
 * +------------+---------------------+
 * |                                  |
 * |                                  |
 * |                                  |
 * |       timer                      |
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * |                                  |
 * +------------+---------------------+
 */
