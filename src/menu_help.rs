use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::plasma::*;
use std::io::Write;
use std::{thread, time};
use termion::event::Key;
use termion::input::Keys;
use termion::raw::RawTerminal;
use termion::terminal_size;
use termion::AsyncReader;

pub fn menu_help(stdin: &mut Keys<AsyncReader>, stdout: &mut RawTerminal<std::io::Stdout>) {
    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new(term_size);

    let frame_fps = 10;
    let frame_len = time::Duration::from_micros(1000000 / frame_fps);
    let mut delta_time: f32 = frame_len.as_micros() as f32 / 1000000.0;

    let camera = Camera {
        position: (0.0, 0.0),
        size: (term_size.0 as f32, term_size.1 as f32),
        zoom: 2.0,
    };

    let mut plasma = FireDrawer::new(term_size);

    let messages = [
        "Unfortunately terminals don't directly support input like UI apps do.",
        "What I mean is that KEY_UP & KEY_DOWN events are not supported,",
        "the way the terminal works is to get a key stroke event and act accordingly.",
        "",
        "This means that buttons have to be tapped in order to get a move",
        "holding a button down will work until another button is pressed",
        "Remember this works inside the terminal, so like a text editor",
        "You cannot press two buttons at once, you got to tap them.",
        "",
        "Due to this limitation the game work like this:",
        "",
        "Pressing **Fire** button will enable firing,",
        "the ship will fire automatically",
        "until the **Fire** is pressed again to stop",
        "",
        "Pressing **Left** or **Right** will turn the ship only a slight amount",
        "",
        "Pressing **Up** will propel the ship forward a slight amount",
        "",
        "",
        "Left - Right  : Rotate ship ",
        "Up            : Move forward",
        "Space         : Fire        ",
        "P             : Pause       ",
        "Q or Ctrl-C   : Exit        ",
    ];

    print!("{}", termion::cursor::Goto(1, 1));

    scr.flush_text_entries();
    scr.flush_triangles();
    scr.flush_points();
    scr.clear();

    for i in 0..messages.len() {
        let message_y = 6;
        let message = String::from(messages[i]);
        let message_x = (term_size.0 - message.len() as u16) / 2;
        scr.add_text_entry(&TextEntry {
            position: (message_x as f32, message_y as f32 + i as f32),
            string: message,
            color_palette: TextColorPalette::Text,
        });
    }

    scr.draw_triangles(&camera);
    scr.draw_points(&camera);
    scr.display();

    loop {
        let frame_start = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Ctrl('c') | Key::Esc | Key::Char('q') | Key::Char('\n') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        plasma.update(delta_time);

        plasma.draw();

        stdout.flush().unwrap();

        if let Some(i) = (frame_len).checked_sub(frame_start.elapsed()) {
            thread::sleep(i)
        }

        delta_time =
            time::Instant::now().duration_since(frame_start).as_micros() as f32 / 1000000.0;
    }
}
