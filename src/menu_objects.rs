use crate::asciicontext::AsciiContext;
use crate::asteroid::*;
use crate::bullet::*;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::enemy::*;
use crate::powerup::PowerupSize;
use crate::powerup::*;
use crate::ship::*;
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

    let stationary = (0.0, 0.0);
    let mut objects: Enemies = Enemies::new();
    objects.enemies.push(EnemyType::Asteroid(Asteroid {
        position: (-20.0, -10.0),
        speed: stationary,
        angle: 0.0,
        size: AsteroidSize::Huge,
        angle_speed: 0.1,
        color_palette: Asteroid::get_random_color(),
    }));
    objects.enemies.push(EnemyType::Asteroid(Asteroid {
        position: (-10.0, -10.0),
        speed: stationary,
        angle: 0.0,
        size: AsteroidSize::Big,
        angle_speed: 0.2,
        color_palette: Asteroid::get_random_color(),
    }));
    objects.enemies.push(EnemyType::Asteroid(Asteroid {
        position: (0.0, -10.0),
        speed: stationary,
        angle: 0.0,
        size: AsteroidSize::Medium,
        angle_speed: 0.3,
        color_palette: Asteroid::get_random_color(),
    }));
    objects.enemies.push(EnemyType::Asteroid(Asteroid {
        position: (10.0, -10.0),
        speed: stationary,
        angle: 0.0,
        size: AsteroidSize::Small,
        angle_speed: 0.4,
        color_palette: Asteroid::get_random_color(),
    }));
    objects.enemies.push(EnemyType::Asteroid(Asteroid {
        position: (20.0, -10.0),
        speed: stationary,
        angle: 0.0,
        size: AsteroidSize::Tiny,
        angle_speed: 0.5,
        color_palette: Asteroid::get_random_color(),
    }));
    objects.enemies.push(EnemyType::StarShip(StarShip {
        position: (-15.0, 1.0),
        speed: stationary,
        angle: 0.0,
        size: StarShipSize::BigCluster,
        angle_speed: 0.03,
        disabled: true,
    }));
    objects.enemies.push(EnemyType::StarShip(StarShip {
        position: (-2.3, 1.0),
        speed: stationary,
        angle: 0.0,
        size: StarShipSize::MediumCluster,
        angle_speed: 0.1,
        disabled: true,
    }));
    objects.enemies.push(EnemyType::StarShip(StarShip {
        position: (11.7, 1.0),
        speed: stationary,
        angle: 0.0,
        size: StarShipSize::SmallCluster,
        angle_speed: 0.2,
        disabled: true,
    }));
    objects.enemies.push(EnemyType::StarShip(StarShip {
        position: (20.0, 1.0),
        speed: stationary,
        angle: 0.0,
        size: StarShipSize::Flying,
        angle_speed: 0.4,
        disabled: true,
    }));
    objects.enemies.push(EnemyType::Powerup(Powerup {
        position: (-15.0, 10.0),
        speed: stationary,
        life: 0.0,
        size: PowerupSize::RapidFire,
    }));
    objects.enemies.push(EnemyType::Powerup(Powerup {
        position: (-5.0, 10.0),
        speed: stationary,
        life: 0.0,
        size: PowerupSize::SplitFire,
    }));
    objects.enemies.push(EnemyType::Powerup(Powerup {
        position: (5.0, 10.0),
        speed: stationary,
        life: 0.0,
        size: PowerupSize::PiercingBullets,
    }));
    objects.enemies.push(EnemyType::Powerup(Powerup {
        position: (15.0, 10.0),
        speed: stationary,
        life: 0.0,
        size: PowerupSize::Shield,
    }));

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

        objects.update(&camera, delta_time);

        print!("{}", termion::cursor::Goto(1, 1));

        scr.flush_text_entries();
        scr.flush_triangles();
        scr.flush_points();
        scr.clear();

        scr.add_text_entry(&TextEntry {
            position: (2.0, 2.0),
            string: String::from("Press ESC to exit"),
            color_palette: TextColorPalette::Warning,
        });
        scr.add_text_entry(&TextEntry {
            position: (10.0, 10.0),
            string: String::from("Asteroids"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (10.0, 31.0),
            string: String::from("Star-ships"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (10.0, 48.0),
            string: String::from("Powerups"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (54.0, 56.0),
            string: String::from("Rapid Fire"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (94.0, 56.0),
            string: String::from("Split Fire"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (132.0, 56.0),
            string: String::from("Piercing Bullets"),
            color_palette: TextColorPalette::Text,
        });
        scr.add_text_entry(&TextEntry {
            position: (177.0, 56.0),
            string: String::from("Shield"),
            color_palette: TextColorPalette::Text,
        });

        objects.draw(&mut scr);

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
