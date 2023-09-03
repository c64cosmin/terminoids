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
    fire_cooldown: f32,
    thrust_speed: f32,
    angle_speed: f32,
    turn_speed: f32,
    spawning: f32,
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
        self.angle_speed *= damp;

        //angle bounds
        if self.angle < 0.0 {
            self.angle += std::f32::consts::PI * 2.0;
        }
        if self.angle > std::f32::consts::PI * 2.0 {
            self.angle -= std::f32::consts::PI * 2.0;
        }

        self.fire_cooldown -= delta;
        self.spawning -= delta;
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
            thrust_speed: 1.5,
            life: 5,
            spawning: 2.0,
        }
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
        if self.fire_cooldown <= 0.0 {
            self.fire_cooldown = 0.3;
            bullets
                .bullets
                .push(Bullet::new(self.position, self.angle, BulletType::Normal));
        }
    }

    pub fn turn(&mut self, angle: f32) {
        if self.spawning > 0.0 {
            return;
        }
        self.angle_speed += angle;
    }

    pub fn turn_left(&mut self) {
        self.turn(-self.turn_speed);
    }

    pub fn turn_right(&mut self) {
        self.turn(self.turn_speed);
    }

    pub fn powerup(&mut self, powerup: &Powerup) {}

    pub fn damage(&mut self, empty: Vec2) {
        if self.spawning > 0.0 {
            return;
        }
        self.position = empty;
        self.life -= 1;
        self.spawning = 2.0;
    }

    pub fn get_description(&self) -> f32 {
        1.5
    }
}
