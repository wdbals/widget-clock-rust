use rand::Rng;

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
    colors: Vec<Color>,
}

impl Palette {
    fn get(&self, index: usize) -> Option<&Color> {
        self.colors.get(index)
    }

    fn add_color(&mut self, color: Color) {
        self.colors.push(color);
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
    Normal(Box<dyn Convolution>),
    Advanced(Box<dyn ConvolutionAdvanced>),
}

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

pub struct FireConvolution {
    pub intensity: f32,  // Un parámetro que controla la "intensidad" del fuego.
}

impl Convolution for FireConvolution {
    fn name(&self) -> &str {
        "Fire"
    }

    fn transform(&mut self, pixels: &mut [u32], _width: usize, _height: usize) {
        // Usamos un generador de números aleatorios para crear variabilidad en el color del fuego
        let mut rng = rand::thread_rng();

        // Iterar sobre todos los píxeles y aplicar el efecto de fuego
        for pixel in pixels.iter_mut() {
            let flame_intensity: f32 = rng.gen_range(0.0..self.intensity); // Aleatorio en el rango [0.0, intensity)

            // Los colores cálidos para simular fuego (rojo, naranja, amarillo)
            let red = (flame_intensity * 255.0) as u8;
            let green = (flame_intensity * 180.0) as u8;
            let blue = (flame_intensity * 100.0) as u8;

            // Aplicar el color "fuego" al píxel (rojo, naranja, amarillo)
            *pixel = Color::rgb(red, green, blue);
        }
    }
}

pub struct HeatFireConvolution {
    base_intensity: f32,  // La intensidad base del fuego en la parte inferior
    falloff: f32,         // La "caída" o disminución de la intensidad a medida que subimos
    heat_buffer: Vec<f32>, // Buffer de calor para acumular la intensidad del fuego
}

impl HeatFireConvolution {
    pub fn new(base_intensity: f32, falloff: f32, width: usize, height: usize) -> Self {
        HeatFireConvolution {
            base_intensity,
            falloff,
            heat_buffer: vec![0.0; width * height], // Inicializa el buffer de calor
        }
    }

    // function to update the heat in the heat_buffer
    fn update_heat_buffer(&mut self, width: usize, height: usize) {
        let mut rng = rand::thread_rng(); // Generador de números aleatorios

        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;

                // Genera un valor aleatorio de fuego en la base (color rojo), ajusta según la posición (altura)
                let intensity = self.base_intensity * (1.0 - self.falloff * (y as f32 / height as f32));
                let flame_intensity = rng.gen_range(0.0..intensity); // Aleatorio en el rango [0.0, intensity)

                // Acumulamos el calor en el heat_buffer
                self.heat_buffer[index] += flame_intensity; // Incrementa la intensidad de calor
            }
        }
    }

    // Function to mix heat with color
    fn apply_heat_to_pixels(&self, pixels: &mut [u32], width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let heat = self.heat_buffer[index];

                // Ajuste del color del fuego dependiendo de la intensidad de calor
                let red = (heat * 255.0) as u8;
                let green = (heat * 150.0) as u8;
                let blue = (heat * 50.0) as u8;

                // Aplicamos el color calculado a cada píxel
                pixels[index] = Color::rgb(red, green, blue);
            }
        }
    }
}

impl Convolution for HeatFireConvolution {
    fn name(&self) -> &str {
        "HeatFire"
    }

    fn transform(&mut self, pixels: &mut [u32], width: usize, height: usize) {
        // Primero actualizamos el buffer de calor
        self.update_heat_buffer(width, height);

        // Luego combinamos el calor con el buffer de píxeles
        self.apply_heat_to_pixels(pixels, width, height);
    }
}

impl ConvolutionAdvanced for HeatFireConvolution {
    fn reset(&mut self) {
        for pixel in self.heat_buffer.iter_mut() {
            *pixel = 0f32;
        }
    }
}

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