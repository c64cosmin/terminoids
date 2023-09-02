pub mod asciicontext;
pub mod drawables;
pub mod drawingcontext;
pub mod terminaldrawable;

use crate::asciicontext::AsciiContext;
use crate::drawingcontext::DrawingContext;
use std::io::{stdin, stdout, Write};
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

    let mut x: u16 = 10;
    let mut y: u16 = 10;
    let mut v: u8 = 1;

    loop {
        let frame_time = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Up => y -= 1,
                    Key::Down => y += 1,
                    Key::Left => x -= 1,
                    Key::Right => x += 1,
                    key => {
                        print!("Key pressed: {:?}", key);
                    }
                },
                _ => {}
            },
            _ => {}
        }

        print!("{}", termion::cursor::Goto(1, 1));

        scr.set((x, y), v);
        v += 1;

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
