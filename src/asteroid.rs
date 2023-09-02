use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum AsteroidSize {
    Tiny,
    Small,
    Medium,
    Big,
    Huge,
}

#[derive(Clone, Debug)]
pub struct Asteroid {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
    pub size: AsteroidSize,
    angle_speed: f32,
    color_palette: ColorPalette,
}

impl TerminalDrawble for Asteroid {
    fn draw(&self, ctx: &mut AsciiContext) {
        let (sides, radius) = self.get_description();
        let mut triangles = vec![EMPTY_TRIANGLE; sides];

        let n = sides as f32;
        let u = 2.0 * std::f32::consts::PI / n;

        for i in 0..sides {
            let angle_left = (i as f32) * u + self.angle;
            let angle_right = (i as f32) * u + u + self.angle;

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
                colors: [0.55, 0.2, 0.3],
                color_palette: self.color_palette.clone(),
            };
        }
        ctx.add_triangles(&triangles);
    }
}

impl Sprite for Asteroid {
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

        self.angle += self.angle_speed * delta;

        //angle bounds
        if self.angle < 0.0 {
            self.angle += std::f32::consts::PI * 2.0;
        }
        if self.angle > std::f32::consts::PI * 2.0 {
            self.angle -= std::f32::consts::PI * 2.0;
        }
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}

impl Collidable for Asteroid {
    fn collide(&self, p: Vec2) -> bool {
        if distance(self.position, p) < self.get_description().1 {
            return true;
        }
        return false;
    }
}

impl Asteroid {
    pub fn new(position: (f32, f32)) -> Asteroid {
        let mut rnd = rand::thread_rng();
        let angle_speed: f32 = (rnd.gen::<f32>() * 0.2 + 0.2)
            * match rand::random() {
                true => -1.0,
                false => 1.0,
            };
        let angle = rnd.gen::<f32>() * std::f32::consts::PI * 2.0;
        let move_speed = rnd.gen::<f32>() * 0.5 + 0.1;
        let speed: Vec2 = (angle.cos() * move_speed, angle.sin() * move_speed);
        Asteroid {
            position,
            speed,
            angle: 0.0,
            size: AsteroidSize::Huge,
            angle_speed,
            color_palette: Asteroid::get_random_color(),
        }
    }

    fn get_random_color() -> ColorPalette {
        let mut rnd = rand::thread_rng();
        match rnd.gen_range(0..6) {
            0 => ColorPalette::Red,
            1 => ColorPalette::Green,
            2 => ColorPalette::Blue,
            3 => ColorPalette::Yellow,
            4 => ColorPalette::Magenta,
            5 => ColorPalette::Cyan,
            _ => ColorPalette::Gray,
        }
    }

    fn get_description(&self) -> (usize, f32) {
        match self.size {
            AsteroidSize::Tiny => (3, 1.9),
            AsteroidSize::Small => (4, 2.1),
            AsteroidSize::Medium => (5, 2.6),
            AsteroidSize::Big => (6, 3.3),
            AsteroidSize::Huge => (7, 4.0),
        }
    }

    pub fn split(&self) -> Vec<Asteroid> {
        let mut rnd = rand::thread_rng();
        let angle_speed: f32 = (rnd.gen::<f32>() * 0.2 + 0.2)
            * match rand::random() {
                true => -1.0,
                false => 1.0,
            };
        let angle = rnd.gen::<f32>() * std::f32::consts::PI * 2.0;
        let angle2 = angle + std::f32::consts::FRAC_PI_2;
        let move_speed = rnd.gen::<f32>() * 1.5 + 0.5;
        [
            Asteroid {
                position: self.position,
                speed: (
                    angle.cos() * move_speed + self.speed.0,
                    angle.sin() * move_speed + self.speed.1,
                ),
                angle,
                angle_speed,
                size: match self.size {
                    AsteroidSize::Small => AsteroidSize::Tiny,
                    AsteroidSize::Medium => AsteroidSize::Small,
                    AsteroidSize::Big => AsteroidSize::Medium,
                    AsteroidSize::Huge => AsteroidSize::Big,
                    _ => AsteroidSize::Tiny,
                },
                color_palette: Asteroid::get_random_color(),
            },
            Asteroid {
                position: self.position,
                speed: (
                    angle2.cos() * move_speed + self.speed.0,
                    angle2.sin() * move_speed + self.speed.1,
                ),
                angle,
                angle_speed,
                size: match self.size {
                    AsteroidSize::Small => AsteroidSize::Tiny,
                    AsteroidSize::Medium => AsteroidSize::Small,
                    AsteroidSize::Big => AsteroidSize::Medium,
                    AsteroidSize::Huge => AsteroidSize::Big,
                    _ => AsteroidSize::Tiny,
                },
                color_palette: Asteroid::get_random_color(),
            },
        ]
        .to_vec()
    }
}
