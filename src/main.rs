pub mod asciicontext;
pub mod asteroid;
pub mod bullet;
pub mod drawables;
pub mod drawingcontext;
pub mod enemy;
pub mod powerup;
pub mod ship;
pub mod sprite;
pub mod starship;
pub mod terminaldrawable;

use crate::asciicontext::AsciiContext;
use crate::bullet::*;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::enemy::*;
use crate::ship::*;
use crate::sprite::Sprite;
use crate::terminaldrawable::TerminalDrawble;
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
    let mut delta_time: f32 = frame_len.as_micros() as f32 / 1000000.0;

    let camera = Camera {
        position: (0.0, 0.0),
        size: (term_size.0 as f32, term_size.1 as f32),
        zoom: 2.0,
    };

    let mut enemies: Enemies = Enemies::new();

    let mut ship = Ship::new();
    let mut ship_bullets = Bullets::new();

    loop {
        let frame_start = time::Instant::now();

        match stdin.next() {
            Some(result) => match result {
                Ok(key) => match key {
                    Key::Ctrl('c') | Key::Char('q') | Key::Esc => {
                        break;
                    }
                    Key::Left => ship.turn_left(),
                    Key::Right => ship.turn_right(),
                    Key::Up => ship.thrust(),
                    Key::Char(' ') => ship.fire(&mut ship_bullets),
                    key => {
                        print!("Key pressed: {:?}", key);
                    }
                },
                _ => {}
            },
            _ => {}
        }

        if ship.life < 0 {
            break;
        }

        //update
        ship.update(&camera, delta_time);
        ship_bullets.update(&camera, delta_time);
        enemies.update_with_ship(&camera, delta_time, &ship);
        enemies.collide_with_bullets(&mut ship_bullets, &mut ship);
        enemies.collide_with_ship(&camera, &mut ship);

        print!("{}", termion::cursor::Goto(1, 1));

        scr.flush_triangles();
        scr.flush_points();
        scr.clear();

        enemies.draw(&mut scr);
        ship.draw(&mut scr);
        ship_bullets.draw(&mut scr);

        scr.draw_triangles(&camera);
        scr.draw_points(&camera);
        scr.lifes = ship.life;
        scr.score = ship.score;
        scr.display();

        print!("{}", termion::cursor::Goto(1, 1));
        print!("{}{}", color::Black.bg_str(), color::White.fg_str());
        print!("\nFPS{:?}", frame_start.elapsed());

        stdout.flush().unwrap();

        if let Some(i) = (frame_len).checked_sub(frame_start.elapsed()) {
            thread::sleep(i)
        }

        delta_time =
            time::Instant::now().duration_since(frame_start).as_micros() as f32 / 1000000.0;
    }

    print!("{}", termion::clear::All);
    stdout.suspend_raw_mode().unwrap();

    println!("Your score was : {}", ship.score);
}

fn main() {
    start();
}
