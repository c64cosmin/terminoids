use crate::drawables::*;
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
