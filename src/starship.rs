use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum StarShipSize {
    Flying,
    SmallCluster,
    MediumCluster,
    BigCluster,
}

#[derive(Clone, Copy, Debug)]
pub struct StarShip {
    pub position: (f32, f32),
    pub speed: (f32, f32),
    pub angle: f32,
    pub size: StarShipSize,
    angle_speed: f32,
}

impl TerminalDrawble for StarShip {
    fn draw(&self, ctx: &mut AsciiContext) {
        match self.size {
            StarShipSize::Flying => {
                let (sides, radius) = self.get_description();
                let mut triangles = vec![EMPTY_TRIANGLE; sides];

                let small_radius = 1.0;

                let ai = self.angle;
                let al = ai - 2.5 * std::f32::consts::FRAC_PI_4;
                let ar = ai + 2.5 * std::f32::consts::FRAC_PI_4;

                let point_head: Vec2 = (
                    ai.cos() * radius * 1.2 + self.position.0,
                    ai.sin() * radius * 1.2 + self.position.1,
                );

                let point_left: Vec2 = (
                    al.cos() * radius * small_radius + self.position.0,
                    al.sin() * radius * small_radius + self.position.1,
                );
                let point_right: Vec2 = (
                    ar.cos() * radius * small_radius + self.position.0,
                    ar.sin() * radius * small_radius + self.position.1,
                );

                let color_a = ai.cos() * 0.4 + 0.5;
                let color_b = (ai + std::f32::consts::FRAC_PI_4).cos() * 0.3 + 0.5;

                triangles[0] = Triangle {
                    points: [self.position, point_left, point_head],
                    colors: [color_a, color_a, color_a],
                    color_palette: ColorPalette::Yellow,
                };
                triangles[1] = Triangle {
                    points: [self.position, point_right, point_head],
                    colors: [color_b, color_b, color_b],
                    color_palette: ColorPalette::Yellow,
                };

                ctx.add_triangles(&triangles);
            }
            StarShipSize::SmallCluster => {
                let (sides, radius) = self.get_description();
                let mut triangles = vec![EMPTY_TRIANGLE; sides];

                let n = 4.0;
                let u = 2.0 * std::f32::consts::PI / n;
                let sides = 4;
                let small_radius = 0.4;

                for i in 0..sides {
                    let ai = (i as f32) * u + self.angle;
                    let al = ai - std::f32::consts::FRAC_PI_4;
                    let ar = ai + std::f32::consts::FRAC_PI_4;

                    let point_head: Vec2 = (
                        ai.cos() * radius * 1.2 + self.position.0,
                        ai.sin() * radius * 1.2 + self.position.1,
                    );

                    let point_left: Vec2 = (
                        al.cos() * radius * small_radius + self.position.0,
                        al.sin() * radius * small_radius + self.position.1,
                    );
                    let point_right: Vec2 = (
                        ar.cos() * radius * small_radius + self.position.0,
                        ar.sin() * radius * small_radius + self.position.1,
                    );

                    let color_a = ai.cos() * 0.4 + 0.5;
                    let color_b = (ai + u).cos() * 0.3 + 0.5;

                    triangles[i * 2] = Triangle {
                        points: [self.position, point_left, point_head],
                        colors: [color_a, color_a, color_a],
                        color_palette: ColorPalette::Magenta,
                    };
                    triangles[i * 2 + 1] = Triangle {
                        points: [self.position, point_right, point_head],
                        colors: [color_b, color_b, color_b],
                        color_palette: ColorPalette::Magenta,
                    };
                }
                ctx.add_triangles(&triangles);
            }
            StarShipSize::MediumCluster => {
                let (sides, radius) = self.get_description();
                let mut triangles = vec![EMPTY_TRIANGLE; sides];

                let n = 4.0;
                let u = 2.0 * std::f32::consts::PI / n;
                let sides = 4;
                let small_radius = 0.6;

                for i in 0..sides {
                    let ai = (i as f32) * u + self.angle;
                    let al = ai - std::f32::consts::FRAC_PI_4;
                    let ar = ai + std::f32::consts::FRAC_PI_4;

                    let point_head: Vec2 = (
                        ai.cos() * radius * 1.2 + self.position.0,
                        ai.sin() * radius * 1.2 + self.position.1,
                    );

                    let point_left: Vec2 = (
                        al.cos() * radius * small_radius + self.position.0,
                        al.sin() * radius * small_radius + self.position.1,
                    );
                    let point_right: Vec2 = (
                        ar.cos() * radius * small_radius + self.position.0,
                        ar.sin() * radius * small_radius + self.position.1,
                    );

                    let color_a = ai.cos() * 0.4 + 0.5;
                    let color_b = (ai + u).cos() * 0.3 + 0.5;

                    triangles[i * 2] = Triangle {
                        points: [self.position, point_left, point_head],
                        colors: [color_a, color_a, color_a],
                        color_palette: ColorPalette::Magenta,
                    };
                    triangles[i * 2 + 1] = Triangle {
                        points: [self.position, point_right, point_head],
                        colors: [color_b, color_b, color_b],
                        color_palette: ColorPalette::Magenta,
                    };
                }

                let n = 4.0;
                let u = 2.0 * std::f32::consts::PI / n;
                let sides = 4;

                for i in 0..sides {
                    let ai = (i as f32) * u + self.angle + std::f32::consts::FRAC_PI_4;
                    let al = ai - std::f32::consts::FRAC_PI_4;
                    let ar = ai + std::f32::consts::FRAC_PI_4;

                    let point_head: Vec2 = (
                        ai.cos() * radius * 1.5 + self.position.0,
                        ai.sin() * radius * 1.5 + self.position.1,
                    );

                    let point_left: Vec2 = (
                        al.cos() * radius * small_radius + self.position.0,
                        al.sin() * radius * small_radius + self.position.1,
                    );
                    let point_right: Vec2 = (
                        ar.cos() * radius * small_radius + self.position.0,
                        ar.sin() * radius * small_radius + self.position.1,
                    );

                    let color_a = ai.cos() * 0.4 + 0.5;
                    let color_b = (ai + u).cos() * 0.3 + 0.5;

                    triangles[i * 2 + 8] = Triangle {
                        points: [self.position, point_left, point_head],
                        colors: [color_a, color_a, color_a],
                        color_palette: ColorPalette::Blue,
                    };
                    triangles[i * 2 + 9] = Triangle {
                        points: [self.position, point_right, point_head],
                        colors: [color_b, color_b, color_b],
                        color_palette: ColorPalette::Blue,
                    };
                }
                ctx.add_triangles(&triangles);
            }
            StarShipSize::BigCluster => {
                let (sides, radius) = self.get_description();
                let mut triangles = vec![EMPTY_TRIANGLE; sides];

                let r = radius;
                let a = self.angle;
                let point_a: Vec2 = (a.cos() * r + self.position.0, a.sin() * r + self.position.1);
                let a = self.angle + 2.0 * std::f32::consts::FRAC_PI_3;
                let point_b: Vec2 = (a.cos() * r + self.position.0, a.sin() * r + self.position.1);
                let a = self.angle + 4.0 * std::f32::consts::FRAC_PI_3;
                let point_c: Vec2 = (a.cos() * r + self.position.0, a.sin() * r + self.position.1);

                triangles[0] = Triangle {
                    points: [point_a, point_b, point_c],
                    colors: [0.3, 0.3, 0.3],
                    color_palette: ColorPalette::Red,
                };

                let a = self.angle + std::f32::consts::FRAC_PI_3;
                let point_a: Vec2 = (a.cos() * r + self.position.0, a.sin() * r + self.position.1);
                let a = self.angle + 3.0 * std::f32::consts::FRAC_PI_3;
                let point_b: Vec2 = (a.cos() * r + self.position.0, a.sin() * r + self.position.1);
                let a = self.angle + 5.0 * std::f32::consts::FRAC_PI_3;
                let point_c: Vec2 = (a.cos() * r + self.position.0, a.sin() * r + self.position.1);

                triangles[1] = Triangle {
                    points: [point_a, point_b, point_c],
                    colors: [0.5, 0.5, 0.5],
                    color_palette: ColorPalette::Red,
                };

                let n = 4.0;
                let u = 2.0 * std::f32::consts::PI / n;
                let sides = 4;
                let small_radius = 0.2;

                for i in 0..sides {
                    let ai = (i as f32) * u + self.angle;
                    let al = ai - std::f32::consts::FRAC_PI_4;
                    let ar = ai + std::f32::consts::FRAC_PI_4;

                    let point_head: Vec2 = (
                        ai.cos() * radius * 1.2 + self.position.0,
                        ai.sin() * radius * 1.2 + self.position.1,
                    );

                    let point_left: Vec2 = (
                        al.cos() * radius * small_radius + self.position.0,
                        al.sin() * radius * small_radius + self.position.1,
                    );
                    let point_right: Vec2 = (
                        ar.cos() * radius * small_radius + self.position.0,
                        ar.sin() * radius * small_radius + self.position.1,
                    );

                    let color_a = ai.cos() * 0.4 + 0.5;
                    let color_b = (ai + u).cos() * 0.3 + 0.5;

                    triangles[i * 2 + 2] = Triangle {
                        points: [self.position, point_left, point_head],
                        colors: [color_a, color_a, color_a],
                        color_palette: ColorPalette::Magenta,
                    };
                    triangles[i * 2 + 3] = Triangle {
                        points: [self.position, point_right, point_head],
                        colors: [color_b, color_b, color_b],
                        color_palette: ColorPalette::Magenta,
                    };
                }

                let n = 4.0;
                let u = 2.0 * std::f32::consts::PI / n;
                let sides = 4;

                for i in 0..sides {
                    let ai = (i as f32) * u + self.angle + std::f32::consts::FRAC_PI_4;
                    let al = ai - std::f32::consts::FRAC_PI_4;
                    let ar = ai + std::f32::consts::FRAC_PI_4;

                    let point_head: Vec2 = (
                        ai.cos() * radius * 1.5 + self.position.0,
                        ai.sin() * radius * 1.5 + self.position.1,
                    );

                    let point_left: Vec2 = (
                        al.cos() * radius * small_radius + self.position.0,
                        al.sin() * radius * small_radius + self.position.1,
                    );
                    let point_right: Vec2 = (
                        ar.cos() * radius * small_radius + self.position.0,
                        ar.sin() * radius * small_radius + self.position.1,
                    );

                    let color_a = ai.cos() * 0.4 + 0.5;
                    let color_b = (ai + u).cos() * 0.3 + 0.5;

                    triangles[i * 2 + 10] = Triangle {
                        points: [self.position, point_left, point_head],
                        colors: [color_a, color_a, color_a],
                        color_palette: ColorPalette::Blue,
                    };
                    triangles[i * 2 + 11] = Triangle {
                        points: [self.position, point_right, point_head],
                        colors: [color_b, color_b, color_b],
                        color_palette: ColorPalette::Blue,
                    };
                }
                ctx.add_triangles(&triangles);
            }
        };
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
            size: StarShipSize::Flying,
            angle_speed,
        }
    }
}

impl StarShip {
    fn get_description(&self) -> (usize, f32) {
        match self.size {
            StarShipSize::Flying => (2, 2.0),
            StarShipSize::SmallCluster => (8, 3.3),
            StarShipSize::MediumCluster => (16, 4.5),
            StarShipSize::BigCluster => (18, 5.2),
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
