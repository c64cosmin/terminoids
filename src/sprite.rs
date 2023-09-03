use crate::drawables::Camera;
use crate::drawables::Vec2;
use crate::enemy::EnemyType;
use crate::ship::Ship;

pub trait Sprite {
    fn update(&mut self, camera: &Camera, delta: f32);
    fn is_alive(&self) -> bool;
}

pub trait Collidable {
    fn collide(&self, p: Vec2) -> bool;
    fn collide_with_ship(&self, ship: &Ship) -> bool;
    fn split(&self) -> Vec<EnemyType>;
}

pub trait Spawnable {
    fn spawn(position: Vec2) -> Self;
}
