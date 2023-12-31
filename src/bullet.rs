use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;

#[derive(Clone, Debug)]
pub enum BulletType {
    Normal,
    Piercing,
}

pub struct Bullet {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub bullet_type: BulletType,
    pub life: f32,
}

impl Bullet {
    pub fn new(position: (f32, f32), angle: f32, bullet_type: BulletType) -> Bullet {
        let linear_speed = match bullet_type {
            BulletType::Normal => 15.0,
            BulletType::Piercing => 20.0,
        };
        let speed = (angle.cos() * linear_speed, angle.sin() * linear_speed);
        Bullet {
            position,
            speed,
            bullet_type: bullet_type.clone(),
            life: match bullet_type {
                BulletType::Normal => 2.0,
                BulletType::Piercing => 3.0,
            },
        }
    }

    pub fn get_drawable_point(&self) -> Point {
        Point {
            position: self.position,
            color: match self.bullet_type {
                BulletType::Normal => 128.0,
                BulletType::Piercing => 129.0 + (self.life * 30.0).rem_euclid(2.0).floor(),
            },
            color_palette: match self.bullet_type {
                BulletType::Normal => ColorPalette::Custom,
                BulletType::Piercing => ColorPalette::Custom,
            },
        }
    }

    pub fn destroy(&mut self) {
        match self.bullet_type {
            BulletType::Normal => self.life = 0.0,
            _ => self.life -= 0.4,
        }
    }
}

impl Sprite for Bullet {
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
        self.life > 0.0
    }
}

impl TerminalDrawble for Bullet {
    fn draw(&self, ctx: &mut AsciiContext) {
        ctx.add_point(&self.get_drawable_point());
    }
}

pub struct Bullets {
    pub bullets: Vec<Bullet>,
}

impl Bullets {
    pub fn new() -> Bullets {
        Bullets {
            bullets: Vec::with_capacity(100),
        }
    }
}

impl Sprite for Bullets {
    fn update(&mut self, camera: &Camera, delta: f32) {
        self.bullets
            .iter_mut()
            .for_each(|bullet| bullet.update(camera, delta));
        self.bullets.retain(|bullet| bullet.is_alive());
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}

impl TerminalDrawble for Bullets {
    fn draw(&self, ctx: &mut AsciiContext) {
        let points = self
            .bullets
            .iter()
            .map(|p| p.get_drawable_point())
            .collect();
        ctx.add_points(&points);
    }
}
