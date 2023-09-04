use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::game::*;
use crate::logo::*;
use crate::menu_help::*;
use crate::menu_objects::*;
use crate::sprite::Sprite;
use crate::terminaldrawable::TerminalDrawble;
use std::io::{stdout, Write};
use std::{thread, time};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::terminal_size;

pub fn menu() {
    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new(term_size);
    let mut stdin = async_stdin().keys();
    let mut stdout: RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();

    let frame_fps = 24;
    let frame_len = time::Duration::from_micros(1000000 / frame_fps);
    let mut delta_time: f32 = frame_len.as_micros() as f32 / 1000000.0;

    let camera = Camera {
        position: (0.0, 0.0),
        size: (term_size.0 as f32, term_size.1 as f32),
        zoom: 2.0,
    };

    let mut logo: DrawbleLogo = DrawbleLogo::new();

    let messages = ["New game", "Help", "Objects", "Exit"];
    let mut message_selection: i8 = 0;

    loop {
        let frame_start = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Ctrl('c') => {
                        break;
                    }
                    Key::Char('\n') => {
                        match message_selection {
                            0 => game(&mut stdin, &mut stdout),
                            1 => menu_help(&mut stdin, &mut stdout),
                            2 => menu_objects(&mut stdin, &mut stdout),
                            _ => break,
                        };
                    }
                    Key::Up => message_selection = (message_selection - 1).max(0),
                    Key::Down => {
                        message_selection = (message_selection + 1).min((messages.len() - 1) as i8)
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        logo.update(&camera, delta_time);

        print!("{}", termion::cursor::Goto(1, 1));

        scr.flush_text_entries();
        scr.flush_triangles();
        scr.flush_points();
        scr.clear();

        scr.add_text_entry(&TextEntry {
            position: (0.0, 4.0),
            string: String::from("Copyright 2023 - c64cosmin - Cosmin MUNTEANU"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (0.0, 0.0),
            string: String::from("Follow me on Twitter @c64cosmin  https://twitter.com/c64cosmin"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (0.0, 2.0),
            string: String::from("Or visit my webpage for more games  https://www.stupidrat.com"),
            color_palette: TextColorPalette::Text,
        });

        for i in 0..messages.len() {
            let message_y = (term_size.1 - messages.len() as u16 * 3) * 2 / 3;
            let mut message = String::from(messages[i]);
            if message_selection == i as i8 {
                message = format!(">>> {} <<<", messages[i]);
            }
            let message_x = (term_size.0 - message.len() as u16) / 2;
            scr.add_text_entry(&TextEntry {
                position: (message_x as f32, message_y as f32 + i as f32 * 3.0),
                string: message,
                color_palette: match message_selection == i as i8 {
                    true => TextColorPalette::Warning,
                    false => TextColorPalette::Menu,
                },
            });
        }

        logo.draw(&mut scr);

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

    print!("{}", termion::cursor::Goto(1, 1));
    print!("{}", termion::clear::All);
    stdout.suspend_raw_mode().unwrap();
}
