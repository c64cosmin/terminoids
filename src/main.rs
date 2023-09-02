pub mod asciicontext;
pub mod drawables;
pub mod drawingcontext;
pub mod ship;
pub mod sprite;
pub mod terminaldrawable;

use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::ship::*;
use std::io::{stdout, Write};
use std::{thread, time};
use termion::async_stdin;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::terminal_size;

fn start() {
    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new(term_size);
    let mut stdin = async_stdin().keys();
    let mut stdout: RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();

    let frame_fps = 30;
    let frame_len = time::Duration::from_micros(1000000 / frame_fps);

    let mut ship = Ship {
        position: (0.0, 0.0),
        speed: (0.0, 0.0),
        angle: 0.0,
    };
    let turn_speed = 5.0;

    loop {
        let frame_time = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Left => ship.angle -= turn_speed,
                    Key::Right => ship.angle += turn_speed,
                    key => {
                        print!("Key pressed: {:?}", key);
                    }
                },
                _ => {}
            },
            _ => {}
        }

        print!("{}", termion::cursor::Goto(1, 1));

        for x in 0..term_size.0 {
            for y in 0..term_size.1 {
                scr.set((x, y), 0);
            }
        }

        scr.add_triangles(
            &[
                Triangle {
                    points: [(0.0, 0.0), (0.2, 0.1), (0.06, 0.2)],
                    colors: [0.0, 1.0, 0.5],
                    color_palette: ColorPalette::Blue,
                },
                Triangle {
                    points: [(0.3, 0.1), (0.5, 0.2), (0.4, 0.4)],
                    colors: [0.0, 0.5, 1.0],
                    color_palette: ColorPalette::Blue,
                },
            ]
            .to_vec(),
        );

        scr.draw_triangles();

        scr.display();

        print!("{}", termion::cursor::Goto(1, 1));
        print!("{}{}", color::Black.bg_str(), color::White.fg_str());
        print!("FPS{:?}", frame_time.elapsed());

        stdout.flush().unwrap();

        if let Some(i) = (frame_len).checked_sub(frame_time.elapsed()) {
            thread::sleep(i)
        }
    }

    print!("{}", termion::clear::All);
    stdout.suspend_raw_mode().unwrap();
}

fn test() {
    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new((term_size.0, term_size.1 / 2));

    let x: f32 = 1.0;
    let y: f32 = 1.0;

    /*
    for x in 0..term_size.0 {
        for y in 0..term_size.1 {
            scr.set((x, y), 0);
        }
    }
    */

    scr.draw_triangles(
        &[
            Triangle {
                points: [(x, y), (x + 20.0, y + 10.0), (x + 6.0, y + 20.0)],
                colors: [0.0, 1.0, 0.5],
                color_palette: ColorPalette::Blue,
            },
            Triangle {
                points: [(30.0, 10.0), (50.0, 20.0), (40.0, 40.0)],
                colors: [0.0, 0.5, 1.0],
                color_palette: ColorPalette::Blue,
            },
        ]
        .to_vec(),
    );

    scr.display();
}

fn main() {
    start();
}
