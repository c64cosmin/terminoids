/*
pub const DEFAULT_COLOR: (&str, &str) = (color::Black.bg_str(), color::White.fg_str());
pub const RED_PALETTE: [(&str, &str); 5] = [
    (color::Black.bg_str(), color::Red.fg_str()),
    (color::Red.bg_str(), color::LightRed.fg_str()),
    (color::LightRed.bg_str(), color::Yellow.fg_str()),
    (color::Yellow.bg_str(), color::LightYellow.fg_str()),
    (color::LightYellow.bg_str(), color::LightWhite.fg_str()),
];
pub const GREEN_PALETTE: [(&str, &str); 3] = [
    (color::Black.bg_str(), color::Green.fg_str()),
    (color::Green.bg_str(), color::LightGreen.fg_str()),
    (color::LightGreen.bg_str(), color::LightYellow.fg_str()),
];
pub const BLUE_PALETTE: [(&str, &str); 4] = [
    (color::Black.bg_str(), color::Blue.fg_str()),
    (color::Blue.bg_str(), color::LightBlue.fg_str()),
    (color::LightBlue.bg_str(), color::LightCyan.fg_str()),
    (color::LightCyan.bg_str(), color::LightWhite.fg_str()),
];
pub const YELLOW_PALETTE: [(&str, &str); 5] = [
    (color::Black.bg_str(), color::Yellow.fg_str()),
    (color::Yellow.bg_str(), color::LightYellow.fg_str()),
    (color::LightYellow.bg_str(), color::LightWhite.fg_str()),
];
pub const MAGENTA_PALETTE: [(&str, &str); 2] = [
    (color::Black.bg_str(), color::Magenta.fg_str()),
    (color::Magenta.bg_str(), color::LightMagenta.fg_str()),
];
pub const GRAY_PALETTE: [(&str, &str); 3] = [
    (color::Black.bg_str(), color::LightBlack.fg_str()),
    (color::LightBlack.bg_str(), color::White.fg_str()),
    (color::White.bg_str(), color::LightWhite.fg_str()),
];
*/
pub const CHAR_BALL: char = '\u{25CF}';
pub const CHARS_GRADIENT: [char; 5] = [' ', '\u{2591}', '\u{2592}', '\u{2593}', '\u{2593}'];
//pub const CHARS_GRADIENT: [char; 5] = ['.', 'x', '%', '#', '@'];
pub const DEFAULT_COLOR: (&str, &str) = ("\u{1b}[48;5;0m", "\u{1b}[38;5;7m");
pub const PALETTE_RANGE: u8 = 16;

pub const RED_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;1m"),
    ("\u{1b}[48;5;1m", "\u{1b}[38;5;9m"),
    ("\u{1b}[48;5;9m", "\u{1b}[38;5;11m"),
];

pub const GREEN_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;2m"),
    ("\u{1b}[48;5;2m", "\u{1b}[38;5;10m"),
    ("\u{1b}[48;5;10m", "\u{1b}[38;5;11m"),
];

pub const BLUE_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;4m"),
    ("\u{1b}[48;5;4m", "\u{1b}[38;5;12m"),
    ("\u{1b}[48;5;12m", "\u{1b}[38;5;14m"),
];

pub const YELLOW_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;8m", "\u{1b}[38;5;3m"),
    ("\u{1b}[48;5;3m", "\u{1b}[38;5;11m"),
    ("\u{1b}[48;5;11m", "\u{1b}[38;5;15m"),
];

pub const MAGENTA_PALETTE: [(&str, &str); 2] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;5m"),
    ("\u{1b}[48;5;5m", "\u{1b}[38;5;13m"),
];

pub const CYAN_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;6m"),
    ("\u{1b}[48;5;6m", "\u{1b}[38;5;14m"),
    ("\u{1b}[48;5;14m", "\u{1b}[38;5;15m"),
];

pub const GRAY_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;8m"),
    ("\u{1b}[48;5;8m", "\u{1b}[38;5;7m"),
    ("\u{1b}[48;5;7m", "\u{1b}[38;5;15m"),
];

pub type Vec2 = (f32, f32);
pub type ColorLuma = f32;

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    f32::sqrt((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2))
}

pub fn length(a: Vec2) -> f32 {
    f32::sqrt(a.0 * a.0 + a.1 * a.1)
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vec2,
    pub size: Vec2,
    pub zoom: f32,
}

#[derive(Clone, Debug)]
pub enum ColorPalette {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Gray,
    Custom,
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub points: [Vec2; 3],
    pub colors: [ColorLuma; 3],
    pub color_palette: ColorPalette,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub position: Vec2,
    pub color: ColorLuma,
    pub color_palette: ColorPalette,
}

pub const EMPTY_POINT: Point = Point {
    position: (0.0, 0.0),
    color: 0.0,
    color_palette: ColorPalette::Red,
};

pub const EMPTY_TRIANGLE: Triangle = Triangle {
    points: [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
    colors: [0.0, 0.0, 0.0],
    color_palette: ColorPalette::Red,
};

impl Camera {
    pub fn get_bounds(&self) -> (f32, f32) {
        let aspect_ratio: (f32, f32) = (1.0, 1.0); //camera.size.1 / camera.size.0, 1.0);
        let char_ratio: (f32, f32) = (17.0 / 8.0, 1.0);
        let default_height: f32 = self.size.1 as f32 / 63.0;
        (
            self.size.0 / default_height / self.zoom / char_ratio.0 / 2.0,
            self.size.1 / default_height / self.zoom / char_ratio.1 / 2.0,
        )
    }
}
