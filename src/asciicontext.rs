use crate::drawables::*;
use crate::drawingcontext::*;
use termion::color;

pub struct AsciiContext {
    bitmap: Vec<u8>,
    size: (u16, u16),
    triangles: Vec<Triangle>,
    points: Vec<Point>,
    text_entries: Vec<TextEntry>,
}

pub fn vertex_shader(input: &Vec2, camera: &Camera) -> Vec2 {
    let _aspect_ratio: (f32, f32) = (1.0, 1.0); //camera.size.1 / camera.size.0, 1.0);
    let char_ratio: (f32, f32) = (17.0 / 8.0, 1.0);
    let default_height: f32 = camera.size.1 as f32 / 63.0;
    (
        input.0 * default_height * camera.zoom * char_ratio.0 + camera.size.0 / 2.0,
        input.1 * default_height * camera.zoom * char_ratio.1 + camera.size.1 / 2.0,
    )
}

impl AsciiContext {
    pub fn new(size: (u16, u16)) -> AsciiContext {
        let bitmap: Vec<u8> = vec![0; (size.0 * size.1) as usize];

        AsciiContext {
            bitmap,
            size,
            triangles: Vec::with_capacity(100),
            points: Vec::with_capacity(100),
            text_entries: Vec::with_capacity(10),
        }
    }

    pub fn set(&mut self, pos: (u16, u16), v: u8) {
        let i = pos.1 * self.size.0 + pos.0;
        if pos.0 < self.size.0 && pos.1 < self.size.1 && i < self.size.0 * self.size.1 {
            self.bitmap[i as usize] = v;
        }
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

    pub fn draw_triangles(&mut self, camera: &Camera) {
        let shaded_triangles: Vec<Triangle> = self
            .triangles
            .iter()
            .map(|tri| {
                //vertex shader
                Triangle {
                    points: [
                        vertex_shader(&tri.points[0], camera),
                        vertex_shader(&tri.points[1], camera),
                        vertex_shader(&tri.points[2], camera),
                    ],
                    colors: tri.colors,
                    color_palette: tri.color_palette.clone(),
                }
            })
            .collect();

        shaded_triangles.iter().for_each(|tri| {
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
            .map(|(a, b)| ((a.0 as u16, a.1 as u16), (b.0 as u16, b.1 as u16)))
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
                    for y in a_y..b_y {
                        let segment = line_segments[y - t];
                        let computed_x = (a.0 as f32
                            + ((y as f32) - a.1 as f32) * (b.0 as f32 - a.0 as f32)
                                / (b.1 as f32 - a.1 as f32))
                            as u16;
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
                    let color = ((get_barycentric((x as f32, *y as f32), &tri)
                        * PALETTE_RANGE as f32) as u8)
                        .min(PALETTE_RANGE - 1);
                    let color_offset = match tri.color_palette {
                        ColorPalette::Red => 1,
                        ColorPalette::Green => 17,
                        ColorPalette::Blue => 33,
                        ColorPalette::Yellow => 49,
                        ColorPalette::Magenta => 65,
                        ColorPalette::Cyan => 81,
                        ColorPalette::Gray => 97,
                        ColorPalette::Custom => 0,
                    };
                    self.set((x, *y), color + color_offset);
                }
            });
        });
    }

    pub fn draw_points(&mut self, camera: &Camera) {
        let shaded_points: Vec<Point> = self
            .points
            .iter()
            .map(|p| Point {
                position: vertex_shader(&p.position, camera),
                color: p.color,
                color_palette: p.color_palette.clone(),
            })
            .collect();
        shaded_points.iter().for_each(|p| {
            let color = match p.color_palette {
                ColorPalette::Custom => p.color as u8,
                _ => (p.color * PALETTE_RANGE as f32) as u8,
            };
            let color_offset = match p.color_palette {
                ColorPalette::Red => 1,
                ColorPalette::Green => 17,
                ColorPalette::Blue => 33,
                ColorPalette::Yellow => 49,
                ColorPalette::Magenta => 65,
                ColorPalette::Cyan => 81,
                ColorPalette::Gray => 97,
                ColorPalette::Custom => 0,
            };
            self.set(
                (p.position.0 as u16, p.position.1 as u16),
                color + color_offset,
            );
        });
    }
}

fn edge_function(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (c.0 - a.0) * (b.1 - a.1) - (c.1 - a.1) * (b.0 - a.0)
}

fn get_barycentric(point: Vec2, triangle: &Triangle) -> ColorLuma {
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

    fn clear(&mut self) {
        self.bitmap.clear();
        self.bitmap.resize((self.size.0 * self.size.1) as usize, 0);
    }

    fn flush_text_entries(&mut self) {
        self.text_entries.resize(0, TextEntry::empty_text_entry());
    }

    fn flush_triangles(&mut self) {
        self.triangles.resize(0, EMPTY_TRIANGLE);
    }

    fn flush_points(&mut self) {
        self.points.resize(0, EMPTY_POINT);
    }

    fn add_text_entry(&mut self, text_entry: &TextEntry) {
        self.text_entries.push(text_entry.clone());
    }

    fn add_triangles(&mut self, triangle: &Vec<Triangle>) {
        triangle
            .iter()
            .for_each(|tri| self.triangles.push(tri.clone()));
    }

    fn add_point(&mut self, point: &Point) {
        self.points.push(point.clone());
    }

    fn add_points(&mut self, points: &Vec<Point>) {
        points.iter().for_each(|p| self.points.push(p.clone()));
    }

    fn display(&self) {
        print!("{}", termion::cursor::Goto(1, 1));
        for (i, line) in self.bitmap.chunks(self.size.0 as usize).enumerate() {
            if i != 0 {
                print!("\n");

                self.text_entries.iter().for_each(|text| {
                    if i - 1 == text.position.1 as usize {
                        let posx = text.position.0 as u16 + 1;
                        let posy = text.position.1 as u16 + 1;
                        print!("{}", termion::cursor::Goto(posx, posy));
                        match text.color_palette {
                            TextColorPalette::Text => {
                                print!("{}{}", color::Bg(color::Black), color::Fg(color::White))
                            }
                            TextColorPalette::Menu => {
                                print!("{}{}", color::Bg(color::Blue), color::Fg(color::LightCyan))
                            }
                            TextColorPalette::Warning => {
                                print!("{}{}", color::Bg(color::LightRed), color::Fg(color::Black))
                            }
                        };
                        print!("{}", text.string);
                    }
                });

                print!("{}", termion::cursor::Goto(1, (i + 1) as u16));
                print!("{}{}", color::Bg(color::Black), color::Fg(color::White));
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
                    128 => {
                        print!("{}", color::Fg(color::LightWhite));
                        print!("{}", CHAR_BALL);
                        last_pixel = pixel;
                        was_colored = true;
                    }
                    129 => {
                        print!("{}{}", color::Bg(color::LightRed), color::Fg(color::Black));
                        print!("{}", CHAR_PIERCING0);
                        last_pixel = pixel;
                        was_colored = true;
                    }
                    130 => {
                        print!("{}{}", color::Bg(color::LightRed), color::Fg(color::Black));
                        print!("{}", CHAR_PIERCING1);
                        last_pixel = pixel;
                        was_colored = true;
                    }
                    _ => {
                        last_char = self.fill_color(pixel - 1, last_pixel - 1, last_char);
                        last_pixel = pixel;
                        was_colored = true;
                    }
                }
            }

            if was_colored {
                print!("{}{}", color::Bg(color::Black), color::Fg(color::White));
            }

            print!("\r");
        }
    }
}
