use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

pub enum AsteroidSize {
    Tiny,
    Small,
    Medium,
    Big,
    Huge,
}

pub struct Asteroid {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
    pub size: AsteroidSize,
    angle_speed: f32,
}

impl TerminalDrawble for Asteroid {
    fn draw(&self, ctx: &mut AsciiContext) {
        let (sides, radius) = match self.size {
            AsteroidSize::Tiny => (3, 1.9),
            AsteroidSize::Small => (4, 2.1),
            AsteroidSize::Medium => (5, 2.6),
            AsteroidSize::Big => (6, 3.3),
            AsteroidSize::Huge => (7, 4.0),
        };

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
                color_palette: ColorPalette::Cyan,
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

impl Asteroid {
    pub fn new(position: (f32, f32)) -> Asteroid {
        let mut rnd = rand::thread_rng();
        let angle_speed: f32 = (rnd.gen::<f32>() * 0.2 + 0.2)
            * match rand::random() {
                true => -1.0,
                false => 1.0,
            };
        Asteroid {
            position,
            speed: (0.0, 0.0),
            angle: 0.0,
            size: AsteroidSize::Small,
            angle_speed,
        }
    }
}
