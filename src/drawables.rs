pub type Point = (f32, f32);
pub type ColorLuma = f32;

#[derive(Clone, Debug)]
pub enum ColorPalette {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Gray,
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub points: [Point; 3],
    pub colors: [ColorLuma; 3],
    pub color_palette: ColorPalette,
}
