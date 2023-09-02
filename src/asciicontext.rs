use crate::drawables::Triangle;
use crate::drawingcontext::DrawingContext;
use termion::color;

pub struct AsciiContext {
    bitmap: Vec<u8>,
    size: (u16, u16),
}

/*
const DEFAULT_COLOR: (&str, &str) = (color::Black.bg_str(), color::White.fg_str());
const RED_PALETTE: [(&str, &str); 5] = [
    (color::Black.bg_str(), color::Red.fg_str()),
    (color::Red.bg_str(), color::LightRed.fg_str()),
    (color::LightRed.bg_str(), color::Yellow.fg_str()),
    (color::Yellow.bg_str(), color::LightYellow.fg_str()),
    (color::LightYellow.bg_str(), color::LightWhite.fg_str()),
];
const GREEN_PALETTE: [(&str, &str); 3] = [
    (color::Black.bg_str(), color::Green.fg_str()),
    (color::Green.bg_str(), color::LightGreen.fg_str()),
    (color::LightGreen.bg_str(), color::LightYellow.fg_str()),
];
const BLUE_PALETTE: [(&str, &str); 4] = [
    (color::Black.bg_str(), color::Blue.fg_str()),
    (color::Blue.bg_str(), color::LightBlue.fg_str()),
    (color::LightBlue.bg_str(), color::LightCyan.fg_str()),
    (color::LightCyan.bg_str(), color::LightWhite.fg_str()),
];
const YELLOW_PALETTE: [(&str, &str); 5] = [
    (color::Black.bg_str(), color::Yellow.fg_str()),
    (color::Yellow.bg_str(), color::LightYellow.fg_str()),
    (color::LightYellow.bg_str(), color::LightWhite.fg_str()),
];
const MAGENTA_PALETTE: [(&str, &str); 2] = [
    (color::Black.bg_str(), color::Magenta.fg_str()),
    (color::Magenta.bg_str(), color::LightMagenta.fg_str()),
];
const GRAY_PALETTE: [(&str, &str); 3] = [
    (color::Black.bg_str(), color::LightBlack.fg_str()),
    (color::LightBlack.bg_str(), color::White.fg_str()),
    (color::White.bg_str(), color::LightWhite.fg_str()),
];
*/
const CHARS_GRADIENT: [char; 5] = [' ', '\u{2591}', '\u{2592}', '\u{2593}', '\u{2593}'];
//const CHARS_GRADIENT: [char; 5] = ['.', 'x', '%', '#', '@'];
const DEFAULT_COLOR: (&str, &str) = ("\u{1b}[48;5;0m", "\u{1b}[38;5;7m");
const PALETTE_RANGE: u8 = 16;

const RED_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;1m"),
    ("\u{1b}[48;5;1m", "\u{1b}[38;5;9m"),
    ("\u{1b}[48;5;9m", "\u{1b}[38;5;11m"),
];

const GREEN_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;2m"),
    ("\u{1b}[48;5;2m", "\u{1b}[38;5;10m"),
    ("\u{1b}[48;5;10m", "\u{1b}[38;5;11m"),
];

const BLUE_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;4m"),
    ("\u{1b}[48;5;4m", "\u{1b}[38;5;12m"),
    ("\u{1b}[48;5;12m", "\u{1b}[38;5;14m"),
];

const YELLOW_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;8m", "\u{1b}[38;5;3m"),
    ("\u{1b}[48;5;3m", "\u{1b}[38;5;11m"),
    ("\u{1b}[48;5;11m", "\u{1b}[38;5;15m"),
];

const MAGENTA_PALETTE: [(&str, &str); 2] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;5m"),
    ("\u{1b}[48;5;5m", "\u{1b}[38;5;13m"),
];

const CYAN_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;6m"),
    ("\u{1b}[48;5;6m", "\u{1b}[38;5;14m"),
    ("\u{1b}[48;5;14m", "\u{1b}[38;5;15m"),
];

const GRAY_PALETTE: [(&str, &str); 3] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;8m"),
    ("\u{1b}[48;5;8m", "\u{1b}[38;5;7m"),
    ("\u{1b}[48;5;7m", "\u{1b}[38;5;15m"),
];

impl AsciiContext {
    pub fn new(size: (u16, u16)) -> AsciiContext {
        let bitmap: Vec<u8> = vec![0; (size.0 * size.1) as usize];

        AsciiContext { bitmap, size }
    }

    pub fn set(&mut self, pos: (u16, u16), v: u8) {
        let i = pos.1 * self.size.0 + pos.0;
        self.bitmap[i as usize] = v;
    }

    fn get_indexes(&self, luma: u8, color_len: usize) -> (usize, usize) {
        let char_len: u8 = CHARS_GRADIENT.len() as u8;
        let color_len: u8 = color_len as u8;
        let v_col = luma * color_len / PALETTE_RANGE;
        let v_char = ((luma * color_len) % PALETTE_RANGE) * char_len / PALETTE_RANGE;

        (v_col as usize, v_char as usize)
    }

    fn red_palette(&self, luma: u8) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, RED_PALETTE.len());
        (RED_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn green_palette(&self, luma: u8) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, GREEN_PALETTE.len());
        (GREEN_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn blue_palette(&self, luma: u8) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, BLUE_PALETTE.len());
        (BLUE_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn yellow_palette(&self, luma: u8) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, YELLOW_PALETTE.len());
        (YELLOW_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn magenta_palette(&self, luma: u8) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, MAGENTA_PALETTE.len());
        (MAGENTA_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn cyan_palette(&self, luma: u8) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, CYAN_PALETTE.len());
        (CYAN_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn gray_palette(&self, luma: u8) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, GRAY_PALETTE.len());
        (GRAY_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn fill_color(&self, color: u8) {
        let ((fg, bg), chr) = match color {
            0..=15 => self.red_palette(color),
            16..=31 => self.green_palette(color - 16),
            32..=47 => self.blue_palette(color - 32),
            48..=63 => self.yellow_palette(color - 48),
            64..=79 => self.magenta_palette(color - 64),
            80..=95 => self.cyan_palette(color - 80),
            96..=111 => self.gray_palette(color - 96),
            _ => (DEFAULT_COLOR, ' '),
        };

        print!("{}{}{}", bg, fg, chr);
    }
}

impl DrawingContext for AsciiContext {
    fn resize(&mut self, size: (u16, u16)) {
        self.bitmap = vec![0; (size.0 * size.1) as usize];
    }

    fn draw_triangles(&self, triangles: &Vec<Triangle>) {}

    fn display(&self) {
        for (i, line) in self.bitmap.chunks(self.size.0 as usize).enumerate() {
            if i != 0 {
                print!("\n");
            }

            let mut was_colored = false;

            for &pixel in line.iter() {
                match pixel {
                    0 => {
                        if was_colored {
                            print!("{}{}", color::Bg(color::Black), color::Fg(color::White));
                            was_colored = false;
                        }
                        print!(" ");
                    }
                    _ => {
                        self.fill_color(pixel - 1);
                        was_colored = true;
                    }
                }
            }

            print!("\r");
        }
    }
}
