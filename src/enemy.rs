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
    Asteroid(Asteroid),
    StarShip(StarShip),
}

pub struct Enemies {
    pub enemies: Vec<EnemyType>,
    time: f32,
}

impl Enemies {
    pub fn new() -> Enemies {
        Enemies {
            enemies: Vec::with_capacity(100),
            time: 0.0,
        }
    }

    pub fn init_level(&mut self, camera: &Camera, ship: &Ship) {
        for _ in 0..1 {
            if let Some(obj) = self.spawn::<StarShip>(camera, ship) {
                self.enemies.push(EnemyType::StarShip(obj));
            }
        }
        for _ in 0..1 {
            if let Some(obj) = self.spawn::<Asteroid>(camera, ship) {
                self.enemies.push(EnemyType::Asteroid(obj));
            }
        }
        self.time = 30.0;
    }

    fn get_entities_no(&self) -> usize {
        return self
            .enemies
            .iter()
            .filter(|asteroid| match asteroid {
                EnemyType::Asteroid(a) => match a.size {
                    AsteroidSize::Huge | AsteroidSize::Big | AsteroidSize::Medium => true,
                    _ => false,
                },
                EnemyType::StarShip(s) => match s.size {
                    StarShipSize::BigCluster
                    | StarShipSize::MediumCluster
                    | StarShipSize::SmallCluster => true,
                    _ => false,
                },
            })
            .count();
    }

    pub fn update_with_ship(&mut self, camera: &Camera, delta: f32, ship: &Ship) {
        self.update(camera, delta);

        if (self.time < 0.0 || self.get_entities_no() == 0) && self.get_entities_no() < 8 {
            if let Some(asteroid) = self.spawn::<Asteroid>(camera, ship) {
                self.enemies.push(EnemyType::Asteroid(asteroid));
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
        let mut new_objects: Vec<EnemyType> = Vec::<EnemyType>::with_capacity(20);
        let damaged: Vec<usize> = self.damage(&self.enemies, bullets);
        damaged.iter().for_each(|&i| match self.enemies[i] {
            EnemyType::Asteroid(a) => a
                .split()
                .iter()
                .for_each(|obj| new_objects.push(EnemyType::Asteroid(obj.clone()))),
            EnemyType::StarShip(s) => s
                .split()
                .iter()
                .for_each(|obj| new_objects.push(EnemyType::StarShip(obj.clone()))),
        });
        damaged.iter().rev().for_each(|&i| {
            self.enemies.remove(i);
        });

        new_objects.iter().for_each(|n| match n {
            EnemyType::Asteroid(a) => self.enemies.push(EnemyType::Asteroid(a.clone())),
            EnemyType::StarShip(s) => self.enemies.push(EnemyType::StarShip(s.clone())),
        });
    }

    fn damage(&self, collection: &Vec<EnemyType>, bullets: &mut Bullets) -> Vec<usize> {
        let mut damaged = Vec::<usize>::with_capacity(10);
        collection.iter().enumerate().for_each(|(i, obj)| {
            let mut collided = false;

            bullets.bullets.iter_mut().for_each(|bullet| {
                if match obj {
                    EnemyType::Asteroid(a) => a.collide(bullet.position),
                    EnemyType::StarShip(s) => s.collide(bullet.position),
                } {
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
        self.enemies.iter().for_each(|obj| match obj {
            EnemyType::Asteroid(a) => a.draw(ctx),
            EnemyType::StarShip(s) => s.draw(ctx),
        });
    }
}

impl Sprite for Enemies {
    fn update(&mut self, camera: &Camera, delta: f32) {
        self.time -= delta;
        self.enemies.iter_mut().for_each(|obj| match obj {
            EnemyType::Asteroid(a) => a.update(camera, delta),
            EnemyType::StarShip(s) => s.update(camera, delta),
        });
        self.enemies.retain(|obj| match obj {
            EnemyType::Asteroid(a) => a.is_alive(),
            EnemyType::StarShip(s) => s.is_alive(),
        });
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}
