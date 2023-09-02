use crate::drawables::Triangle;

pub trait DrawingContext {
    fn resize(&mut self, size: (u16, u16));
    fn draw_triangles(&self, triangles: &Vec<Triangle>);
    fn display(&self);
}
