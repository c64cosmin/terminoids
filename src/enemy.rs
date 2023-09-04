use crate::asciicontext::AsciiContext;
use crate::asteroid::*;
use crate::bullet::*;
use crate::drawables::*;
use crate::powerup::Powerup;
use crate::ship::*;
use crate::sprite::*;
use crate::starship::*;
use crate::terminaldrawable::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum EnemyType {
    Asteroid(Asteroid),
    StarShip(StarShip),
    Powerup(Powerup),
}

pub struct Enemies {
    pub enemies: Vec<EnemyType>,
    time: f32,
    time_interval: f32,
    level: u8,
    level_interval: u8,
}

impl Enemies {
    pub fn new() -> Enemies {
        Enemies {
            enemies: Vec::with_capacity(100),
            time: 60.0,
            time_interval: 40.0,
            level: 0,
            level_interval: 8,
        }
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
                EnemyType::Powerup(_) => false,
            })
            .count();
    }

    pub fn update_with_ship(&mut self, camera: &Camera, delta: f32, ship: &Ship) {
        self.update(camera, delta);

        self.spawn_stuff(camera, ship);
    }

    fn spawn_stuff(&mut self, camera: &Camera, ship: &Ship) {
        if (self.time < 0.0 || self.get_entities_no() == 0) && self.get_entities_no() < 8 {
            let mut rnd = rand::thread_rng();
            let choice: u8 = rnd.gen_range(0..self.level_interval);
            if choice > 0 {
                if let Some(asteroid) = self.spawn::<Asteroid>(camera, ship) {
                    self.level += 1;
                    if self.level > self.level_interval {
                        self.level = 0;
                        self.level_interval = (self.level_interval - 1).max(2);
                    }
                    self.enemies.push(EnemyType::Asteroid(asteroid));
                    self.time = self.time_interval;
                }
            } else {
                if let Some(starship) = self.spawn::<StarShip>(camera, ship) {
                    self.enemies.push(EnemyType::StarShip(starship));
                    self.time = self.time_interval;
                }
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

    fn get_empty_point(&self, camera: &Camera) -> Vec2 {
        let mut rnd = rand::thread_rng();
        let bounds = camera.get_bounds();
        let mut position: (f32, f32) = (0.0, 0.0);

        for _ in 0..32 {
            let mut found_space = true;

            self.enemies.iter().for_each(|enemy| {
                let enemy_position = match enemy {
                    EnemyType::Asteroid(a) => a.position,
                    EnemyType::StarShip(s) => s.position,
                    EnemyType::Powerup(p) => p.position,
                };
                if distance(position, enemy_position) < 10.0 {
                    found_space = false;
                }
            });

            if found_space {
                return position;
            }

            position = (
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
        }

        return position;
    }

    pub fn collide_with_ship(&mut self, camera: &Camera, ship: &mut Ship) {
        self.enemies.iter().for_each(|enemy| {
            if enemy.collide_with_ship(ship) {
                match enemy {
                    EnemyType::Powerup(powerup) => ship.powerup(&powerup),
                    _ => ship.damage(self.get_empty_point(camera)),
                }
            }
        });
    }

    pub fn collide_with_bullets(&mut self, bullets: &mut Bullets, ship: &mut Ship) {
        let mut new_objects: Vec<EnemyType> = Vec::<EnemyType>::with_capacity(20);
        let damaged: Vec<usize> = self.damage(&self.enemies, bullets);
        damaged.iter().for_each(|&i| {
            Collidable::split(&self.enemies[i])
                .iter()
                .for_each(|obj| new_objects.push(obj.clone()));
        });
        damaged.iter().rev().for_each(|&i| {
            ship.score += match self.enemies[i] {
                EnemyType::Powerup(p) => {
                    ship.powerup(&p);
                    1000
                }
                EnemyType::Asteroid(a) => match a.size {
                    AsteroidSize::Huge => 200,
                    AsteroidSize::Big => 150,
                    AsteroidSize::Medium => 100,
                    AsteroidSize::Small => 75,
                    AsteroidSize::Tiny => 50,
                },
                EnemyType::StarShip(s) => match s.size {
                    StarShipSize::BigCluster => 250,
                    StarShipSize::MediumCluster => 200,
                    StarShipSize::SmallCluster => 150,
                    StarShipSize::Flying => 100,
                },
            };
            self.enemies.remove(i);
        });

        new_objects.iter().for_each(|n| match n {
            EnemyType::Asteroid(a) => self.enemies.push(EnemyType::Asteroid(a.clone())),
            EnemyType::StarShip(s) => self.enemies.push(EnemyType::StarShip(s.clone())),
            EnemyType::Powerup(s) => self.enemies.push(EnemyType::Powerup(s.clone())),
        });
    }

    fn damage(&self, collection: &Vec<EnemyType>, bullets: &mut Bullets) -> Vec<usize> {
        let mut damaged = Vec::<usize>::with_capacity(10);
        collection.iter().enumerate().for_each(|(i, obj)| {
            let mut collided = false;

            bullets.bullets.iter_mut().for_each(|bullet| {
                if obj.collide(bullet.position) {
                    collided = true;
                    bullet.destroy();
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
            EnemyType::Powerup(p) => p.draw(ctx),
        });
    }
}

impl Sprite for Enemies {
    fn update(&mut self, camera: &Camera, delta: f32) {
        self.time -= delta;
        self.time_interval = (self.time_interval - delta / 120.0).max(1.0);

        self.enemies.iter_mut().for_each(|obj| match obj {
            EnemyType::Asteroid(a) => a.update(camera, delta),
            EnemyType::StarShip(s) => s.update(camera, delta),
            EnemyType::Powerup(p) => p.update(camera, delta),
        });
        self.enemies.retain(|obj| match obj {
            EnemyType::Asteroid(a) => a.is_alive(),
            EnemyType::StarShip(s) => s.is_alive(),
            EnemyType::Powerup(_) => true,
        });
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}

impl Collidable for EnemyType {
    fn collide(&self, point: Vec2) -> bool {
        match self {
            EnemyType::Asteroid(a) => a.collide(point),
            EnemyType::StarShip(s) => s.collide(point),
            EnemyType::Powerup(p) => p.collide(point),
        }
    }

    fn collide_with_ship(&self, ship: &Ship) -> bool {
        if ship.shield > 0.0 {
            return false;
        }

        match self {
            EnemyType::Asteroid(a) => a.collide_with_ship(ship),
            EnemyType::StarShip(s) => s.collide_with_ship(ship),
            EnemyType::Powerup(p) => p.collide_with_ship(ship),
        }
    }

    fn split(&self) -> Vec<EnemyType> {
        match self {
            EnemyType::Asteroid(a) => a.split(),
            EnemyType::StarShip(s) => s.split(),
            EnemyType::Powerup(p) => p.split(),
        }
    }
}
