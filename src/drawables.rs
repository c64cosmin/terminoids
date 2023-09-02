pub type Point = (f32, f32);
pub type ColorLuma = f32;

#[derive(Clone)]
pub enum ColorPalette {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Gray,
}

#[derive(Clone)]
pub struct Triangle {
    pub points: [Point; 3],
    pub color_palette: ColorPalette,
    pub color_intensity: ColorLuma,
}
