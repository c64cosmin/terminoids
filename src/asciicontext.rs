use crate::drawables::Triangle;
use crate::drawingcontext::DrawingContext;
use termion::color;

pub struct AsciiContext {
    bitmap: Vec<u8>,
    size: (u16, u16),
}

impl AsciiContext {
    pub fn new(size: (u16, u16)) -> AsciiContext {
        let bitmap: Vec<u8> = vec![0; (size.0 * size.1) as usize];

        AsciiContext { bitmap, size }
    }

    pub fn set(&mut self, pos: (u16, u16), v: u8) {
        let i = pos.1 * self.size.0 + pos.0;
        self.bitmap[i as usize] = v;
    }

    fn fill_color(&self, color: u8) {
        let colors = 8;
        let chars = 5;

        let (fg, bg) = match (color % (colors * chars)) / chars {
            0 => (color::Black.bg_str(), color::Blue.fg_str()),
            1 => (color::Blue.bg_str(), color::LightBlue.fg_str()),
            2 => (color::LightBlue.bg_str(), color::LightCyan.fg_str()),
            3 => (color::LightCyan.bg_str(), color::LightWhite.fg_str()),
            4 => (color::LightWhite.bg_str(), color::LightCyan.fg_str()),
            5 => (color::LightCyan.bg_str(), color::LightBlue.fg_str()),
            6 => (color::LightBlue.bg_str(), color::Blue.fg_str()),
            7 => (color::Blue.bg_str(), color::Black.fg_str()),
            _ => (color::Black.bg_str(), color::White.fg_str()),
        };

        let chr = match (color % (colors * chars)) % chars {
            /*
            0 => '.',
            1 => 'x',
            2 => '%',
            3 => '#',
            4 => '@',
            */
            0 => ' ',
            1 => '\u{2591}',
            2 => '\u{2592}',
            3 => '\u{2593}',
            4 => '\u{2593}',
            _ => 0 as char,
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
