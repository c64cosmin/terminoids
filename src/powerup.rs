use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::enemy::*;
use crate::ship::*;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum PowerupSize {
    SplitFire,
    PiercingBullets,
    Shield,
    RapidFire,
}

#[derive(Clone, Copy, Debug)]
pub struct Powerup {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub size: PowerupSize,
    pub life: f32,
}

impl TerminalDrawble for Powerup {
    fn draw(&self, ctx: &mut AsciiContext) {
        let sides = 6;
        let radius = self.get_description() * (0.8 + self.life.cos() * 0.3);
        let mut triangles = vec![EMPTY_TRIANGLE; sides];

        let n = sides as f32;
        let u = 2.0 * std::f32::consts::PI / n;

        let color0 = self.life.cos() * 0.4 + 0.5;
        let color1 = color0 + 0.1;

        for i in 0..sides {
            let angle_left = (i as f32) * u;
            let angle_right = (i as f32) * u + u;

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
                color_palette: match self.size {
                    PowerupSize::Shield => ColorPalette::Blue,
                    PowerupSize::SplitFire => ColorPalette::Green,
                    PowerupSize::PiercingBullets => ColorPalette::Red,
                    PowerupSize::RapidFire => ColorPalette::Gray,
                },
            };
        }
        ctx.add_triangles(&triangles);
    }
}

impl Sprite for Powerup {
    fn update(&mut self, camera: &Camera, delta: f32) {
        self.life += delta * 10.0;

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
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}

impl Collidable for Powerup {
    fn get_position(&self) -> Vec2 {
        self.position.clone()
    }

    fn collide(&self, p: Vec2) -> bool {
        if distance(self.position, p) < self.get_description() {
            return true;
        }
        return false;
    }

    fn collide_with_ship(&self, ship: &Ship) -> bool {
        if distance(self.position, ship.position)
            < self.get_description() + ship.get_description() * 2.0
        {
            return true;
        }
        return false;
    }

    fn split(&self) -> Vec<EnemyType> {
        Vec::new()
    }
}

impl Spawnable for Powerup {
    fn spawn(position: (f32, f32)) -> Powerup {
        let mut rnd = rand::thread_rng();
        let angle = rnd.gen::<f32>() * std::f32::consts::PI * 2.0;
        let move_speed = rnd.gen::<f32>() * 1.5 + 0.5;
        let speed: Vec2 = (angle.cos() * move_speed, angle.sin() * move_speed);
        let type_number = rnd.gen_range(0..15);
        Powerup {
            position,
            speed,
            size: match type_number {
                0..=1 => PowerupSize::PiercingBullets,
                2..=4 => PowerupSize::RapidFire,
                5..=8 => PowerupSize::SplitFire,
                _ => PowerupSize::Shield,
            },
            life: 0.0,
        }
    }
}

impl Powerup {
    fn get_description(&self) -> f32 {
        2.0
    }
}
