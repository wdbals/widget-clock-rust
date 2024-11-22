use crate::convolutions::{Color, Convolution};

pub struct ToGreenConvolution;

impl Convolution for ToGreenConvolution {
    fn name(&self) -> &str {
        "ToGreen"
    }

    fn transform(&mut self, pixels: &mut [u32], _width: usize, _height: usize) {
        for i in 0..pixels.len() {
            pixels[i] = Color::rgb(0, 0xFF, 0);
        }
    }
}

pub struct SingleColorConvolution {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Convolution for SingleColorConvolution {
    fn name(&self) -> &str {
        "ToColor"
    }

    fn transform(&mut self, pixels: &mut [u32], _width: usize, _height: usize) {
        for i in 0..pixels.len() {
            pixels[i] = Color::rgb(self.red, self.green, self.blue)
        }
    }
}