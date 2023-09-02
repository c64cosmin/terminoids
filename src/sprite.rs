use crate::drawables::Camera;

pub trait Sprite {
    fn update(&mut self, camera: &Camera, delta: f32);
    fn is_alive(&self) -> bool;
}
