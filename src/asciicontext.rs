use crate::drawables::Triangle;
use crate::drawingcontext::DrawingContext;
use termion::color;

pub struct AsciiContext {
    bitmap: Vec<u8>,
    size: (u16, u16),
}

/*
const DEFAULT_COLOR: (&str, &str) = (color::Black.bg_str(), color::White.fg_str());
const BLUE_PALETTE: [(&str, &str); 4] = [
    (color::Black.bg_str(), color::Blue.fg_str()),
    (color::Blue.bg_str(), color::LightBlue.fg_str()),
    (color::LightBlue.bg_str(), color::LightCyan.fg_str()),
    (color::LightCyan.bg_str(), color::LightWhite.fg_str()),
];
const RED_PALETTE: [(&str, &str); 5] = [
    (color::Black.bg_str(), color::Red.fg_str()),
    (color::Red.bg_str(), color::LightRed.fg_str()),
    (color::LightRed.bg_str(), color::Yellow.fg_str()),
    (color::Yellow.bg_str(), color::LightYellow.fg_str()),
    (color::LightYellow.bg_str(), color::LightWhite.fg_str()),
];
const GRAY_PALETTE: [(&str, &str); 3] = [
    (color::Black.bg_str(), color::LightBlack.fg_str()),
    (color::LightBlack.bg_str(), color::White.fg_str()),
    (color::White.bg_str(), color::LightWhite.fg_str()),
];
*/
//const CHARS_GRADIENT: [char; 4] = ['\u{2591}', '\u{2592}', '\u{2593}', '\u{2593}'];
const CHARS_GRADIENT: [char; 4] = ['.', 'x', '%', '#'];
const DEFAULT_COLOR: (&str, &str) = ("\u{1b}[48;5;0m", "\u{1b}[38;5;7m");
const PALETTE_RANGE: u8 = 32;
const BLUE_PALETTE: [(&str, &str); 4] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;4m"),
    ("\u{1b}[48;5;4m", "\u{1b}[38;5;12m"),
    ("\u{1b}[48;5;12m", "\u{1b}[38;5;14m"),
    ("\u{1b}[48;5;14m", "\u{1b}[38;5;15m"),
];

const RED_PALETTE: [(&str, &str); 4] = [
    ("\u{1b}[48;5;0m", "\u{1b}[38;5;1m"),
    ("\u{1b}[48;5;1m", "\u{1b}[38;5;9m"),
    ("\u{1b}[48;5;9m", "\u{1b}[38;5;11m"),
    ("\u{1b}[48;5;11m", "\u{1b}[38;5;15m"),
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

    fn blue_palette(&self, luma: u8) -> ((&str, &str), char) {
        let collen: u8 = BLUE_PALETTE.len() as u8;
        let charrange: u8 = PALETTE_RANGE / CHARS_GRADIENT.len() as u8;
        let v_col = (luma * collen / PALETTE_RANGE) as usize;
        let v_char = (((luma * collen) % PALETTE_RANGE) / charrange) as usize;
        (
            match v_col {
                0..=3 => BLUE_PALETTE[v_col],
                _ => DEFAULT_COLOR,
            },
            match v_char {
                0..=4 => CHARS_GRADIENT[v_char],
                _ => ' ',
            },
        )
    }

    fn red_palette(&self, luma: u8) -> ((&str, &str), char) {
        let collen: u8 = RED_PALETTE.len() as u8;
        let charrange: u8 = PALETTE_RANGE / CHARS_GRADIENT.len() as u8;
        let v_col = (luma * collen / PALETTE_RANGE) as usize;
        let v_char = (((luma * collen) % PALETTE_RANGE) / charrange) as usize;
        (
            match v_col {
                0..=4 => RED_PALETTE[v_col],
                _ => DEFAULT_COLOR,
            },
            match v_char {
                0..=4 => CHARS_GRADIENT[v_char],
                _ => ' ',
            },
        )
    }

    fn gray_palette(&self, luma: u8) -> ((&str, &str), char) {
        let collen: u8 = GRAY_PALETTE.len() as u8;
        let charrange: u8 = PALETTE_RANGE / CHARS_GRADIENT.len() as u8;
        let v_col = (luma * collen / PALETTE_RANGE) as usize;
        let v_char = (((luma * collen) % PALETTE_RANGE) / charrange) as usize;
        (
            match v_col {
                0..=2 => GRAY_PALETTE[v_col],
                _ => DEFAULT_COLOR,
            },
            match v_char {
                0..=4 => CHARS_GRADIENT[v_char],
                _ => ' ',
            },
        )
    }

    fn fill_color(&self, color: u8) {
        let ((fg, bg), chr) = match color {
            0..=31 => self.blue_palette(color),
            32..=63 => self.red_palette(color - 32),
            64..=95 => self.gray_palette(color - 64),
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
