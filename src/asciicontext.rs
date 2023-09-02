use crate::drawables::Triangle;
use crate::drawingcontext::DrawingContext;
use termion::color;

pub struct AsciiContext {
    bitmap: Vec<u8>,
    size: (u16, u16),
    last_color: u8,
    last_char: char,
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

        AsciiContext {
            bitmap,
            size,
            last_color: 0,
            last_char: ' ',
        }
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

    fn fill_color(&mut self, color: u8) {
        if self.last_color == color {
            print!("{}", self.last_char);
            return;
        }

        self.last_color = color;
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

    fn draw_triangles(&mut self, triangles: &Vec<Triangle>) {
        //find the minimum on Y axis and it's index
        let min_y: Vec<(u16, usize)> = triangles
            .iter()
            .map(|tri| {
                let min = tri.points[0].1.min(tri.points[1].1).min(tri.points[2].1);
                let mut i: usize = 0;
                if tri.points[1].1 == min {
                    i = 1;
                }
                if tri.points[2].1 == min {
                    i = 2;
                }

                (min as u16, i)
            })
            .collect();
        //find the maximum on Y axis and it's index
        let max_y: Vec<(u16, usize)> = triangles
            .iter()
            .map(|tri| {
                let max = tri.points[0].1.max(tri.points[1].1).max(tri.points[2].1);
                let mut i: usize = 0;
                if tri.points[1].1 == max {
                    i = 1;
                }
                if tri.points[2].1 == max {
                    i = 2;
                }

                (max as u16, i)
            })
            .collect();

        //first element is number of lines
        //second is the index of minimum
        //third is the index of maximum
        //forth is the index of the middle on Y axis
        //the forth is extracted by doing a trick
        //3 - min_index - max_index is always going to be the remaining vertice
        type Data = (u16, usize, usize, usize);
        let data: Vec<Data> = min_y
            .iter()
            .zip(max_y)
            .map(|(min, max)| (max.0 - min.0, min.1, max.1, 3 - min.1 - max.1))
            .collect();

        let number_of_lines = data.iter().fold(0, |acc, el| acc + el.0);

        //x, y, length
        type Line = (u16, u16, u16);
        let mut lines: Vec<Line> = vec![(0, 0, 0); number_of_lines as usize];
        let mut lines_it: usize = 0;

        triangles.iter().zip(data).for_each(|(tri, data)| {
            let indices = [data.1, data.3, data.2];
            for y in 0..data.0 {
                lines[lines_it + y as usize] = (
                    tri.points[0].0 as u16,
                    tri.points[1].0 as u16,
                    tri.points[0].1 as u16 + y,
                );
            }
            lines_it += data.0 as usize;
        });

        lines.iter().for_each(|line| {
            for x in 0..line.2 {
                self.set((line.0 + x, line.1), x as u8);
            }
        });
    }

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
