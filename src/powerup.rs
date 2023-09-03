use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::enemy::*;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum PowerupSize {
    RapidFire,
    PiercingBullets,
    Shield,
}

#[derive(Clone, Copy, Debug)]
pub struct Powerup {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub size: PowerupSize,
}

impl TerminalDrawble for Powerup {
    fn draw(&self, ctx: &mut AsciiContext) {
        let radius = self.get_description();
        /*
        let mut triangles = vec![EMPTY_TRIANGLE; sides];

        let n = sides as f32;
        let u = 2.0 * std::f32::consts::PI / n;

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
                colors: [0.55, 0.2, 0.3],
                color_palette: ColorPalette::Red,
            };
        }
        ctx.add_triangles(&triangles);
        */
    }
}

impl Sprite for Powerup {
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
    }

    fn is_alive(&self) -> bool {
        return true;
    }
}

impl Collidable for Powerup {
    fn collide(&self, p: Vec2) -> bool {
        if distance(self.position, p) < self.get_description() {
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
        let angle_speed: f32 = (rnd.gen::<f32>() * 0.2 + 0.2)
            * match rand::random() {
                true => -1.0,
                false => 1.0,
            };
        let angle = rnd.gen::<f32>() * std::f32::consts::PI * 2.0;
        let move_speed = rnd.gen::<f32>() * 0.5 + 0.1;
        let speed: Vec2 = (angle.cos() * move_speed, angle.sin() * move_speed);
        Powerup {
            position,
            speed,
            size: PowerupSize::RapidFire,
        }
    }
}

impl Powerup {
    fn get_description(&self) -> f32 {
        1.0
    }
}
