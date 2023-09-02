pub struct Ship {
    position: (f32, f32),
    speed: (f32, f32),
    angle: f32,
}

impl TerminalDrawble for Ship {
    fn draw(&self) {}
}

impl Sprite for Ship {
    fn update(&mut self) {
        position.0 += speed.0;
        position.1 += speed.1;
    }
}

impl Ship {
    fn thrust(&mut self, speed: f32, angle: f32) {
        self.speed.0 += angle.cos() * speed;
        self.speed.1 += angle.sin() * speed;
    }
}
