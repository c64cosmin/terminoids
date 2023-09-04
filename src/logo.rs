use crate::asciicontext::AsciiContext;
use crate::drawables::*;
use crate::drawingcontext::DrawingContext;
use crate::sprite::*;
use crate::terminaldrawable::TerminalDrawble;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn load_logo() -> Vec<Triangle> {
    let mut vertices: Vec<Vec2> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    if let Some(binary_dir) = current_binary_directory() {
        let file = match File::open(binary_dir.join("logo.obj")) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                return [].to_vec();
            }
        };

        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    continue;
                }
            };

            match line.chars().next() {
                Some(c) => match c {
                    'v' => {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        let x = parts[1].parse::<f32>().unwrap();
                        let y = parts[3].parse::<f32>().unwrap();
                        vertices.push((x, y));
                    }
                    'f' => {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        let a = (parts[1].parse::<i16>().unwrap() - 1) as usize;
                        let b = (parts[2].parse::<i16>().unwrap() - 1) as usize;
                        let c = (parts[3].parse::<i16>().unwrap() - 1) as usize;
                        triangles.push(Triangle {
                            points: [vertices[a], vertices[b], vertices[c]],
                            colors: [0.0, 0.0, 0.0],
                            color_palette: ColorPalette::Custom,
                        });
                    }
                    _ => {}
                },
                None => {}
            }
        }
    }

    triangles
}

fn current_binary_directory() -> Option<PathBuf> {
    // Get the path to the currently running binary
    if let Ok(current_exe) = env::current_exe() {
        // Extract the directory of the binary
        if let Some(binary_dir) = current_exe.parent() {
            return Some(binary_dir.to_path_buf());
        }
    }
    None
}

pub struct DrawbleLogo {
    pub color_palette: ColorPalette,
    logo: Vec<Triangle>,
    time: f32,
}

impl DrawbleLogo {
    pub fn new() -> DrawbleLogo {
        DrawbleLogo {
            color_palette: ColorPalette::Green,
            logo: load_logo(),
            time: 0.0,
        }
    }
}

impl Sprite for DrawbleLogo {
    fn update(&mut self, _camera: &Camera, delta: f32) {
        self.time += delta;
    }

    fn is_alive(&self) -> bool {
        true
    }
}

impl TerminalDrawble for DrawbleLogo {
    fn draw(&self, ctx: &mut AsciiContext) {
        let shader_scale = (25.0 + (0.3 * self.time).cos() * 5.0, 25.0);
        let shader_offset = (
            0.0 + (0.4 * self.time).cos(),
            -5.0 + (0.3 * self.time).sin(),
        );
        let mut logo_shaded = vec![EMPTY_TRIANGLE; self.logo.len()];
        self.logo.iter().enumerate().for_each(|(i, triangle)| {
            logo_shaded[i].color_palette = self.color_palette;
            for j in 0..3 {
                let point = (
                    triangle.points[j].0 * shader_scale.0 + shader_offset.0,
                    triangle.points[j].1 * shader_scale.1 + shader_offset.1,
                );
                logo_shaded[i].points[j] = point;
                logo_shaded[i].colors[j] =
                    (point.0 * 0.1 + point.1 + 0.1 + self.time * 0.5).cos() * 0.4 + 0.6;
            }
        });
        ctx.add_triangles(&logo_shaded);
    }
}
