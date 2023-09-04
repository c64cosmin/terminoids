use crate::asciicontext::AsciiContext;
use crate::bullet::*;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::enemy::*;
use crate::ship::*;
use crate::sprite::Sprite;
use crate::terminaldrawable::TerminalDrawble;
use std::io::Write;
use std::{thread, time};
use termion::event::Key;
use termion::input::Keys;
use termion::raw::RawTerminal;
use termion::terminal_size;
use termion::AsyncReader;

pub fn game(stdin: &mut Keys<AsyncReader>, stdout: &mut RawTerminal<std::io::Stdout>) {
    let term_size = terminal_size().unwrap();
    let mut scr: AsciiContext = AsciiContext::new(term_size);

    let frame_fps = 30;
    let frame_len = time::Duration::from_micros(1000000 / frame_fps);
    let mut delta_time: f32 = frame_len.as_micros() as f32 / 1000000.0;

    let camera = Camera {
        position: (0.0, 0.0),
        size: (term_size.0 as f32, term_size.1 as f32),
        zoom: 2.0,
    };

    let mut enemies: Enemies = Enemies::new();

    let mut ship = Ship::new();
    let mut ship_bullets = Bullets::new();

    let mut paused = false;
    let mut paused_draw = false;

    loop {
        let frame_start = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Ctrl('c') | Key::Char('q') | Key::Esc => {
                        ship.damage((0.0, 0.0), true);
                    }
                    Key::Left => ship.turn_left(),
                    Key::Right => ship.turn_right(),
                    Key::Up => ship.thrust(),
                    Key::Char(' ') => ship.fire(),
                    Key::Char('p') | Key::Char('P') => paused = !paused,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        if !paused {
            paused_draw = false;

            if ship.life < 0 {
                break;
            }

            //update
            ship.update(&camera, delta_time);
            ship.update_switches(&mut ship_bullets);
            ship_bullets.update(&camera, delta_time);
            enemies.update_with_ship(&camera, delta_time, &ship);
            enemies.collide_with_bullets(&mut ship_bullets, &mut ship);
            enemies.collide_with_ship(&camera, &mut ship);

            //add the text
            scr.flush_text_entries();

            let score_string: String = format!("Score : {}", ship.score);
            let life_string: String = format!("Life : {}", ship.life);
            scr.add_text_entry(&TextEntry {
                position: (0.0, 0.0),
                string: score_string,
                color_palette: TextColorPalette::Menu,
            });
            scr.add_text_entry(&TextEntry {
                position: (0.0, 1.0),
                string: life_string,
                color_palette: TextColorPalette::Text,
            });

            scr.flush_triangles();
            scr.flush_points();
            scr.clear();

            enemies.draw_particles(&mut scr);
            scr.draw_points(&camera);
            scr.flush_points();

            enemies.draw(&mut scr);
            ship.draw(&mut scr);
            ship_bullets.draw(&mut scr);

            scr.draw_triangles(&camera);
            scr.draw_points(&camera);

            scr.display();

            stdout.flush().unwrap();
        } else {
            if !paused_draw {
                let messages = ["==============", ">>> PAUSED <<<", "=============="];
                let message_x = (term_size.0 - messages[0].len() as u16) / 2;
                let message_y = (term_size.1 - messages.len() as u16) / 2;

                scr.flush_text_entries();
                for i in 0..messages.len() {
                    scr.add_text_entry(&TextEntry {
                        position: (message_x as f32, message_y as f32 + i as f32),
                        string: String::from(messages[i]),
                        color_palette: TextColorPalette::Warning,
                    });
                }

                scr.display();

                stdout.flush().unwrap();

                paused_draw = true;
            }
        }

        if let Some(i) = (frame_len).checked_sub(frame_start.elapsed()) {
            thread::sleep(i)
        }

        delta_time =
            time::Instant::now().duration_since(frame_start).as_micros() as f32 / 1000000.0;
    }
}
