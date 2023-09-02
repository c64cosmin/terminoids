pub type Point = (f32, f32);
pub type ColorLuma = f32;

enum ColorPalette {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Gray,
}

pub struct Triangle {
    points: [Point; 3],
    colors: [ColorLuma; 3],
    color: ColorPalette,
}
