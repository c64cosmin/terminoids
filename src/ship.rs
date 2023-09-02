use crate::asciicontext::AsciiContext;
use crate::sprite::*;
use crate::terminaldrawable::*;

pub struct Ship {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
}

impl TerminalDrawble for Ship {
    fn draw(&self, ctx: AsciiContext) {}
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
