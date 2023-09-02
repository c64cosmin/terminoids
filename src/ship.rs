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
        let triangles = [
            Triangle {
                points: [(0.0, 0.0), (0.2, 0.1), (0.06, 0.2)],
                colors: [0.0, 1.0, 0.5],
                color_palette: ColorPalette::Blue,
            },
            Triangle {
                points: [(0.01, 0.01), (0.3, 0.3), (0.01, 0.3)],
                colors: [0.0, 0.5, 1.0],
                color_palette: ColorPalette::Magenta,
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
