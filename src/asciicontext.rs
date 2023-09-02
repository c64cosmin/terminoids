use crate::drawables::*;
use crate::drawingcontext::*;
use termion::color;

pub struct AsciiContext {
    bitmap: Vec<u8>,
    size: (u16, u16),
    triangles: Vec<Triangle>,
}

impl AsciiContext {
    pub fn new(size: (u16, u16)) -> AsciiContext {
        let bitmap: Vec<u8> = vec![0; (size.0 * size.1) as usize];

        AsciiContext {
            bitmap,
            size,
            triangles: Vec::with_capacity(100),
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

    fn fill_color(&self, color: u8, last_color: u8, last_char: char) -> char {
        if last_color == color {
            print!("{}", last_char);
            return last_char;
        }

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

        return chr;
    }

    pub fn draw_triangles(&mut self) {
        self.triangles.iter().for_each(|tri| {
            //find the extremities of the triangle
            let top = tri.points[0].1.min(tri.points[1].1).min(tri.points[2].1);
            let bot = tri.points[0].1.max(tri.points[1].1).max(tri.points[2].1);

            let vector_len = (bot - top) as usize + 1;
            //create the vector of lines
            let mut line_segments: Vec<(u16, u16, u16)> =
                vec![(u16::MAX, u16::MAX, u16::MAX); vector_len];

            //all edges
            [
                (tri.points[0], tri.points[1]),
                (tri.points[1], tri.points[2]),
                (tri.points[2], tri.points[0]),
            ]
            .iter()
            .map(|(a, b)| {
                //first point is the top one
                if b.1 < a.1 {
                    return (b, a);
                }
                return (a, b);
            })
            .for_each(|(a, b)| {
                let a_y = a.1 as usize;
                let b_y = b.1 as usize;
                let t = top as usize;
                if a_y != b_y {
                    for y in a_y..=b_y {
                        let segment = line_segments[y - t];
                        let computed_x =
                            (a.0 + ((y as f32) - a.1) * (b.0 - a.0) / (b.1 - a.1)) as u16;
                        if segment.0 == u16::MAX {
                            line_segments[y - t] = (computed_x, u16::MAX, y as u16);
                        } else {
                            line_segments[y - t] = (
                                computed_x.min(segment.0),
                                computed_x.max(segment.0),
                                y as u16,
                            );
                        }
                    }
                }
            });

            line_segments.iter().for_each(|(x0, x1, y)| {
                for x in *x0..=*x1 {
                    let color =
                        (get_barycentric((x as f32, *y as f32), tri) * PALETTE_RANGE as f32) as u8;
                    self.set((x, *y), color);
                }
            });
        });
    }
}

fn edge_function(a: Point, b: Point, c: Point) -> f32 {
    (c.0 - a.0) * (b.1 - a.1) - (c.1 - a.1) * (b.0 - a.0)
}

fn get_barycentric(point: Point, triangle: &Triangle) -> ColorLuma {
    let area: f32 = edge_function(triangle.points[0], triangle.points[1], triangle.points[2]);
    let w0: f32 = edge_function(triangle.points[1], triangle.points[2], point) / area;
    let w1: f32 = edge_function(triangle.points[2], triangle.points[0], point) / area;
    let w2: f32 = edge_function(triangle.points[0], triangle.points[1], point) / area;

    triangle.colors[0] * w0 + triangle.colors[1] * w1 + triangle.colors[2] * w2
}

impl DrawingContext for AsciiContext {
    fn resize(&mut self, size: (u16, u16)) {
        self.bitmap = vec![0; (size.0 * size.1) as usize];
    }

    fn flush_triangles(&mut self) {
        self.triangles.resize(0, EMPTY_TRIANGLE);
    }

    fn add_triangles(&mut self, triangle: &Vec<Triangle>) {
        triangle.iter().for_each(|tri| self.triangles.push(*tri));
    }

    fn display(&self) {
        for (i, line) in self.bitmap.chunks(self.size.0 as usize).enumerate() {
            if i != 0 {
                print!("\n");
            }

            let mut was_colored = false;
            let mut last_pixel: u8 = 0;
            let mut last_char: char = '!';

            for &pixel in line.iter() {
                match pixel {
                    0 => {
                        if was_colored {
                            print!("{}{}", color::Bg(color::Black), color::Fg(color::White));
                            was_colored = false;
                            last_pixel = 0;
                        }
                        print!(" ");
                    }
                    _ => {
                        last_char = self.fill_color(pixel - 1, last_pixel - 1, last_char);
                        last_pixel = pixel;
                        was_colored = true;
                    }
                }
            }

            print!("\r");
        }
    }
}
