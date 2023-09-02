use crate::asciicontext::AsciiContext;
use crate::asteroid::*;
use crate::drawables::*;
use crate::ship::*;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

pub struct Enemies {
    asteroids: Vec<Asteroid>,
    time: f32,
}

impl Enemies {
    pub fn new() -> Enemies {
        Enemies {
            asteroids: Vec::with_capacity(100),
            time: 0.0,
        }
    }

    pub fn init_level(&mut self, level: u16) {
        self.asteroids.clear();
    }

    pub fn update_with_ship(&mut self, camera: &Camera, delta: f32, ship: &Ship) {
        self.update(camera, delta);

        if self.time > 5.0 {
            let mut rnd = rand::thread_rng();
            let bounds = camera.get_bounds();
            let position: (f32, f32) = (
                (rnd.gen::<f32>() * bounds.0)
                    * match rand::random() {
                        true => -1.0,
                        false => 1.0,
                    },
                (rnd.gen::<f32>() * bounds.1)
                    * match rand::random() {
                        true => -1.0,
                        false => 1.0,
                    },
            );

            if distance(ship.position, position) > 7.0 {
                self.asteroids.push(Asteroid::new(position));
                self.time = 0.0;
            }
        }
    }
}

impl TerminalDrawble for Enemies {
    fn draw(&self, ctx: &mut AsciiContext) {
        self.asteroids.iter().for_each(|a| a.draw(ctx));
    }
}

impl Sprite for Enemies {
    fn update(&mut self, camera: &Camera, delta: f32) {
        self.time += delta;
        self.asteroids
            .iter_mut()
            .for_each(|a| a.update(camera, delta));
        self.asteroids.retain(|a| a.is_alive());
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}
