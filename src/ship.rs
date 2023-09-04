use crate::asciicontext::AsciiContext;
use crate::bullet::*;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::powerup::*;
use crate::sprite::*;
use crate::terminaldrawable::*;

pub struct Ship {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
    pub life: i8,
    pub score: u32,
    fire_cooldown: f32,
    thrust_speed: f32,
    angle_speed: f32,
    turn_speed: f32,
    spawning: f32,
    piercing: f32,
    splitfire: f32,
    rapidfire: f32,
    pub shield: f32,
    firing: bool,
}

impl TerminalDrawble for Ship {
    fn draw(&self, ctx: &mut AsciiContext) {
        if self.spawning > 0.0 {
            let r = self.spawning * 13.0;
            let n = 64;
            let u: f32 = std::f32::consts::PI * 2.0 / n as f32;

            let mut points: Vec<Point> = Vec::with_capacity(n);

            for i in 0..n {
                let a = (i as f32) * u;
                let point: Vec2 = (a.cos() * r + self.position.0, a.sin() * r + self.position.1);

                points.push(Point {
                    position: point,
                    color: self.spawning * 0.4 + 0.2,
                    color_palette: ColorPalette::Red,
                });
            }

            ctx.add_points(&points);
            return;
        }

        if self.shield > 0.0 {
            let sides = 7;
            let radius = self.get_description() * (2.5 + (self.shield * 5.0).cos() * 0.3);
            let mut triangles = vec![EMPTY_TRIANGLE; sides];

            let n = sides as f32;
            let u = 2.0 * std::f32::consts::PI / n;

            let color0 = ((self.shield * 5.0).cos() * 0.4 + 0.5) * (self.shield / 15.0) + 0.1;
            let color1 = color0 + 0.1;

            for i in 0..sides {
                let angle_left = (i as f32) * u + self.angle + self.shield;
                let angle_right = (i as f32) * u + u + self.angle + self.shield;

                let point_left: Vec2 = (
                    f32::cos(angle_left) * radius + self.position.0,
                    f32::sin(angle_left) * radius + self.position.1,
                );
                let point_right: Vec2 = (
                    f32::cos(angle_right) * radius + self.position.0,
                    f32::sin(angle_right) * radius + self.position.1,
                );

                triangles[i] = Triangle {
                    points: [self.position, point_left, point_right],
                    colors: [color1, color0, color0],
                    color_palette: ColorPalette::Blue,
                };
            }
            ctx.add_triangles(&triangles);
        }

        let front = (
            f32::cos(self.angle) * 1.5 + self.position.0,
            f32::sin(self.angle) * 1.5 + self.position.1,
        );
        let back = (
            f32::cos(self.angle + std::f32::consts::PI) + self.position.0,
            f32::sin(self.angle + std::f32::consts::PI) + self.position.1,
        );
        let left = (
            f32::cos(self.angle + std::f32::consts::PI + std::f32::consts::FRAC_PI_4) * 2.0
                + self.position.0,
            f32::sin(self.angle + std::f32::consts::PI + std::f32::consts::FRAC_PI_4) * 2.0
                + self.position.1,
        );
        let right = (
            f32::cos(self.angle + std::f32::consts::PI - std::f32::consts::FRAC_PI_4) * 2.0
                + self.position.0,
            f32::sin(self.angle + std::f32::consts::PI - std::f32::consts::FRAC_PI_4) * 2.0
                + self.position.1,
        );
        let triangles = [
            Triangle {
                points: [front, back, left],
                colors: [0.8, 1.0, 0.2],
                color_palette: ColorPalette::Gray,
            },
            Triangle {
                points: [right, back, front],
                colors: [0.7, 1.0, 0.7],
                color_palette: ColorPalette::Gray,
            },
        ]
        .to_vec();
        ctx.add_triangles(&triangles);
    }
}

impl Sprite for Ship {
    fn update(&mut self, camera: &Camera, delta: f32) {
        self.position.0 += self.speed.0 * delta;
        self.position.1 += self.speed.1 * delta;

        //screen bounds
        let bounds = camera.get_bounds();
        if self.position.0 < -bounds.0 {
            self.position.0 = bounds.0;
        }
        if self.position.0 > bounds.0 {
            self.position.0 = -bounds.0;
        }
        if self.position.1 < -bounds.1 {
            self.position.1 = bounds.1;
        }
        if self.position.1 > bounds.1 {
            self.position.1 = -bounds.1;
        }

        let damp = 0.96;
        self.speed.0 *= damp;
        self.speed.1 *= damp;

        self.angle += self.angle_speed * delta;
        self.angle_speed *= damp * damp * damp;

        //angle bounds
        if self.angle < 0.0 {
            self.angle += std::f32::consts::PI * 2.0;
        }
        if self.angle > std::f32::consts::PI * 2.0 {
            self.angle -= std::f32::consts::PI * 2.0;
        }

        self.fire_cooldown -= delta;
        self.spawning -= delta;
        self.splitfire -= delta;
        self.rapidfire -= delta;
        self.shield -= delta;
        self.piercing -= delta;
    }

    fn is_alive(&self) -> bool {
        self.life >= 0
    }
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: (0.0, 0.0),
            speed: (0.0, 0.0),
            angle: 0.0,
            fire_cooldown: 0.0,
            angle_speed: 0.0,
            turn_speed: 2.0,
            thrust_speed: 1.0,
            life: 5,
            spawning: 2.0,
            score: 0,
            shield: 3.0,
            splitfire: 0.0,
            rapidfire: 0.0,
            piercing: 0.0,
            firing: false,
        }
    }

    pub fn turn_left(&mut self) {
        if self.spawning > 0.0 {
            return;
        }

        self.angle_speed -= self.turn_speed;
    }

    pub fn turn_right(&mut self) {
        if self.spawning > 0.0 {
            return;
        }

        self.angle_speed += self.turn_speed;
    }

    pub fn thrust(&mut self) {
        if self.spawning > 0.0 {
            return;
        }

        self.speed.0 += self.angle.cos() * self.thrust_speed;
        self.speed.1 += self.angle.sin() * self.thrust_speed;
    }

    pub fn fire(&mut self, bullets: &mut Bullets) {
        if self.spawning > 0.0 {
            return;
        }
        self.firing = !self.firing;
    }

    pub fn update_switches(&mut self, bullets: &mut Bullets) {
        if self.spawning > 0.0 {
            self.firing = false;
        }

        if self.firing {
            self.spawn_bullet(bullets);
        }
    }

    fn spawn_bullet(&mut self, bullets: &mut Bullets) {
        if self.fire_cooldown <= 0.0 {
            self.fire_cooldown = match self.piercing > 0.0 {
                false => 0.4,
                true => 0.2,
            };
            if self.rapidfire > 0.0 {
                self.fire_cooldown = 0.1;
            }

            let n = match self.splitfire > 0.0 {
                false => 1,
                true => 7,
            };

            let u = std::f32::consts::PI * 2.0 / n as f32;

            for i in 0..n {
                let angle = (i as f32) * u + self.angle;
                bullets.bullets.push(Bullet::new(
                    self.position,
                    angle,
                    match self.piercing > 0.0 {
                        true => BulletType::Piercing,
                        false => BulletType::Normal,
                    },
                ));
            }
        }
    }

    pub fn powerup(&mut self, powerup: &Powerup) {
        match powerup.size {
            PowerupSize::Shield => self.shield = 10.0,
            PowerupSize::PiercingBullets => self.piercing = 5.0,
            PowerupSize::SplitFire => self.splitfire = 7.0,
            PowerupSize::RapidFire => self.rapidfire = 5.0,
        }
    }

    pub fn damage(&mut self, empty: Vec2) {
        self.position = empty;
        if self.spawning <= 0.0 {
            self.life -= 1;
        }
        self.spawning = 2.0;
        self.shield = 5.0;
    }

    pub fn get_description(&self) -> f32 {
        1.5
    }
}
