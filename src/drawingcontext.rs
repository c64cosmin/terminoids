use crate::drawables::*;

pub trait DrawingContext {
    fn resize(&mut self, size: (u16, u16));
    fn clear(&mut self);
    fn flush_triangles(&mut self);
    fn flush_points(&mut self);
    fn add_triangles(&mut self, triangles: &Vec<Triangle>);
    fn add_points(&mut self, points: &Vec<Point>);
    fn display(&self);
}
