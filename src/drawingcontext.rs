use crate::drawables::Triangle;

pub trait DrawingContext {
    fn resize(&mut self, size: (u16, u16));
    fn flush_triangles(&mut self);
    fn add_triangles(&mut self, triangles: &Vec<Triangle>);
    fn display(&self);
}
