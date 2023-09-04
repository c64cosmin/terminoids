use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    color_palette: ColorPalette,
    life: f32,
    life_max: f32,
}

impl TerminalDrawble for Particle {
    fn draw(&self, ctx: &mut AsciiContext) {
        let point = Point {
            position: self.position,
            color: self.life / self.life_max / 2.0,
            color_palette: self.color_palette,
        };
        ctx.add_point(&point);
    }
}

impl Sprite for Particle {
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

        self.life -= delta;
    }

    fn is_alive(&self) -> bool {
        return self.life > 0.0;
    }
}

impl Spawnable for Particle {
    fn spawn(position: (f32, f32)) -> Particle {
        let mut rnd = rand::thread_rng();
        let angle = rnd.gen::<f32>() * std::f32::consts::PI * 2.0;
        let move_speed = rnd.gen::<f32>() * 10.0 + 5.0;
        let speed: Vec2 = (angle.cos() * move_speed, angle.sin() * move_speed);
        Particle {
            position,
            speed,
            color_palette: Particle::get_random_color(),
            life: 2.0,
            life_max: 2.0,
        }
    }
}

impl Particle {
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
}
