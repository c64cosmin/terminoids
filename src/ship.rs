use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;

pub struct Ship {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
}

impl TerminalDrawble for Ship {
    fn draw(&self, ctx: &mut AsciiContext) {
        let front = (
            f32::cos(self.angle) + self.position.0,
            f32::sin(self.angle) + self.position.1,
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
    fn update(&mut self) {
        self.position.0 += self.speed.0;
        self.position.1 += self.speed.1;
    }
}

impl Ship {
    fn thrust(&mut self, speed: f32, angle: f32) {
        self.speed.0 += angle.cos() * speed;
        self.speed.1 += angle.sin() * speed;
    }
}
