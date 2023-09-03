use crate::asciicontext::AsciiContext;
use crate::asteroid::*;
use crate::bullet::*;
use crate::drawables::*;
use crate::ship::*;
use crate::sprite::*;
use crate::starship::*;
use crate::terminaldrawable::*;
use rand::Rng;

pub enum EnemyType {
    Asteroid,
    StarShip,
}

pub struct Enemies {
    pub asteroids: Vec<Asteroid>,
    pub ships: Vec<StarShip>,
    time: f32,
}

impl Enemies {
    pub fn new() -> Enemies {
        Enemies {
            asteroids: Vec::with_capacity(100),
            ships: Vec::with_capacity(100),
            time: 0.0,
        }
    }

    pub fn init_level(&mut self, camera: &Camera, ship: &Ship) {
        for _ in 0..3 {
            if let Some(asteroid) = self.spawn::<Asteroid>(camera, ship) {
                self.asteroids.push(asteroid);
            }
        }
        self.time = 30.0;
    }

    fn get_entities_no(&self) -> usize {
        return self
            .asteroids
            .iter()
            .filter(|a| match a.size {
                AsteroidSize::Huge | AsteroidSize::Big | AsteroidSize::Medium => true,
                _ => false,
            })
            .count();
    }

    pub fn update_with_ship(&mut self, camera: &Camera, delta: f32, ship: &Ship) {
        self.update(camera, delta);

        if (self.time < 0.0 || self.get_entities_no() == 0) && self.get_entities_no() < 8 {
            if let Some(asteroid) = self.spawn::<Asteroid>(camera, ship) {
                self.asteroids.push(asteroid);
                self.time = 10.0;
            }
        }
    }

    fn spawn<T: Spawnable>(&mut self, camera: &Camera, ship: &Ship) -> Option<T> {
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
            return Some(T::spawn(position));
        }

        None
    }

    pub fn collide(&mut self, bullets: &mut Bullets) {
        let mut new_asteroids: Vec<Asteroid> = Vec::<Asteroid>::with_capacity(10);
        let damaged: Vec<usize> = self.damage::<Asteroid>(&self.asteroids, bullets);
        damaged.iter().for_each(|&i| match self.asteroids[i].size {
            AsteroidSize::Tiny => {}
            _ => self.asteroids[i]
                .split()
                .iter()
                .for_each(|a| new_asteroids.push(a.clone())),
        });
        damaged.iter().rev().for_each(|&i| {
            self.asteroids.remove(i);
        });

        new_asteroids
            .iter()
            .for_each(|na| self.asteroids.push(na.clone()));
    }

    fn damage<T: Collidable>(&self, collection: &Vec<T>, bullets: &mut Bullets) -> Vec<usize> {
        let mut damaged = Vec::<usize>::with_capacity(10);
        collection.iter().enumerate().for_each(|(i, obj)| {
            let mut collided = false;

            bullets.bullets.iter_mut().for_each(|bullet| {
                if obj.collide(bullet.position) {
                    collided = true;
                    bullet.life = 0.0;
                }
            });

            if collided {
                damaged.push(i);
            }
        });

        damaged
    }
}

impl TerminalDrawble for Enemies {
    fn draw(&self, ctx: &mut AsciiContext) {
        self.asteroids.iter().for_each(|a| a.draw(ctx));
    }
}

impl Sprite for Enemies {
    fn update(&mut self, camera: &Camera, delta: f32) {
        self.time -= delta;
        self.asteroids
            .iter_mut()
            .for_each(|a| a.update(camera, delta));
        self.asteroids.retain(|a| a.is_alive());
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}
