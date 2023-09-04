use crate::asciicontext::AsciiContext;
use crate::asteroid::*;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::enemy::*;
use crate::powerup::PowerupSize;
use crate::powerup::*;
use crate::sprite::Sprite;
use crate::starship::StarShipSize;
use crate::starship::*;
use crate::terminaldrawable::TerminalDrawble;
use std::io::Write;
use std::{thread, time};
use termion::event::Key;
use termion::input::Keys;
use termion::raw::RawTerminal;
use termion::terminal_size;
use termion::AsyncReader;

pub fn menu_objects(stdin: &mut Keys<AsyncReader>, stdout: &mut RawTerminal<std::io::Stdout>) {
    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new(term_size);

    let frame_fps = 24;
    let frame_len = time::Duration::from_micros(1000000 / frame_fps);
    let mut delta_time: f32 = frame_len.as_micros() as f32 / 1000000.0;

    let camera = Camera {
        position: (0.0, 0.0),
        size: (term_size.0 as f32, term_size.1 as f32),
        zoom: 2.0,
    };

    let messages = ["New game", "Help", "Objects", "Exit"];
    let mut message_selection: i8 = 0;

    loop {
        let frame_start = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Ctrl('c') | Key::Esc | Key::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        print!("{}", termion::cursor::Goto(1, 1));

        scr.flush_text_entries();
        scr.flush_triangles();
        scr.flush_points();
        scr.clear();

        scr.draw_triangles(&camera);
        scr.draw_points(&camera);
        scr.display();

        stdout.flush().unwrap();

        if let Some(i) = (frame_len).checked_sub(frame_start.elapsed()) {
            thread::sleep(i)
        }

        delta_time =
            time::Instant::now().duration_since(frame_start).as_micros() as f32 / 1000000.0;
    }
}
