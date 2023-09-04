use crate::drawables::*;
use rand::Rng;

pub struct FireDrawer {
    bitmap: Vec<f32>,
    size: (usize, usize),
    terminal_size: (u16, u16),
    wind: i16,
    time: f32,
}

impl FireDrawer {
    pub fn new(size: (u16, u16)) -> FireDrawer {
        let plasma_size = (size.0 as usize, 30);
        FireDrawer {
            bitmap: vec![0.0; plasma_size.0 * plasma_size.1],
            size: plasma_size,
            terminal_size: size,
            wind: 0,
            time: 3.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        let mut rnd = rand::thread_rng();

        self.time -= delta;
        if self.time < 0.0 {
            self.time = 3.0;
            self.wind = rnd.gen_range(0..=2) - 1;
        }

        for x in 0..self.size.0 {
            self.set_value(x, self.size.1 - 1, rnd.gen::<f32>() * 0.2 + 0.8);
        }

        for y in (0..self.size.1 - 1).rev() {
            for x in 0..self.size.0 {
                let offset_x = rnd.gen_range(0..=2) - 1 + self.wind;
                let offset_y = rnd.gen_range(0..=1);
                let new_x = (x as i16 + offset_x).clamp(0, self.size.0 as i16 - 1);
                let new_y = (y as i16 + offset_y).clamp(0, self.size.1 as i16 - 1);
                let pixel = self.get_value(new_x as usize, new_y as usize)
                    * (1.0 - rnd.gen::<f32>() * 0.2).clamp(0.0, 1.0);
                self.set_value(x, y, pixel);
            }
        }
    }

    fn get_value(&mut self, x: usize, y: usize) -> f32 {
        self.bitmap[x + y * self.size.0 as usize]
    }

    fn set_value(&mut self, x: usize, y: usize, value: f32) {
        self.bitmap[x + y * self.size.0 as usize] = value;
    }

    pub fn draw(&self) {
        print!(
            "{}",
            termion::cursor::Goto(1, self.terminal_size.1 - self.size.1 as u16 + 1)
        );
        self.bitmap
            .chunks(self.size.0 as usize)
            .for_each(|line| line.iter().for_each(|&pixel| self.fill_color(pixel)));
    }

    fn fill_color(&self, color: f32) {
        let ((fg, bg), chr) = self.fire_palette((color * 100.0) as u16);

        print!("{}{}{}", bg, fg, chr);
    }

    fn fire_palette(&self, luma: u16) -> ((&str, &str), char) {
        let (v_col, v_char) = self.get_indexes(luma, FIRE_PALETTE.len());
        (FIRE_PALETTE[v_col], CHARS_GRADIENT[v_char])
    }

    fn get_indexes(&self, luma: u16, color_len: usize) -> (usize, usize) {
        let palette_range = 100;
        let char_len: u16 = CHARS_GRADIENT.len() as u16;
        let color_len: u16 = color_len as u16;
        let v_col = luma * color_len / palette_range;
        let v_char = ((luma * color_len) % palette_range) * char_len / palette_range;

        (v_col as usize, v_char as usize)
    }
}
