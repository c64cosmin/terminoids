use crate::asciicontext::AsciiContext;
use crate::bullet::*;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;

pub struct Ship {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
    pub fire_cooldown: f32,
    thrust_speed: f32,
    angle_speed: f32,
    turn_speed: f32,
}

impl TerminalDrawble for Ship {
    fn draw(&self, ctx: &mut AsciiContext) {
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
    }

    fn is_alive(&self) -> bool {
        return true;
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
        }
    }
    pub fn thrust(&mut self) {
        self.speed.0 += self.angle.cos() * self.turn_speed;
        self.speed.1 += self.angle.sin() * self.turn_speed;
    }

    pub fn fire(&mut self, bullets: &mut Bullets) {
        if self.fire_cooldown <= 0.0 {
            self.fire_cooldown = 0.3;
            bullets
                .bullets
                .push(Bullet::new(self.position, self.angle, BulletType::Normal));
        }
    }

    pub fn turn(&mut self, angle: f32) {
        self.angle_speed += angle;
    }

    pub fn turn_left(&mut self) {
        self.turn(-self.turn_speed);
    }

    pub fn turn_right(&mut self) {
        self.turn(self.turn_speed);
    }
}
