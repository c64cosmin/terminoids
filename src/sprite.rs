use crate::drawables::Camera;
use crate::drawables::Vec2;

pub trait Sprite {
    fn update(&mut self, camera: &Camera, delta: f32);
    fn is_alive(&self) -> bool;
}

pub trait Collidable {
    fn collide(&self, p: Vec2) -> bool;
}

pub trait Spawnable {
    fn spawn(position: Vec2) -> Self;
}
