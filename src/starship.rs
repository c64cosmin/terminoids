use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum StarShipSize {
    Flying,
    SmallCluster,
    MediumCluster,
    BigCluster,
}

#[derive(Clone, Debug)]
pub struct StarShip {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
    pub size: StarShipSize,
    angle_speed: f32,
}

impl TerminalDrawble for StarShip {
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
                color_palette: ColorPalette::Red,
            };
        }
        ctx.add_triangles(&triangles);
    }
}

impl Sprite for StarShip {
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

impl Collidable for StarShip {
    fn collide(&self, p: Vec2) -> bool {
        if distance(self.position, p) < self.get_description().1 {
            return true;
        }
        return false;
    }
}

impl Spawnable for StarShip {
    fn spawn(position: (f32, f32)) -> StarShip {
        let mut rnd = rand::thread_rng();
        let angle_speed: f32 = (rnd.gen::<f32>() * 0.2 + 0.2)
            * match rand::random() {
                true => -1.0,
                false => 1.0,
            };
        let angle = rnd.gen::<f32>() * std::f32::consts::PI * 2.0;
        let move_speed = rnd.gen::<f32>() * 0.5 + 0.1;
        let speed: Vec2 = (angle.cos() * move_speed, angle.sin() * move_speed);
        StarShip {
            position,
            speed,
            angle: 0.0,
            size: StarShipSize::BigCluster,
            angle_speed,
        }
    }
}

impl StarShip {
    fn get_description(&self) -> (usize, f32) {
        match self.size {
            StarShipSize::Flying => (3, 1.9),
            StarShipSize::SmallCluster => (4, 2.1),
            StarShipSize::MediumCluster => (5, 2.6),
            StarShipSize::BigCluster => (6, 3.3),
        }
    }

    pub fn split(&self) -> Vec<StarShip> {
        let mut rnd = rand::thread_rng();
        let angle_speed: f32 = (rnd.gen::<f32>() * 0.2 + 0.2)
            * match rand::random() {
                true => -1.0,
                false => 1.0,
            };
        let angle = rnd.gen::<f32>() * std::f32::consts::PI * 2.0;
        let angle2 = angle + std::f32::consts::PI;
        let move_speed = rnd.gen::<f32>() * 2.0 + 0.8;
        [
            StarShip {
                position: self.position,
                speed: (
                    angle.cos() * move_speed + self.speed.0 * 0.5,
                    angle.sin() * move_speed + self.speed.1 * 0.5,
                ),
                angle,
                angle_speed,
                size: match self.size {
                    StarShipSize::BigCluster => StarShipSize::MediumCluster,
                    StarShipSize::MediumCluster => StarShipSize::SmallCluster,
                    _ => StarShipSize::Flying,
                },
            },
            StarShip {
                position: self.position,
                speed: (
                    angle2.cos() * move_speed + self.speed.0 * 0.5,
                    angle2.sin() * move_speed + self.speed.1 * 0.5,
                ),
                angle,
                angle_speed,
                size: match self.size {
                    StarShipSize::BigCluster => StarShipSize::MediumCluster,
                    StarShipSize::MediumCluster => StarShipSize::SmallCluster,
                    _ => StarShipSize::Flying,
                },
            },
        ]
        .to_vec()
    }
}
