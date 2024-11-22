use rand::Rng;
use crate::convolutions::{Color, Convolution};

pub struct TimeConvolution {
    pub f24: bool, // Si se usa formato de 24 horas o no
    pub color: Option<u32>
}

impl TimeConvolution {
    fn random_color(&self) -> u32 {
        let mut rng = rand::thread_rng();

        Color::hsv(rng.gen_range(0.0..=360.), 0.8, 0.6)
    }
}

impl Convolution for TimeConvolution {
    fn name(&self) -> &str {
        "Time"
    }

    fn transform(&mut self, pixels: &mut [u32], width: usize, height: usize) {
        let time = chrono::Local::now();

        let time_string = if self.f24 {
            time.format("%H:%M:%S").to_string() // 24 horas
        } else {
            time.format("%I:%M:%S%p").to_string() // 12 horas con AM/PM
        };

        let mut new_buffer = vec![0; width * height];
        let color: u32 = match &self.color {
            Some(c) => *c,
            None => self.random_color(),
        };

        let font_render = minifb_fonts::font5x8::new_renderer(
            width,
            height,
            color,
        );

        font_render.draw_text(&mut new_buffer, (width - time_string.len() * 5 + 7) / 2, (height - 7) / 2, time_string.as_str());

        for i in 0..pixels.len() {
            if new_buffer[i] != 0 {
                pixels[i] = new_buffer[i];
            }
        }
    }
}