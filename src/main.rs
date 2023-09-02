pub mod asciicontext;
pub mod drawables;
pub mod drawingcontext;
pub mod terminaldrawable;

use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use std::io::{stdout, Write};
use std::{thread, time};
use termion::async_stdin;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::terminal_size;

fn main() {
    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new(term_size);
    let mut stdin = async_stdin().keys();
    let mut stdout: RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();

    let frame_fps = 30;
    let frame_len = time::Duration::from_micros(1000000 / frame_fps);

    let mut x: f32 = 1.0;
    let mut y: f32 = 1.0;

    loop {
        let frame_time = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Up => y -= 1.0,
                    Key::Down => y += 1.0,
                    Key::Left => x -= 1.0,
                    Key::Right => x += 1.0,
                    key => {
                        print!("Key pressed: {:?}", key);
                    }
                },
                _ => {}
            },
            _ => {}
        }

        print!("{}", termion::cursor::Goto(1, 1));

        let triangle = Triangle {
            points: [(x, y), (x + 10.0, y + 10.0), (x + 6.0, y + 20.0)],
            colors: [0.0, 1.0, 0.5],
            color: ColorPalette::Blue,
        };

        /*
        for x in 0..term_size.0 {
            for y in 0..term_size.1 {
                scr.set((x, y), 0);
            }
        }
        */

        scr.draw_triangles(&[triangle].to_vec());

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
