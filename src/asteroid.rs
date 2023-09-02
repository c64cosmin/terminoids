use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;

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

            let point_left: Point = (
                f32::cos(angle_left) * radius + self.position.0,
                f32::sin(angle_left) * radius + self.position.1,
            );
            let point_right: Point = (
                f32::cos(angle_right) * radius + self.position.0,
                f32::sin(angle_right) * radius + self.position.1,
            );

            triangles[i] = Triangle {
                points: [self.position, point_left, point_right],
                colors: [0.5, 0.1, 0.2],
                color_palette: ColorPalette::Cyan,
            };
        }
        ctx.add_triangles(&triangles);
    }
}

impl Sprite for Asteroid {
    fn update(&mut self) {
        //self.position.0 += self.speed.0;
        //self.position.1 += self.speed.1;
        self.angle += 0.05;
    }
}

impl Asteroid {
    fn thrust(&mut self, speed: f32, angle: f32) {
        self.speed.0 += angle.cos() * speed;
        self.speed.1 += angle.sin() * speed;
    }
}
