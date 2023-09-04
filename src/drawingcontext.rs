use crate::drawables::*;

pub trait DrawingContext {
    fn resize(&mut self, size: (u16, u16));
    fn clear(&mut self);
    fn flush_text_entries(&mut self);
    fn flush_triangles(&mut self);
    fn flush_points(&mut self);
    fn add_text_entry(&mut self, text_entry: &TextEntry);
    fn add_triangles(&mut self, triangles: &Vec<Triangle>);
    fn add_point(&mut self, point: &Point);
    fn add_points(&mut self, points: &Vec<Point>);
    fn display(&self);
}
