pub mod base;
pub mod fire;
pub mod time;

/// Color representation for MiniFB,
pub struct Color;

impl Color {
    /// Returns u32 representation of the RGB value
    pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
        let (r, g, b) = (red as u32, green as u32, blue as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn hsv(hue: f64, saturation: f64, value: f64) -> u32 {
        let (red, green, blue) = hsv::hsv_to_rgb(hue, saturation, value);
        Self::rgb(red, green, blue)
    }
}

/// Palette's representation for a
/// pixel value
struct Palette {
    colors: Vec<u32>,
}

impl Palette {
    pub fn new() -> Palette {
        Palette { colors: vec![] }
    }

    fn get(&self, index: usize) -> Option<&u32> {
        self.colors.get(index)
    }

    fn add_color(&mut self, color: u32) {
        self.colors.push(color);
    }

    pub fn add_colors(&mut self, colors: Vec<u32>) {
        self.colors.extend(colors); // Suponiendo que `self.colors` es un Vec<u32>
    }
}

/// Trait to get a convoluted buffer
pub trait Convolution {
    /// Function to get Convolution name
    fn name(&self) -> &str;
    /// Function to implement the transformation of the main buffer
    /// to be displayed
    fn transform(&mut self, pixels: &mut [u32], width: usize, height: usize);
}

pub trait ConvolutionAdvanced: Convolution {
    /// Function to reset the own Convolution buffer
    fn reset(&mut self);
}

pub enum ConvolutionType {
    Simple(Box<dyn Convolution>),
    Advanced(Box<dyn ConvolutionAdvanced>),
}